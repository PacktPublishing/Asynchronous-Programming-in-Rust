use std::{
    io::{ErrorKind, Read, Write},
    thread,
    time::Duration,
};

use runtime::join_all;

// later fn async_main() {
//     println!("Program starting")
//     let mut buffer = String::new()
//     let formatter = Formatter::new(&mut buffer);
//     let http = Http::new();
//     let txt = siesta http.get("/1000/HelloWorld");
//     formatter.format(txt);
//     let txt2 = siesta http.get("500/HelloWorld2");
//     formatter.format(txt2);
//     println!("{}", buffer);
// }

// later fn async_main() {
//     println!("Program starting")

//     let http = Http::new();
//     let txt = siesta http.get("/1000/HelloWorld");
//     println!("{txt}");
//     let txt2 = siesta http.get("500/HelloWorld2");
//     println!("{txt2}");
// }

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

struct Formatter<'a> {
    buffer: &'a mut String,
}

impl<'a> Formatter<'a> {
    fn new(buffer: &'a mut String) -> Self {
        Self { buffer }
    }
    fn format(&mut self, txt: String) {
        *self.buffer += "---------\n";
        *self.buffer += &txt;
        *self.buffer += "+++++++++\n";
    }
}

struct Http;

impl Http {
    fn get(path: &'static str) -> impl Future<Output = String> {
        HttpGetFuture::new(path)
    }
}

struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    path: &'static str,
}

impl HttpGetFuture {
    fn new(path: &'static str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path,
        }
    }
}

impl Future for HttpGetFuture {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        // If this is first time polled, start the operation
        // see: https://users.rust-lang.org/t/is-it-bad-behaviour-for-a-future-or-stream-to-do-something-before-being-polled/61353
        // Avoid dns lookup this time
        if self.stream.is_none() {
            println!("FIRST POLL - START OPERATION");
            let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
            stream.set_nonblocking(true).unwrap();
            let mut stream = mio::net::TcpStream::from_std(stream);
            stream.write_all(get_req(self.path).as_bytes()).unwrap();
            self.stream = Some(stream);
            return PollState::NotReady;
        }

        let mut buff = vec![0u8; 4096];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break PollState::NotReady;
                }

                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

enum MyOneStageFut<'a> {
    Start,
    Wait1(Box<dyn Future<Output = String>>, String, Formatter<'a>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl<'a> MyOneStageFut<'a> {
    fn new() -> Self {
        Self::Start
    }
}

impl<'a> Future for MyOneStageFut<'a> {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        let mut this = std::mem::replace(self, Self::Resolved);
        match this {
            Self::Start => {
                println!("Program starting");
                let mut buffer = String::new();
                let formatter = Formatter::new(&mut buffer);
                let fut = Box::new(Http::get("/1000/HelloWorld1"));
                *self = MyOneStageFut::Wait1(fut, buffer, formatter);
                PollState::NotReady
            }

            Self::Wait1(ref mut fut, ref mut buffer, ref mut formatter) => {
                let txt = match fut.poll() {
                    PollState::Ready(s) => s,
                    PollState::NotReady => {
                        *self = this;
                        return PollState::NotReady;
                    }
                };
                println!("{txt}");

                let fut2 = Box::new(Http::get("/600/HelloWorld2"));
                *self = Self::Wait2(fut2);
                PollState::NotReady
            }

            Self::Wait2(ref mut fut) => {
                let txt2 = match fut.poll() {
                    PollState::Ready(s) => s,
                    PollState::NotReady => {
                        *self = this;
                        return PollState::NotReady;
                    }
                };
                println!("{txt2}");
                *self = Self::Resolved;
                PollState::Ready(())
            }

            Self::Resolved => panic!("Polled a resolved future"),
        }
    }
}


pub trait Future {
    type Output;
    fn poll(&mut self) -> PollState<Self::Output>;
}

pub enum PollState<T> {
    Ready(T),
    NotReady,
}

fn async_main() -> impl Future<Output = ()> {
    MyOneStageFut::new()
}

fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(s) => break s,
        }
    }
}

mod runtime {
    use super::*;

    pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
        let futures = futures.into_iter().map(|f| (false, f)).collect();
        JoinAll {
            futures,
            finished_count: 0,
        }
    }

    pub struct JoinAll<F: Future> {
        futures: Vec<(bool, F)>,
        finished_count: usize,
    }

    impl<F: Future> Future for JoinAll<F> {
        type Output = ();

        fn poll(&mut self) -> PollState<Self::Output> {
            for (finished, fut) in self.futures.iter_mut() {
                if *finished {
                    continue;
                }

                match fut.poll() {
                    PollState::Ready(_) => {
                        *finished = true;
                        self.finished_count += 1;
                    }

                    PollState::NotReady => continue,
                }
            }

            if self.finished_count == self.futures.len() {
                PollState::Ready(())
            } else {
                PollState::NotReady
            }
        }
    }
}
