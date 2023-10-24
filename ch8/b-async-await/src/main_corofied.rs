use std::time::Instant;

mod http;
mod future;

use future::*;
use crate::http::Http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i * 1000)
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

// =================================
// We rewrite this:
// =================================
    
// coro fn async_main() {
//     println!("Program starting");
// 
//     let txt = Http::get(&get_path(0)).wait;
//     println!("{txt}");
//     let txt = Http::get(&get_path(1)).wait;
//     println!("{txt}");
//     let txt = Http::get(&get_path(2)).wait;
//     println!("{txt}");
//     let txt = Http::get(&get_path(3)).wait;
//     println!("{txt}");
//     let txt = Http::get(&get_path(4)).wait;

// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output=String> {
    Coroutine0::new()
}
        
enum State0 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Wait3(Box<dyn Future<Output = String>>),
    Wait4(Box<dyn Future<Output = String>>),
    Wait5(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine0 {
    state: State0,
}

impl Coroutine0 {
    fn new() -> Self {
        Self { state: State0::Start }
    }
}


impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State0::Start => {
                    // ---- Code you actually wrote ----
                    println!("Program starting");


                    // ---------------------------------
                    let fut1 = Box::new( Http::get(&get_path(0)));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

                            // ---------------------------------
                            let fut2 = Box::new( Http::get(&get_path(1)));
                            self.state = State0::Wait2(fut2);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

                            // ---------------------------------
                            let fut3 = Box::new( Http::get(&get_path(2)));
                            self.state = State0::Wait3(fut3);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait3(ref mut f3) => {
                    match f3.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

                            // ---------------------------------
                            let fut4 = Box::new( Http::get(&get_path(3)));
                            self.state = State0::Wait4(fut4);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait4(ref mut f4) => {
                    match f4.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

                            // ---------------------------------
                            let fut5 = Box::new( Http::get(&get_path(4)));
                            self.state = State0::Wait5(fut5);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait5(ref mut f5) => {
                    match f5.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                        
                            // ---------------------------------
                            self.state = State0::Resolved;
                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}
