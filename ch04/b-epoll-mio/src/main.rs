//! ## TROUBLESHOOTING (KNOWN POTENTIAL ISSUE)
//!
//! ### EXAMPLE DOESN'T WORK AS EXPECTED - PROBLEM WITH DNS LOOKUP
//! If you first run this example on Linux under WSL and then immediately run it on
//! Windows, I've observed issues with the DNS lookup for "localhost" being so slow
//! that it defeats the purpose of the example. This issue could potentially also
//! happen under other scenarios than the one mentioned here and the fix will be
//! the same regardless.
//!
//! I don't consider this a bug with our code but a surprising behavior of the
//! WSL/Windows network stack. Anyway, if you encounter this, the fix is simple:
//!
//! Change `base_url = String::from("localhost");` to `base_url = String::from("127.0.0.1");`.
//!

use std::io::{self, Read, Result, Write};
use mio::event::Event;
use mio::net::TcpStream;
use mio::{Interest, Poll, Token, Registry};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn handle_events(events: &[Event], streams: &mut [TcpStream], registry: &Registry) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // We need to extract the value we wrapped in Token
        let index: usize = event.token().into();
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        let mut data = vec![0u8; 4096];
        loop {
            let stream = &mut streams[index];
            match stream.read(&mut data) {
                Ok(n) if n == 0 => {
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
        let std_stream = std::net::TcpStream::connect(&addr)?;
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
        let registry = poll.registry();
        handled_events += handle_events(&events, &mut streams, &registry)?;
    }

    println!("FINISHED");
    Ok(())
}
