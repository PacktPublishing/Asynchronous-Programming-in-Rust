use std::io::{ErrorKind, Read, Write};

use mio::{Interest, Registry, Token};

use crate::{
    future::PollState,
    runtime::{self, reactor, Waker},
    Future,
};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

pub struct Http;

impl Http {
    pub fn get(path: &str) -> impl Future<Output = String> {
        HttpGetFuture::new(path.to_string())
    }
}
struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    path: String,
    id: usize,
}

impl HttpGetFuture {
    fn new(path: String) -> Self {
        let id = reactor().next_id();
        Self {
            stream: None,
            buffer: vec![],
            path,
            id,
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        stream.set_nonblocking(true).unwrap();
        let mut stream = mio::net::TcpStream::from_std(stream);
        stream.write_all(get_req(&self.path).as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}

impl Future for HttpGetFuture {
    type Output = String;

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        // If this is first time polled, start the operation
        // see: https://users.rust-lang.org/t/is-it-bad-behaviour-for-a-future-or-stream-to-do-something-before-being-polled/61353
        // Avoid dns lookup this time
        if self.stream.is_none() {
            println!("FIRST POLL - START OPERATION");
            self.write_request();
            // CHANGED
            runtime::reactor().register(
                        self.stream.as_mut().unwrap(),
                        Interest::READABLE,
                        waker.clone(),
                        self.id,
                    );
            // ============
        }

        let mut buff = vec![0u8; 147];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    runtime::reactor().deregister(self.stream.as_mut().unwrap(), self.id);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    if !self.buffer.is_empty() {
                        continue;
                    }
                    
                    break PollState::NotReady;
                }

                Err(e) => panic!("{e:?}"),
            }
        }
    }
}
