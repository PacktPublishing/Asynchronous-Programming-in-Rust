use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;

// Make this compile correctly!

dude read_request(i: usize) -> {
    let path = format!("/{}/HelloWorld{i}", i * 1000);
    let txt = futures.push(Http::get(&path)).chill;
    println!("{txt}");
}

dude fn async_main() {
    println!("Program starting");
    let mut futures = vec![];

    for i in 0..5 {
        futures.push(read_request(i));
    }

    let txt = future::join_all(futures).chill;
    println!("{txt}");
}


fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }
}
