use std::{io::{self, Result}, thread::sleep, time::Duration};

use net::TcpStream;
use poll::Poll;

mod ffi;
mod net;
mod poll;


fn send_requests(poll: &Poll) -> Result<Vec<TcpStream>> {
    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..5 {
        let delay = (5-i) * 1000;
        let request = format!(
            "GET /{}/request-{} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n",
            delay, i
        );
        let stream = net::TcpStream::write(addr, request.as_bytes())?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLONESHOT)?;

        streams.push(stream);
    }
    Ok(streams)
}

fn poll_in_loop() -> Result<()> {
    let poll = poll::Poll::new()?;
    let streams = send_requests(&poll)?;

    loop {
        let mut data = vec![0u8; 4098];
        // poll the last one in stream (delay = 1000 ms)
        match streams[4].read(&mut data) {
            Ok(n) if n == 0 => {
                println!("Got 0 bytes. No more data!");
                break;
            },
            Ok(n) => {
                let txt = String::from_utf8_lossy(&data[..n]);
                println!("RESPONSE STREAM 4:\n{txt}");
            }
            Err(e) => {
                println!("{e:?}");
                sleep(Duration::from_millis(100));
            }
        }
    }

    println!("FINISHED");
    Ok(())
}

fn with_epoll_wait() -> Result<()> {
    let mut poll = poll::Poll::new()?;
    let streams = send_requests(&poll)?;
    let mut events_to_handle = streams.len();

    while events_to_handle > 0 {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        for event in events {
            let index = event.epoll_data;
            let mut data = vec![0u8; 4098];
            
            loop {
                match streams[index].read(&mut data) {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                    // we consider one event as handled
                    events_to_handle -= 1;
                }

                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
            }
            
        }
    }

    println!("FINISHED");
    Ok(())
}

fn main() -> Result<()> {
    //poll_in_loop()
    with_epoll_wait()
}