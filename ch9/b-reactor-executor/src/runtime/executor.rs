use std::thread;

use crate::future::{Future, Waker, PollState};

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn block_on<F>(&self, future: F)
    where
        F: Future<Output = ()>,
    {
        let mut future = future;
        let waker = Waker::new(thread::current());
        loop {
            match future.poll(&waker) {
                PollState::NotReady => {
                    println!("Schedule other tasks\n");
                    thread::park();
                }

                PollState::Ready(_) => break,
            }
        }
    }
}