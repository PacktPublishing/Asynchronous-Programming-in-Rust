use std::{thread, time::Duration};

mod future;
mod http;
mod runtime;

use future::{Future, PollState, Waker};
use runtime::{Executor, Reactor};

use crate::http::Http;

// This state machine would be similar to the one created by:
// async fn async_main() {
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").await;
//     println!("{txt}");
//     let txt = http::Http::get("/400/HelloAsyncAwait").await;
//     println!("{txt}");
// }

struct Coroutine {
    state: State,
}

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Program starting");
                    let fut = Box::new(Http::get("/600/HelloWorld1"));
                    self.state = State::Wait1(fut);
                }

                State::Wait1(ref mut fut) => match fut.poll(waker) {
                    PollState::Ready(txt) => {
                        println!("{txt}");
                        let fut2 = Box::new(Http::get("/400/HelloWorld2"));
                        self.state = State::Wait2(fut2);
                    }

                    PollState::NotReady => break PollState::NotReady,
                },

                State::Wait2(ref mut fut2) => match fut2.poll(waker) {
                    PollState::Ready(txt2) => {
                        println!("{txt2}");
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }

                    PollState::NotReady => break PollState::NotReady,
                },

                State::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

fn main() {
    let future = async_main();
    Reactor::start();
    let executor = Executor::new();
    executor.block_on(future);
}
