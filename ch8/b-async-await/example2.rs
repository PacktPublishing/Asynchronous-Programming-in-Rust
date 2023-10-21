use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;

fn read_request(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i * 1000)
}

coro fn async_main() {
    println!("Program starting");

    let txt = Http::get(&read_request(0)).wait;
    println!("{txt}");
    let txt = Http::get(&read_request(1)).wait;
    println!("{txt}");
    let txt = Http::get(&read_request(2)).wait;
    println!("{txt}");
    let txt = Http::get(&read_request(3)).wait;
    println!("{txt}");
    let txt = Http::get(&read_request(4)).wait;
}


fn main() {
    let start = Instant::now();
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }

    println!("\nELAPSED TIME: {}", start.elapsed().as_secs_f32());
}
