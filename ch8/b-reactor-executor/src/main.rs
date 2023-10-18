use std::{
    collections::HashMap,
    io::{self, ErrorKind, Read, Write},
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    }, thread, time::Duration,
};

later fn async_main() {
    let stream = std::net::TcpStream::connect("localhost:8080").unwrap();
    stream.set_nonblocking(true).unwrap();
    let mut stream = mio::net::TcpStream::from_std(stream);
    let mut txt = String::new();
    manjana stream.read_to_string(&mut txt);
    println!("{txt}");
}

fn main() {
    
    let mut future = NonLeafFuture::new();
    
    let txt = loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }
            
            PollState::Ready(s) => break s,
        }
    };
    
    println!("{txt}");
    println!("Finished");
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

enum LeafFuture {
    Step1(mio::net::TcpStream),
    Resolved,
}

impl Future for LeafFuture {
    type Output = String;
    fn poll(&mut self) -> PollState<Self::Output> {
        match self {
            Self::Step1(ref mut stream) => {
                let mut s = String::new();
                match stream.read_to_string(&mut s) {
                    Ok(_) => {
                        *self = LeafFuture::Resolved;
                        PollState::Ready(s)
                    },
                    Err(e) if e.kind() == ErrorKind::WouldBlock => PollState::NotReady,
                    Err(e) => panic!("e:?"),
                }
            }
            
            Self::Resolved => panic!("Polled a futures that's finished!"),
        }
    }
}

enum NonLeafFuture {
    Step1,
    Step2(Box<dyn Future>),
    Resolved,
}

impl NonLeafFuture {
    fn new() -> Self {
        Self::Step1
    }
}

impl Future for NonLeafFuture {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        match self {
            NonLeafFuture::Step1 => {
                let stream = std::net::TcpStream::connect("localhost:8080").unwrap();
                stream.set_nonblocking(true).unwrap();
                let mut stream = mio::net::TcpStream::from_std(stream);
                stream
                    .write_all(get_req("/1000/helloworld").as_bytes())
                    .unwrap();
                let leaf = LeafFuture::Step1(stream);
                *self = NonLeafFuture::Step2(Box::new(leaf));  
                PollState::NotReady 
            }
            
            NonLeafFuture::Step2(ref mut leaf) => match leaf.poll() {
                PollState::Ready(txt) => {
                    *self = NonLeafFuture::Resolved;
                    PollState::Ready(txt)
                }
                
                PollState::NotReady => PollState::NotReady,
            }
            
            NonLeafFuture::Resolved => panic!("Polled a future that's finished"),
        }
    }
}

trait Future {
    type Output;

    fn poll(&mut self) -> PollState<Self::Output>;
}

enum PollState<T> {
    Ready(T),
    NotReady,
}

// trait Prom {
//     // What promise resolves into
//     type Item;
//     type Data;
//     fn resolve(&mut self) -> PromiseState<Self::Item>;
//     fn set_data(&mut self, data: Self::Data);
// }

// enum PromiseState<T> {
//     Ready(T),
//     NotReady,
// }

// struct LeafPromise {
//     stream: mio::net::TcpStream,
// }

// impl LeafPromise {
//     fn new(stream: mio::net::TcpStream) -> Self {
//         Self { stream }
//     }

//     fn then(self, op: impl FnOnce(String) -> ()) -> impl Prom {
//         let next = NonLeafPromise {

//         }
//     }
// }

// impl Prom for LeafPromise {
//     type Item = String;
//     type Data = ();

//     fn resolve(&mut self) -> PromiseState<Self::Item> {
//         let mut s = String::new();
//         match self.stream.read_to_string(&mut s) {
//             Ok(_) => PromiseState::Ready(s),
//             Err(e) if e.kind() == ErrorKind::WouldBlock => PromiseState::NotReady,
//             Err(e) => panic!("{e:?}"),
//         }
//     }

//     fn set_data(&mut self, data: Self::Data) {
//     }
// }

// struct NonLeafPromise {
//     data: Option<String>,
//     op: Option<Box<dyn FnOnce(String) -> ()>>
// }

// impl Prom for NonLeafPromise {
//     type Item = ();
//     type Data = String;

//     fn resolve(&mut self) -> PromiseState<Self::Item> {
//         let op = self.op.take().unwrap();
//         op(self.data.take().unwrap());
//         PromiseState::Ready(())

//     }

//     fn set_data(&mut self, data: Self::Data) {
//         self.data = Some(data);
//     }
// }

// struct Chained<A: Prom, B: Prom> {
//     current: A,
//     next: B,
// }

// impl<A: Prom, B: Prom> Prom for Chained<A, B> {
//     type Item = B::Item;

//     fn resolve(&mut self) -> PromiseState<Self::Item> {
//         match self.current.resolve() {
//             PromiseState::Ready(txt) => {
//                 self.next.set_data(txt);
//                 self.next.resolve()
//             }

//             PromiseState::NotReady => PromiseState::NotReady,
//         }
//     }
// }

// struct PromiseResolved;

// impl Prom for PromiseResolved {
//     type Item = ();

//     fn resolve(&mut self) -> PromiseState<Self::Item> {
//         PromiseState::Ready(())
//     }
// }

