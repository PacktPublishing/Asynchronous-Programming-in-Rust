use std::time::Instant;

mod http;
mod future;

use future::*;
use crate::http::Http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i * 1000)
}

coro fn async_main() {
    println!("Program starting");

    let txt = Http::get(&get_path(0)).wait;
    println!("{txt}");
    let txt = Http::get(&get_path(1)).wait;
    println!("{txt}");
    let txt = Http::get(&get_path(2)).wait;
    println!("{txt}");
    let txt = Http::get(&get_path(3)).wait;
    println!("{txt}");
    let txt = Http::get(&get_path(4)).wait;
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