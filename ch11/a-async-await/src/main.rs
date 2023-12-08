mod future;
mod http;
mod runtime;
use crate::http::Http;
use std::fmt::Write;

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

async fn async_main() {
    let mut buffer = String::from("\nBUFFER:\n----\n");
    let writer = &mut buffer;
    println!("Program starting");
    let txt = Http::get("/600/HelloAsyncAwait").await;
    writeln!(writer, "{txt}").unwrap();
    let txt = Http::get("/400/HelloAsyncAwait").await;
    writeln!(writer, "{txt}").unwrap();

    println!("{}", buffer);
}

// use isahc::prelude::*;

// async fn async_main2() {
//     let mut buffer = String::from("\nBUFFER:\n----\n");
//     let writer = &mut buffer;
//     println!("Program starting");
//     let mut res = isahc::get_async("http://127.0.0.1:8080/600/HelloAsyncAwait").await.unwrap();
//     let txt = res.text().await.unwrap();
//     writeln!(writer, "{txt}").unwrap();
//     let mut res = isahc::get_async("http://127.0.0.1:8080/400/HelloAsyncAwait").await.unwrap();
//     let txt = res.text().await.unwrap();
//     writeln!(writer, "{txt}").unwrap();

//     println!("{}", buffer);
// }

// async fn spawn_many() {
//     for i in 0..100 {
//         let delay = i * 10;
//         let req = format!("http://127.0.0.1:8080/{delay}/HelloAsyncAwait{i}");

//         runtime::spawn(async move {
//             let mut res = isahc::get_async(&req).await.unwrap();
//             let txt = res.text().await.unwrap();
//             println!("{txt}");
//         });
//     }
// }
