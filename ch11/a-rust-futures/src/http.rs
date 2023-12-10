use crate::runtime::{self, reactor};
use mio::Interest;
use std::{
    future::Future,
    io::{ErrorKind, Read, Write},
    pin::Pin,
    task::{Context, Poll},
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // If this is first time polled, start the operation
        // see: https://users.rust-lang.org/t/is-it-bad-behaviour-for-a-future-or-stream-to-do-something-before-being-polled/61353
        // Avoid dns lookup this time
        //let this = self.get_mut();

        let id = self.id;
        if self.stream.is_none() {
            println!("FIRST POLL - START OPERATION");
            self.write_request();
            // CHANGED
            let stream = (&mut self).stream.as_mut().unwrap();
            runtime::reactor().register(stream, Interest::READABLE, id);
            runtime::reactor().set_waker(cx, self.id);
            // ============
        }

        let mut buff = vec![0u8; 147];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer).to_string();
                    runtime::reactor().deregister(self.stream.as_mut().unwrap(), id);
                    break Poll::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // always store the last given Waker
                    runtime::reactor().set_waker(cx, self.id);
                    break Poll::Pending;
                }

                Err(e) => panic!("{e:?}"),
            }
        }
    }
}
