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
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }
}