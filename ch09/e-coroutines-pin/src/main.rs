mod future;
mod http;
mod runtime;
use future::{Future, PollState};
use runtime::Waker;
use std::{fmt::Write, marker::PhantomPinned, pin::Pin};

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

// =================================
// We rewrite this:
// =================================

// coro fn async_main() {
//     let mut buffer = String::from("\nBUFFER:\n----\n");
//     let writer = &mut buffer;
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").wait;
//     writeln!(writer, "{txt}").unwrap();
//     let txt = http::Http::get("/400/HelloAsyncAwait").wait;
//     writeln!(writer, "{txt}").unwrap();
//
//     println!("{}", buffer);
// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output = String> {
    Coroutine0::new()
}

enum State0 {
    Start,
    Wait1(Pin<Box<dyn Future<Output = String>>>),
    Wait2(Pin<Box<dyn Future<Output = String>>>),
    Resolved,
}

#[derive(Default)]
struct Stack0 {
    buffer: Option<String>,
    writer: Option<*mut String>,
}

struct Coroutine0 {
    stack: Stack0,
    state: State0,
    _pin: PhantomPinned,
}

impl Coroutine0 {
    fn new() -> Self {
        Self {
            state: State0::Start,
            stack: Stack0::default(),
            _pin: PhantomPinned,
        }
    }
}

impl Future for Coroutine0 {
    type Output = String;

    fn poll(self: Pin<&mut Self>, waker: &Waker) -> PollState<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match this.state {
                State0::Start => {
                    // initialize stack (hoist declarations - no stack yet)
                    this.stack.buffer = Some(String::from("\nBUFFER:\n----\n"));
                    this.stack.writer = Some(this.stack.buffer.as_mut().unwrap());
                    // ---- Code you actually wrote ----
                    println!("Program starting");

                    // ---------------------------------
                    let fut1 = Box::pin(http::Http::get("/600/HelloAsyncAwait"));
                    this.state = State0::Wait1(fut1);

                    // save stack
                    // nothing to save
                }

                State0::Wait1(ref mut f1) => {
                    match f1.as_mut().poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            let writer = unsafe { &mut *this.stack.writer.take().unwrap() };

                            // ---- Code you actually wrote ----
                            writeln!(writer, "{txt}").unwrap();
                            // ---------------------------------
                            let fut2 = Box::pin(http::Http::get("/400/HelloAsyncAwait"));
                            this.state = State0::Wait2(fut2);

                            // save stack
                            this.stack.writer = Some(writer);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.as_mut().poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            let buffer = this.stack.buffer.as_ref().take().unwrap();
                            let writer = unsafe { &mut *this.stack.writer.take().unwrap() };

                            // ---- Code you actually wrote ----
                            writeln!(writer, "{txt}").unwrap();

                            println!("{}", buffer);
                            // ---------------------------------
                            this.state = State0::Resolved;

                            // Save stack / free resources
                            let _ = this.stack.buffer.take();

                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}
