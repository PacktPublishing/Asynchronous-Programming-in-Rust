use std::{io::{Write, Read, self}, collections::HashMap, sync::{Mutex, Arc}};


fn main() {
    let p = Promise::Pending;
    p.then(|| {
        // let stream = std::net::TcpStream::connect("localhost:8080").unwrap();
        // stream.set_nonblocking(true).unwrap();
        // let mut stream = mio::net::TcpStream::from_std(stream);
        // stream.write_all(get_req("/1000/helloworld").as_bytes()).unwrap();
        // let mut s = String::new();
        // match stream.read_to_string(&mut s) {
        //     Ok(_) => {
        //         println!("DATA: {s}");
        //         Promise::Fulfilled
        //     }
            
        //     Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                
        //     },
        //     Err(e) => panic!("{e:?}"),
        // }
    });  
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

struct Executor {
    tasks: Arc<Mutex<HashMap<usize, Promise>>>,
}

impl Executor {
    pub fn spawn(&self, p: Promise) {
        self.tasks.lock().map(|mut tasks| tasks.insert(1, p)).unwrap();
    }
    
    pub fn run(&self) {
        
    }
}

struct StreamTask {
    id: usize,
    promise: StreamPromise,
}

enum StreamPromise {
    Fulfilled,
    Pending(usize),
}

impl StreamPromise {
    fn then(self, cb: impl FnOnce(mio::net::TcpStream) -> Self) -> Self {
        match cb() {
            Self::Fulfilled => Self::Fulfilled,
            Self::Pending(id) => {
                
            }
        }
    }
}

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
