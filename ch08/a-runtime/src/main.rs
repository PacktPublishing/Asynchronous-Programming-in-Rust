mod future;
mod http;
mod runtime;

use future::{Future, PollState};
use runtime::Runtime;

fn main() {
    let future = async_main();
    let mut runtime = Runtime::new();
    runtime.block_on(future);
}



// =================================
// We rewrite this:
// =================================

// coroutine fn async_main() {
//     println!("Program starting");
//     
//     let txt = http::Http::get("/600/HelloAsyncAwait").wait;
//     println!("{txt}");
//     let txt = http::Http::get("/400/HelloAsyncAwait").wait;
//     println!("{txt}");

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
                    let fut1 = Box::new( http::Http::get("/600/HelloAsyncAwait"));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{txt}");

                            // ---------------------------------
                            let fut2 = Box::new( http::Http::get("/400/HelloAsyncAwait"));
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
