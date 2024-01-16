mod http;
mod runtime;
use crate::http::Http;


fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

async fn async_main() {
    println!("Program starting");
    let txt = Http::get("/600/HelloAsyncAwait").await;
    println!("{txt}");
    let txt = Http::get("/400/HelloAsyncAwait").await;
    println!("{txt}");
}

