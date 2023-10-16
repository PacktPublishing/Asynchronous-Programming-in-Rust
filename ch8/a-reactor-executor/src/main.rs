use mio::{Events, Poll, Registry, net::TcpStream, Token, Interest};
use std::{io::{self, Result}, io::Read, io::Write, thread, time::Duration};
use std::sync::mpsc::{channel, Receiver, Sender};

const TEST_TOKEN: usize = 10;

fn main() -> Result<()> {
    let (evt_sender, evt_receiver) = channel();
    let reactor = Reactor::new(evt_sender);
    let mut executor = Excutor::new(evt_receiver);
    

    let stream = std::net::TcpStream::connect("flash.siwalik.in:80")?;
    stream.set_nonblocking(true);
    let mut stream = TcpStream::from_std(stream);
    
    let request = b"GET /delay/1000/url/http://www.google.com HTTP/1.1\r\nHost: flash.siwalik.in\r\nConnection: close\r\n\r\n";
    stream.write_all(request).expect("Stream write err.");
    reactor.register_stream_read_interest(&mut stream, Token(TEST_TOKEN));

    executor.suspend(TEST_TOKEN, move || {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        println!("{}", buffer);
        assert!(!buffer.is_empty(), "Got an empty buffer");
        //reactor.stop_loop();
    });

    executor.block_on_all();
    // NB! Best practice is to make sure to join our child thread. We skip it here for brevity.
    println!("EXITING");
    Ok(())
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
                    Ok(..) => (),
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("INTERRUPTED: {}", e);
                        break;
                    }
                    Err(e) => panic!("Poll error: {:?}, {}", e.kind(), e),
                };
                for event in &events {
                    let event_token = event.token();
                    evt_sender.send(event_token).expect("send event_token err.");
                }
            }
        });

        Reactor { handle, registrator }
    }

    fn register_stream_read_interest(&self, stream: &mut TcpStream, token: Token) {
        self.registrator.register(stream, token, Interest::READABLE).expect("registration err.");
    }

    // fn stop_loop(&self) {
    //     self.registrator.close_loop().expect("close loop err.");
    // }
}

struct Excutor {
    events: Vec<(usize, Box<dyn FnMut()>)>,
    evt_receiver: Receiver<Token>,
}

impl Excutor {
    fn new(evt_receiver: Receiver<Token>) -> Self {
        Excutor { events: vec![], evt_receiver }
    }
    fn suspend(&mut self, id: usize, f: impl FnMut() + 'static) {
        self.events.push((id, Box::new(f)));
    }
    fn resume(&mut self, event: usize) {
        println!("RESUMING TASK: {}", event);
        let (_, f) = self.events
            .iter_mut()
            .find(|(e, _)| *e == event)
            .expect("Couldn't find event.");
        f();
    }
    fn block_on_all(&mut self) {
        while let Ok(received_token) = self.evt_receiver.recv() {
            assert_eq!(Token(TEST_TOKEN), received_token, "Non matching tokens.");
            let event_id: usize = received_token.into();
            println!("EVENT: {} is ready", event_id);
            self.resume(event_id);
        }
    }
}