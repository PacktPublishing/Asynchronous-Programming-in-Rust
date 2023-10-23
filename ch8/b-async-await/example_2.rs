use std::time::Instant;

mod http;
mod future;

use future::*;
use crate::http::Http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i * 1000)
}

coro fn read_request(i: usize) {
    let txt = Http::get(&get_path(i)).wait;
    println!("{txt}");
}


coro fn async_main() {
    println!("Program starting");

    let mut futures = vec![];
    for i in 0..5 {
        futures.push(read_request(i));
    }

    futures.pop().unwrap().wait;
}


fn main() {
    let start = Instant::now();
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => (),
            PollState::Ready(_) => break,
        }
    }
    println!("\nELAPSED TIME: {}", start.elapsed().as_secs_f32());
}