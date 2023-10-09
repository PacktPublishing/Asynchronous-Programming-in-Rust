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

fn main() -> Result<()> {
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
        streams.push(stream);
    }

    for stream in &mut streams {
        let mut data = vec![];
        match stream.read_to_end(&mut data) {
            Ok(n) => {
                let txt = String::from_utf8_lossy(&data[..n]);
                println!("{txt}\n------\n");
            }

            Err(e) => eprintln!("ERROR: {e}"),
        }
    }

    println!("FINISHED");
    Ok(())
}