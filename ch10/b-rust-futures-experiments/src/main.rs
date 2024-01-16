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

use tokio::runtime::Runtime;
async fn async_main2() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    println!("Program starting");
    let url = "http://127.0.0.1:8080/600/HelloAsyncAwait1";
    let res = reqwest::get(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
    let url = "http://127.0.0.1:8080/400/HelloAsyncAwait2";
    let res = reqwest::get(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
}


use isahc::prelude::*;
async fn async_main3() {
    println!("Program starting");
    let url = "http://127.0.0.1:8080/600/HelloAsyncAwait1";
    let mut res = isahc::get_async(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
    let url = "http://127.0.0.1:8080/400/HelloAsyncAwait2";
    let mut res = isahc::get_async(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
}


async fn spawn_many() {
    for i in 0..100 {
        let delay = i * 10;
        let req = format!("http://127.0.0.1:8080/{delay}/HelloAsyncAwait{i}");

        runtime::spawn(async move {
            let mut res = isahc::get_async(&req).await.unwrap();
            let txt = res.text().await.unwrap();
            println!("{txt}");
        });
    }
}
