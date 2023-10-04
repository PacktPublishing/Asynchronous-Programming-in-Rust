use std::{
    io::{self, Result},
    thread::sleep,
    time::Duration,
};

use net::TcpStream;
use poll::Poll;

mod ffi;
mod net;
mod poll;

fn send_requests(poll: &Poll) -> Result<Vec<TcpStream>> {
    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..5 {
        let delay = (5 - i) * 1000;
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

/// This is the example from the book (in the book we put the code in `main`
/// instead of this function). The function waits for events using epoll and
/// handles the events as they arrive.
fn wait_with_epoll() -> Result<()> {
    let mut poll = poll::Poll::new()?;
    let streams = send_requests(&poll)?;
    let mut events_to_handle = streams.len();

    while events_to_handle > 0 {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        for event in events {
            let index = event.epoll_data;
            let mut data = vec![0u8; 4096];
            match streams[index].read(&mut data) {
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");

                    // we consider one event as handled
                    events_to_handle -= 1;
                    break;
                }

                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }

    println!("FINISHED");
    Ok(())
}

/// This function is just an example on how you can experiment with this code.
/// Here we send the requests and immidiately try to read data from socket #4.
/// Data won't be ready yet so we get a WouldBlock error until the data is
/// ready to be read on the socket.
#[allow(dead_code)]
fn poll_in_loop() -> Result<()> {
    let poll = poll::Poll::new()?;
    let streams = send_requests(&poll)?;

    loop {
        let mut data = vec![0u8; 4096];
        // poll the last one in stream (delay = 1000 ms)
        match streams[4].read(&mut data) {
            Ok(n) if n == 0 => {
                println!("Got 0 bytes. No more data!");
                break;
            }
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

/// This is the example from the book (in the book we put the code in `main`
/// instead of this function). The function waits for events using epoll and
/// handles the events as they arrive.
#[allow(dead_code)]
fn wait_with_epoll_extended() -> Result<()> {
    let mut poll = poll::Poll::new()?;
    let streams = send_requests(&poll)?;
    let mut events_to_handle = streams.len();

    while events_to_handle > 0 {
        let mut events = Vec::with_capacity(10);

        // experiment setting different timeouts
        poll.poll(&mut events, Some(100))?;

        if events.is_empty() {
            // We either got a timeout or a spurious wakeup.
            println!("TIMEOUT");
            continue;
        }

        for event in events {
            let index = event.epoll_data;
            let mut data = vec![0u8; 4096];

            // We should always read in a loop since we can get
            // more data than our buffer can hold
            loop {
                match streams[index].read(&mut data) {
                    /*
                    The only way we should get a count of 0 is if the
                    size of the data recieved was just large enough to fill
                    the entire buffer so we had to call read again to check
                    if there was more data on the socket. If that happens,
                    we conclude that the event is handled and stop reading
                    the socket
                    */
                    Ok(n) if n == 0 => {
                        events_to_handle -= 1;
                        break;
                    }
                    Ok(n) => {
                        let txt = String::from_utf8_lossy(&data[..n]);
                        println!("RECEIVED: {:?}", event);
                        println!("{txt}\n------\n");

                        /*
                        If we get less data than our buffer could hold we
                        consider the event as handled.

                        In real life this
                        might be a bit more complicated since you can't
                        always rely on all the data to come in one read call.
                        If you expect more data related to handling the event
                        you would need to re-enable the event in the epoll
                        instance using `epoll_ctl` with the `EPOLL_CTL_MOD`
                        argument, since epoll was configured with
                        EPOLLONESHOT for this event. This way you will get
                        further notifications when more data is available.
                        */
                        if n < data.len() {
                            events_to_handle -= 1;
                            break;
                        }
                    }
                    // This error could occur on both writes and reads under
                    // certain conditions and should be expected.
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
    wait_with_epoll()
    // wait_with_epoll_extended()
    //poll_in_loop()

}
