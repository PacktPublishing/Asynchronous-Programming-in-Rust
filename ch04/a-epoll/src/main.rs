//! # FIXES:
//! The number is identical to the number in the GitHub issue tracker
//!
//! ## FIX ISSUE #4:
//! See:https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/4
//! Some users reported false event notification causing the counter to increase
//! due to the OS reporting a READ event after we already read the TcpStream to EOF.
//! This caused the counter to increment on the same TcpStream twice and thereby
//! exiting the program before all events were handled.
//!
//! The fix for this is to account for false wakeups which is an easy fix but requires
//! a few changes to the example. I've added an explicit comment: "FIX #4", the places
//! I made a change so it's easy to spot the differences to the example code in the book.

use std::{
    // FIX #4 (import `HashSet``)
    collections::HashSet,
    io::{self, Read, Result, Write},
    net::TcpStream,
    env
};

use ffi::Event;
use poll::Poll;

mod ffi;
mod poll;

/// Not the entire url, but everyhing after the domain addr
/// i.e. http://localhost/1000/hello => /1000/hello
fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn handle_events(
    events: &[Event],
    streams: &mut [TcpStream],
    // FIX #4: accepts a set of handled events as argument
    handled: &mut HashSet<usize>,
) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    // FIX #4
                    // `insert` returns false if the value already existed in the set.
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

    let base_url = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost"));

    let addr = format!("{}:8080", &base_url);

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(&addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        // NB! Token is equal to index in Vec
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;

        streams.push(stream);
    }

    // FIX #4: store the handled IDs
    let mut handled_ids = HashSet::new();

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        // ------------------------------------------------------⌄ FIX #4 (new signature)
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("FINISHED");
    Ok(())
}
