use crate::future::{Future, PollState};
use mio::{Events, Poll, Registry};
use std::sync::OnceLock;

static REGISTRY: OnceLock<Registry> = OnceLock::new();

pub fn registry() -> &'static Registry {
    REGISTRY.get().expect("Called outside a runtime context")
}

pub struct Runtime {
    poll: Poll,
    // tasks: Vec<Box<dyn Future<Output = String>>>,
}

impl Runtime {
    pub fn new() -> Self {
        let poll = Poll::new().unwrap();
        let registry = poll.registry().try_clone().unwrap();
        REGISTRY.set(registry).unwrap();
        Self {
            poll,
            // tasks: vec![],
        }
    }

    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = String>,
    {
        let mut future = future;
        loop {
            match future.poll() {
                PollState::NotReady => {
                    println!("Schedule other tasks\n");
                    let mut events = Events::with_capacity(100);
                    self.poll.poll(&mut events, None).unwrap();
                }

                PollState::Ready(_) => break,
            }
        }
    }

    // pub fn block_on_fast<F>(&mut self, future: F)
    // where
    //     F: Future<Output = String> + 'static,
    // {
    //     // fast path optimization, assume future is ready
    //     let mut future = future;
    //     match future.poll() {
    //         PollState::NotReady => {
    //             println!("Schedule other tasks\n");
    //             self.tasks.push(Box::new(future));
    //         }
    //         PollState::Ready(_) => return,
    //     }
    //     // it was not ready so, slow path it is
    //     loop {
    //         let mut future = self.tasks.pop().unwrap();
    //         match future.poll() {
    //             PollState::NotReady => {
    //                 println!("Schedule other tasks\n");
    //                 self.tasks.push(future);
    //                 let mut events = Events::with_capacity(100);
    //                 self.poll.poll(&mut events, None).unwrap();
    //             }
    //             PollState::Ready(_) => break,
    //         }
    //     }
    // }
}
