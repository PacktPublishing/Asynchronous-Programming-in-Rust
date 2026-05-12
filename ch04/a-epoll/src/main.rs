//! ## Mac users:
//! To make this example work with Docker for Mac users there is a small change
//! to the code where you can override "localhost" by passing in a command line
//! argument.

use ffi::Event;
use poll::{Poll, Registry};
use std::{io::{self, Read, Result, Write}, net::TcpStream};

mod ffi;
mod poll;

/// Not the entire url, but everything after the domain addr
/// i.e. http://localhost/1000/hello => /1000/hello
fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn handle_events(events: &[Event], streams: &mut [TcpStream], registry: &Registry) -> Result<u64> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token() as usize;
        let mut data = vec![0u8; 4096];

        loop {
            let stream = &mut streams[index];
            match stream.read(&mut data) {
                Ok(0) => {
                    registry.deregister(stream)?;
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(&addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        // NB! Token is equal to index in Vec
        poll.registry().register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;

        streams.push(stream);
    }

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }
        let registry = poll.registry();
        handled_events += handle_events(&events, &mut streams, &registry)?;
    }

    println!("FINISHED");
    Ok(())
}
