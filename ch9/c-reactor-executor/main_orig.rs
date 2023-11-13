mod future;
mod http;
mod runtime;

use future::{Future, PollState};
use runtime::Waker;

use crate::http::Http;

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}


coro fn request(i: usize) {
    let path = format!("/{}/HelloWorld{i}", i * 1000);
    let txt = Http::get(&path).wait;
    println!("{txt}");

}


coro fn async_main() {
    println!("Program starting");

    for i in 0..5 {
        let future = request(i);
        runtime::spawn(future);
    }
}