use std::collections::HashMap;

use crate::future::Future;

pub struct Executor {
    ready: Arc<Mutex<HashMap<usize, dyn Future<Output = String>>>>,
    pending: Arc<Mutex<HashMap<usize, dyn Future<Output = String>>>>,
    task_counter: usize,
}

impl Executor {
    pub fn run(&mut self) {
        while task_counter > 0 {
            for task in ready {
                match future.poll() {
                    PollState::NotReady => {
                        println!("NotReady");
                    }

                    PollState::Ready(s) => break s,
                }
            }
        }
    }
}
