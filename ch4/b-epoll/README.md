


## Notes

Change `delay` function on the delayserver to return huge amount of fill data
(enough to force a `WouldBlock` error on the reciever):

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

To get a more readable output, you can change the `handle_events` function
to something like this. You still have to handle the `WouldBlock` error in
this case.

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

