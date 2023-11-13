use std::time::Instant;

mod http;
mod future;

use future::*;
use crate::http::Http;






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
    
// coro fn request(i: usize) {
//     let path = format!("/{}/HelloWorld{i}", i * 1000);
//     let txt = Http::get(&path).wait;
//     println!("{txt}");

// }

// =================================
// Into this:
// =================================

fn request(i: usize) -> impl Future<Output=String> {
    Coroutine0::new(i)
}
        
enum State0 {
    Start(usize),
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine0 {
    state: State0,
}

impl Coroutine0 {
    fn new(i: usize) -> Self {
        Self { state: State0::Start(i) }
    }
}


impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State0::Start(i) => {
                    // ---- Code you actually wrote ----
                    let path = format!("/{}/HelloWorld{i}", i * 1000);

                    // ---------------------------------
                    let fut1 = Box::new( Http::get(&path));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

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


// =================================
// We rewrite this:
// =================================
    
// coro fn async_main() {
//     println!("Program starting");
//     let mut futures = vec![];
// 
//     for i in 0..5 {
//         futures.push(request(i));
//     }
// 
//     future::join_all(futures).wait;

// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output=String> {
    Coroutine1::new()
}
        
enum State1 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine1 {
    state: State1,
}

impl Coroutine1 {
    fn new() -> Self {
        Self { state: State1::Start }
    }
}


impl Future for Coroutine1 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State1::Start => {
                    // ---- Code you actually wrote ----
                    println!("Program starting");
    let mut futures = vec![];

    for i in 0..5 {
        futures.push(request(i));
    }


                    // ---------------------------------
                    let fut1 = Box::new(future::join_all(futures));
                    self.state = State1::Wait1(fut1);
                }

                State1::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(_) => {
                            // ---- Code you actually wrote ----
                        
                            // ---------------------------------
                            self.state = State1::Resolved;
                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State1::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}
