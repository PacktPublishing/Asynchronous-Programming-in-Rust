use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i");
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
            PollState::NotReady => {
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }
    println!("\nELAPSED TIME: {}", start.elapsed().as_secs_f32());
}
