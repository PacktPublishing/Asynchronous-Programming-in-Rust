//! # FIXES:
//!
//! The number is identical to the number in the GitHub issue tracker
//!
//! ## FIX ISSUE #4:
//!
//! See:https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/4
//! Some users reported false event notification causing the counter to increase
//! due to the OS reporting a READ event after we already read the TcpStream to EOF.
//! This caused the counter to increment on the same TcpStream twice and thereby
//! exiting the program before all events were handled.
//!
//! The fix for this is to account for false wakeups which is an easy fix but requires
//! a few changes to the example. I've added an explicit comment: "FIX #4", the places
//! I made a change so it's easy to spot the differences to the example code in the book.
//!
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

// FIX #4 (import `HashSet``)
use std::collections::HashSet;
use std::env;
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

fn handle_events(events: &[Event], streams: &mut [TcpStream], handled: &mut HashSet<usize>) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // We need to extract the value we wrapped in Token
        let index: usize = event.token().into();
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        let mut data = vec![0u8; 4096];
        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    // FIX #4
                    // `insert` returns false if the value already existed in the set. We
                    // handle it here since we must be sure that the TcpStream is fully
                    // drained due to using edge triggered epoll.
                    if !handled.insert(index) {
                        break;
                    }
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                // this was not in the book example, but it's a error condition
                // you probably want to handle in some way (either by breaking
                // out of the loop or trying a new read call immediately)
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
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

    let args: Vec<String> = env::args().collect();
    let base_url;
    if args.len() > 1 {
        base_url = args[1].clone();
    } else {
        base_url = String::from("localhost");
    }
    let addr = format!("{}:8080", &base_url);

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

    // FIX #4: store the handled IDs
    let mut handled_ids = HashSet::new();

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

        // ------------------------------------------------------⌄ FIX #4 (new signature)
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("FINISHED");
    Ok(())
}
