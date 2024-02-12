# b-async-await

This is the first example where we introduce the `coro/wait`
keywords and use `corofy` to re-write our coro-functions
into state-machines.

Install corofy by entering the `ch8/corofy` folder and
write `cargo install --path .`.

When that's done you can come back here and make sure that
`src/main.rs` contains the same code as the `original_main.rs`
file does (which is the same as presented in the book).

## How to run the example

To reqrite the coro/wait functions into state machines write
`corofy ./src/main.rs`.

You should find the re-written file in `src/main_corofied.rs`.
You can take the contents of this file and paste it into
`main.rs` and run the program.

An alternative way of accomplishing the same task is
writing `corofy ./original_main.rs ./src/main.rs`. In this case
you don't have to copy/paste anything.

## Note

You can confirm that corofy writes the exact same state machine
as we did in the first example (found in `a-coroutine`), by placing
the following code in main.rs and following the instructions in
**How to run the exaple** segment above.

```rust
use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;

coro fn async_main() {
    println!("Program starting");
    let txt = Http::get("/1000/HelloWorld").wait;
    println!("{txt}");
    let txt2 = Http::get("/500/HelloWorld2").wait;
    println!("{txt2}");
}


fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("Schedule other tasks");
            }
            PollState::Ready(_) => break,
        }

        // Since we print every poll, slow down the loop
        thread::sleep(Duration::from_millis(100));
    }
}
```
