use std::{io::{self, Read, Result, Write}, net::TcpStream};

use ffi::Event;
use poll::Poll;

mod ffi;
mod poll;

/// Not the entire url, but everyhing after the domain addr
/// i.e. http://localhost/1000/hello => /1000/hello
fn get_req(url_part: &str) -> Vec<u8> {
    format!(
        "GET {url_part} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
    .bytes()
    .collect()
}


fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![];

        match streams[index].read_to_end(&mut data) {
            Ok(n) => {
                let txt = String::from_utf8_lossy(&data[..n]);
                println!("RECEIVED: {:?}", event);
                println!("{txt}\n------\n");
                handled_events += 1;
            }
            // Not ready to read in a non-blocking manner. This could
            // happen even if the event was reported as ready
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => return Err(e),
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
        let url_part = format!("/{delay}/request-{i}");
        let request = get_req(&url_part);
        let mut stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(&request)?;
        // NB! Token is equal to index in Vec
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLONESHOT)?;

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

        handled_events += handle_events(&events, &mut streams)?;
    }

    println!("FINISHED");
    Ok(())
}