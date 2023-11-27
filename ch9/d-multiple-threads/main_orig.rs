mod future;
mod http;
mod runtime;
use crate::http::Http;
use future::{Future, PollState};
use runtime::{Executor, Waker};
use std::thread::Builder;

fn main() {
    let mut executor = runtime::init();
    let mut handles = vec![];
    
    for i in 1..12 {
        let name = format!("exec-{i}");
        let h = Builder::new().name(name).spawn(move || {
            let mut executor = Executor::new();
            executor.block_on(async_main());
        }).unwrap();
        handles.push(h);
    }
    executor.block_on(async_main());
    handles.into_iter().for_each(|h| h.join().unwrap());
}

coro fn request(i: usize) {
    let path = format!("/{}/HelloWorld{i}", i * 1000);
    let txt = Http::get(&path).wait;
    let txt = txt.lines().last().unwrap_or_default();
    println!("{txt}");


}

coro fn async_main() {
    println!("Program starting");

    for i in 0..5 {
        let future = request(i);
        runtime::spawn(future);
    }
}