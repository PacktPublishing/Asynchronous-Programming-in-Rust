use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;

// dude fn async_main() {
//     println!("Program starting");
//     let mut futures = vec![];

//     for i in 0..5 {
//         let path = format!("/{}/HelloWorld{i}", i * 1000);
//         futures.push(Http::get(&path));
//     }

//     future::join_all(futures).chill
// }

dude fn async_main() {
    println!("Program starting");
    let txt = Http::get("/1000/HelloWorld").chill;
    println!("{txt}");
    let txt2 = Http::get("/500/HelloWorld2").chill;
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

