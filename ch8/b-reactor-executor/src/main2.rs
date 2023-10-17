use std::collections::HashMap;

use runtime::{Runtime, TcpStream};

fn main() {
    let rt = Runtime::new();

    rt.run(async_main)
}

fn async_main() {
    let addr = "localhost:8080";
    TcpStream::connect(addr)
        .then(|stream| {
            stream
                .write_all("abc".as_bytes())
                .then(|stream| {
                    stream
                        .read_to_string()
                        .then(|stream, txt| {
                            println!("{txt}");
                        })
                        
                })
                

            stream.write_all("def".as_bytes()).then(|stream| {
                stream
                    .read_to_string()
                    .then(|(stream, txt)| {
                        println!("{txt}");
                        runtime::Promise::Fulfilled(())
                    })
                    
            })
        });
}

mod runtime {
    use mio::{net, Interest};
    use once_cell::sync::Lazy;
    use std::{
        collections::HashMap,
        io::{ErrorKind, Read, Write},
        net::ToSocketAddrs,
        sync::{Arc, Mutex},
    };

    struct Reactor {}

    impl Reactor {
        fn new() -> Self {
            Self {}
        }
    }

    struct Executor {
        tasks: HashMap<usize, Task>,
    }

    struct Task {}

    pub fn run(f: fn() -> ()) {}

    pub struct TcpStream(net::TcpStream);

    impl TcpStream {
        pub fn connect<F>(addr: impl ToSocketAddrs) -> Promise<TcpStream>
        where
            F: FnOnce(TcpStream) + 'static, 
            {
            let stream = std::net::TcpStream::connect(addr).unwrap();
            let stream = net::TcpStream::from_std(stream);
            Promise::Fulfilled(Self(stream))
        }
        
        pub fn write_all<F>(mut self, data: &[u8]) -> Promise<TcpStream> 
            {
                self.0.write_all(data).unwrap();
                Promise::Fulfilled(self)
            }

        pub fn read_to_string(mut self) -> Promise<(TcpStream, String)>
        {
            let mut s = String::new();
            match self.0.read_to_string(&mut s) {
                Ok(_) => {
                    Promise::Fulfilled((self, s))
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => Promise::Pending(Box::new(|| {
                    self.read_to_string()
                })),

                Err(e) => panic!("e:?"),
            }
        }
    }

    pub enum Promise<T>
    {
        Fulfilled(T),
        Pending(Box<dyn Fn() -> Self + 'static>),
    }

    impl<T> Promise<T>
    {
        pub fn then<F>(self, f: F) -> Self
        where F: FnOnce(T) -> Self + 'static {
            match self {
                Promise::Fulfilled(s) => f(s),
                Promise::Pending(op) => {
                    op()
                } 
            }
        }
    }
}
