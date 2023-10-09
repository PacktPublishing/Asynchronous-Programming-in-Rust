use std::io::{self, Read, Result, Write};

use mio::event::Event;
use mio::net::TcpStream;
use mio::{Interest, Poll, Token};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // We need to extract the value we wrapped in Token
        let index: usize = event.token().into();
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        let mut data = vec![];

        match streams[index].read_to_end(&mut data) {
            Ok(n) => {
                let txt = String::from_utf8_lossy(&data[..n]);
                println!("RECEIVED: {:?}", event);
                println!("{txt}\n------\n");
                handled_events += 1;
            }
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
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let std_stream = std::net::TcpStream::connect(addr)?;
        std_stream.set_nonblocking(true)?;

        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // Mio wraps `std::net::TcpStream`
        let mut stream = TcpStream::from_std(std_stream);
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

        stream.write_all(request.as_bytes())?;

        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // Slightly different arguments. `Token` is a wrapper so we just wrap the value
        // Interests are expressed slightly different but boil down to the same
        // arguments to `epoll_ctl`
        poll.registry()
            .register(&mut stream, Token(i), Interest::READABLE)?;
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

        streams.push(stream);
    }

    let mut handled_events = 0;
    while handled_events < n_events {
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // Mio has it's own collection type instead of Vec<Event>
        let mut events = mio::Events::with_capacity(10);
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // To make minimal changes to the existing code we create a Vec<Event> from mio's
        // Events collection
        let events: Vec<Event> = events.into_iter().map(|e| e.clone()).collect();
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

        handled_events += handle_events(&events, &mut streams)?;
    }

    println!("FINISHED");
    Ok(())
}
