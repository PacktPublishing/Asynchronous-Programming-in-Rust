use mio::{net::TcpStream, Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    cell::RefCell,
    io::Read,
    io::Write,
    io::{self, Result},
    thread,
    time::Duration,
};

const TEST_TOKEN: usize = 10;

thread_local! {
    static RT: Runtime = Runtime::new();
    static ID: RefCell<usize> = RefCell::new(0);
}

fn async_main() {
    let stream = runtime::TcpStream::connect("localhost:8080").unwrap();
    let request = b"GET /1000/HelloWorld HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream
        .write_all(request, move |stream| {
            println!("Write CB");
            stream
                .read_to_string(|_, text| {
                    println!("{text}");
                })
                .unwrap();
        })
        .unwrap();
}

fn main() -> Result<()> {
    runtime::run(async_main);

    // let (evt_sender, evt_receiver) = channel();
    // let reactor = Reactor::new(evt_sender);
    // let mut executor = Executor::new(evt_receiver);

    // let stream = std::net::TcpStream::connect("localhost:8080")?;
    // stream.set_nonblocking(true)?;
    // let mut stream = TcpStream::from_std(stream);

    // let request = b"GET /1000/HelloWorld HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    // stream.write_all(request).expect("Stream write err.");
    // reactor.register_stream_read_interest(&mut stream, Token(TEST_TOKEN));

    // executor.spawn(TEST_TOKEN, move || {
    //     let mut buffer = String::new();
    //     stream.read_to_string(&mut buffer).unwrap();
    //     println!("{}", buffer);
    //     assert!(!buffer.is_empty(), "Got an empty buffer");
    //     //reactor.stop_loop();
    // });

    // executor.block_on_all();
    // // NB! Best practice is to make sure to join our child thread. We skip it here for brevity.
    // println!("EXITING");
    Ok(())
}

mod runtime {
    use std::{
        borrow::BorrowMut,
        cell::RefCell,
        io::{self, Read, Result, Write},
        net::ToSocketAddrs,
        rc::Rc,
    };

    use mio::{Interest, Token};

    use crate::TEST_TOKEN;

    use super::{ID, RT};
    pub fn spawn(token: usize, f: impl FnOnce() + 'static) {
        RT.with(|rt| rt.executor.borrow_mut().spawn(token, f))
    }

    pub fn run(f: fn() -> ()) {
        f();
        RT.with(|rt| rt.executor.borrow_mut().block_on_all());
    }

    fn next_id() -> usize {
        ID.with(|id| {
            let mut id = id.borrow_mut();
            *id += 1;
            *id
        })
    }

    pub struct TcpStream(mio::net::TcpStream);
    impl TcpStream {
        pub fn connect(addr: impl ToSocketAddrs) -> io::Result<Self> {
            let stream = std::net::TcpStream::connect(addr)?;
            stream.set_nonblocking(true)?;
            let stream = mio::net::TcpStream::from_std(stream);
            Ok(Self(stream))
        }

        pub fn write_all<CB>(mut self, buf: &[u8], mut cb: CB) -> Result<()>
        where
            CB: FnOnce(Self) + 'static,
        {
            // TODO: Rewrite to add to queue and handle WouldBlock
            self.0.write_all(buf)?;
            //spawn(TEST_TOKEN, cb);
            cb(self);
            Ok(())
        }

        pub fn read_to_string<CB>(mut self, cb: CB) -> Result<()>
        where
            CB: FnOnce(Self, String) + 'static,
        {
            let id = next_id();
            RT.with(|rt| {
                let token = Token(id);
                rt.reactor.register(&mut self.0, token, Interest::READABLE);
            });

            spawn(id, move || {
                let mut x = self;
                let mut s = String::new();
                let _ = x.0.read_to_string(&mut s).unwrap();
                cb(x, s);
            });

            Ok(())
        }
    }
}

struct Runtime {
    executor: RefCell<Executor>,
    reactor: Reactor,
}

impl Runtime {
    fn new() -> Self {
        let (evt_sender, evt_receiver) = channel();
        Self {
            reactor: Reactor::new(evt_sender),
            executor: RefCell::new(Executor::new(evt_receiver)),
        }
    }
}

struct Reactor {
    handle: std::thread::JoinHandle<()>,
    registrator: Registry,
}

impl Reactor {
    fn new(evt_sender: Sender<Token>) -> Reactor {
        let mut poll = Poll::new().unwrap();
        let registrator = poll.registry().try_clone().unwrap();

        // Set up the epoll/IOCP event loop in a seperate thread
        let handle = thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            loop {
                println!("Waiting! {:?}", poll);
                let timeout = Duration::from_millis(200);
                match poll.poll(&mut events, Some(timeout)) {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("INTERRUPTED: {}", e);
                        break;
                    }
                    Err(e) => panic!("Poll error: {:?}, {}", e.kind(), e),
                };

                // =======================================================================
                for event in &events {
                    println!("EASHOISJDLAKJDLJADKS");
                    let event_token = event.token();
                    evt_sender.send(event_token).expect("send event_token err.");
                }
            }
        });

        Reactor {
            handle,
            registrator,
        }
    }

    fn register(&self, stream: &mut TcpStream, token: Token, interests: Interest) {
        self.registrator
            .register(stream, token, interests)
            .expect("registration err.");
    }

    // fn stop_loop(&self) {
    //     self.registrator.close_loop().expect("close loop err.");
    // }
}

struct Executor {
    events: HashMap<usize, Box<dyn FnOnce()>>,
    evt_receiver: Receiver<Token>,
}

impl Executor {
    fn new(evt_receiver: Receiver<Token>) -> Self {
        Executor {
            events: HashMap::new(),
            evt_receiver,
        }
    }
    fn spawn(&mut self, id: usize, f: impl FnOnce() + 'static) {
        self.events.insert(id, Box::new(f));
    }
    fn resume(&mut self, event: usize) {
        println!("RESUMING TASK: {}", event);

        let f = self.events.remove(&event).unwrap();
        f();
    }
    fn block_on_all(&mut self) {
        while let Ok(received_token) = self.evt_receiver.recv() {
            let event_id: usize = received_token.into();
            println!("EVENT: {} is ready", event_id);
            self.resume(event_id);
        }
    }
}