// struct Promise<T> {
//     state: PromiseState,
//     value: ValueKind<T>,
//     handler: Option<Box<dyn FnOnce() -> () + 'static>>,
// }

// impl<T> Promise<T> {
//     fn new<F: FnOnce() -> () + 'static>(cb: F) -> Self {
//         Self {
//             state: PromiseState::Pending,
//             value: ValueKind::None,
//             handler: None,
//         }
//     }

//     fn resolve<U>(&mut self, value: ValueKind<T>) -> Promise<U>  {
//        todo!()
//     }

//     fn then<U, F: FnOnce() -> () + 'static>(cb: F) -> Promise<U> {
//         Promise::new(cb)
//     }
// }

// enum PromiseState {
//     Pending,
//     Fulfilled,
// }

// enum ValueKind<T> {
//     Data(T),
//     Promise(T),
//     None,
// }

// struct Executor {
//     tasks: Arc<Mutex<HashMap<usize, Promise>>>,
// }

// impl Executor {
//     pub fn spawn(&self, p: Promise) {
//         self.tasks
//             .lock()
//             .map(|mut tasks| tasks.insert(1, p))
//             .unwrap();
//     }

//     pub fn run(&self) {}
// }

// trait Promise {
//     type Output;
//     fn then<T>(&mut self, cb: impl FnOnce(Self::Output)) -> Box<dyn Promise<Output = T>>;
// }

// struct StreamTask {
//     ready: Arc<AtomicBool>,
//     stream: Option<mio::net::TcpStream>,
//     id: usize,
//     next: Option<Box<StreamTask>>,
// }

// impl Promise for StreamTask {
//     type Output = String;

//     fn then<T>(&mut self, cb: impl FnOnce(Self::Output)) -> PromiseState<mio::net::TcpStream> {
//         if self.ready.load(Ordering::Acquire) {
//             let stream = self.stream.take().unwrap();
//             let mut s = String::new();
//             stream.read_to_string(&mut s).unwrap();
//             cb(s);
//             PromiseState::Fulfilled(self.stream)
//         } else {
//             PromiseState::Pending
//         }
//     }
// }

// enum PromiseState<T> {
//     Fulfilled(T),
//     Pending,
// }

// use std::collections::HashMap;

// use runtime::{Runtime, TcpStream};

// fn main() {
//     let rt = Runtime::new();

//     rt.run(async_main)
// }

// fn async_main() {
//     let addr = "localhost:8080";
//     TcpStream::connect(addr)
//         .then(|stream| {
//             stream
//                 .write_all("abc".as_bytes())
//                 .then(|stream| {
//                     stream
//                         .read_to_string()
//                         .then(|stream, txt| {
//                             println!("{txt}");
//                         })

//                 })

//             stream.write_all("def".as_bytes()).then(|stream| {
//                 stream
//                     .read_to_string()
//                     .then(|(stream, txt)| {
//                         println!("{txt}");
//                         runtime::Promise::Fulfilled(())
//                     })

//             })
//         });
// }

// mod runtime {
//     use mio::{net, Interest};
//     use once_cell::sync::Lazy;
//     use std::{
//         collections::HashMap,
//         io::{ErrorKind, Read, Write},
//         net::ToSocketAddrs,
//         sync::{Arc, Mutex},
//     };

//     struct Reactor {}

//     impl Reactor {
//         fn new() -> Self {
//             Self {}
//         }
//     }

//     struct Executor {
//         tasks: HashMap<usize, Task>,
//     }

//     struct Task {}

//     pub fn run(f: fn() -> ()) {}

//     pub struct TcpStream(net::TcpStream);

//     impl TcpStream {
//         pub fn connect<F>(addr: impl ToSocketAddrs) -> Promise
//         where
//             F: FnOnce(TcpStream) + 'static,
//             {
//             let stream = std::net::TcpStream::connect(addr).unwrap();
//             let stream = net::TcpStream::from_std(stream);
//             Promise::Fulfilled(DataType::TcpStream(Self(stream)))
//         }

//         pub fn write_all<F>(mut self, data: &[u8]) -> Promise
//             {
//                 self.0.write_all(data).unwrap();
//                 Promise::Fulfilled(DataType::TcpStream(self))
//             }

//         pub fn read_to_string(mut self) -> Promise
//         {
//             let mut s = String::new();
//             match self.0.read_to_string(&mut s) {
//                 Ok(_) => {
//                     Promise::Fulfilled(DataType::ReadResult(self, s))
//                 }
//                 Err(e) if e.kind() == ErrorKind::WouldBlock => Promise::Pending(Box::new(|| {
//                     self.read_to_string()
//                 })),

//                 Err(e) => panic!("e:?"),
//             }
//         }
//     }

//     pub enum DataType {
//         TcpStream(TcpStream),
//         ReadResult(TcpStream, String),
//         Nothing,
//     }

//     pub enum Promise
//     {
//         Fulfilled(DataType),
//         Pending(Box<dyn Fn() -> Self + 'static>),
//     }

//     impl Promise
//     {
//         pub fn then<F>(self, f: F) -> Self
//         where F: FnOnce(DataType) -> Self + 'static {
//             match self {
//                 Promise::Fulfilled(s) => f(s),
//                 Promise::Pending(op) => {
//                     op()
//                 }
//             }
//         }
//     }
// }
