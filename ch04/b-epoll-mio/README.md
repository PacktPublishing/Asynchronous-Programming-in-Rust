# b-epoll-mio

This create contains the example code for chapter 4, but instead of using
our own queue, we use one created by [mio](https://github.com/tokio-rs/mio). Since we modelled our own code after
mio you only need to make a few very minor changes to get it working which I've
commented and marked out as clearly as I can.

If running on a Mac system (which only supports kqueue but not epoll), docker
can be used to run the example by running the epoll_mio_docker.sh script

You can run the example by simply writing `cargo run`

## Note

There is one downside of having a local server on the same machine to mimic
real life behavior. The network will never be slow, and packages will never
need to be resent. Latency is not a problem either.

For example, encountering an actual `io::WouldBlock` error when reading
from a `TcpStream` that's reported as ready is very hard, but if you
want to test it out I manage to reliably get that error by simply transferring
so much data that the OS needs to do extra work to handle it.

You can reproduce it if you make som minor changes to the delayserver code
as well as the program in main.rs as outlined below. Simply copy and replace
the appropriate functions with these will do it.


First, change the `delay` function on the delayserver to return huge amount of fill data
(enough to force a `WouldBlock` error on the receiver):

```rust
#[get("/{delay}/{message}")]
async fn delay(path: web::Path<(u64, String)>) -> impl Responder {
    let (delay_ms, message) = path.into_inner();
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("#{count} - {delay_ms}ms: {message}");
    sleep(Duration::from_millis(delay_ms)).await;
    let extra: String = "HelloWorld".chars().cycle().take(100_000).collect();
    format!("{message} -- {extra}")
}
```

Secondly, to get a more readable output, you probably should change the
`handle_events` function to something like this. You can print out a snapshot
of state or just a message when you encounter the WouldBlock error.

```rust
fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 150];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    if txt.starts_with("HTTP") {
                        println!("RECEIVED: {:?}", event);
                        println!("{txt}\n------\n");
                    }
                }
                // Not ready to read in a non-blocking manner. This could
                // happen even if the event was reported as ready
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}
```
