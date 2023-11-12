use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, OnceLock},
    thread::{self, Thread},
};

use crate::future::{Future, PollState};

type Tasks = Arc<Mutex<VecDeque<Arc<dyn Future<Output = String> + 'static + Send + Sync>>>>;

static READY_QUEUE: OnceLock<Tasks> = OnceLock::new();

fn ready_queue<'a>() -> &'a Tasks {
    READY_QUEUE.get().expect("ready_queue missing")
}

pub struct Executor {
    outstanding: usize,
}

pub fn spawn<F>(future: F) -> JoinHandle
where
    F: Future<Output = String> + 'static + Send + Sync,
{
    ready_queue().lock().map(|queue| {
        queue.push_back(Arc::new(future));
    });

    JoinHandle {  }
}

pub struct JoinHandle {}

impl Executor {
    pub fn new() -> Self {
        READY_QUEUE
            .set(Arc::new(Mutex::new(VecDeque::new())))
            .ok().unwrap();
        Self { outstanding: 0 }
    }

    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = String> + 'static + Send + Sync + Sized,
    {
        spawn(future);

        //let mut future = future;
        // let waker = Waker::new(thread::current());
        loop {
            while let Some(mut f) = ready_queue().lock().map(|mut q| q.pop_front()).unwrap() {
                let waker = Waker::new(f.clone(), thread::current());
                match future.poll(&waker) {
                    PollState::NotReady => self.outstanding += 1,

                    PollState::Ready(_) => self.outstanding -= 1,
                }
            }
            if self.outstanding > 0 {
                println!("Schedule other tasks\n");
                thread::park();
            } else {
                println!("All tasks are finished");
            }
        }
    }
}

#[derive(Clone)]
pub struct Waker {
    thread: Thread,
    task: Arc<dyn Future<Output = String> + 'static + Send + Sync>,
}

impl Waker {
    pub fn new<F>(task: Arc<F>, thread: Thread) -> Self
    where
        F: Future<Output = String> + 'static + Send + Sync + Sized,
    {
        Self { task: task, thread }
    }

    pub fn wake(&self) {
        ready_queue()
            .lock()
            .map(|mut q| q.push_back(self.task.clone()));
        self.thread.unpark();
    }
}


#[derive(Clone)]
pub struct TreadWaker(Thread);

impl TreadWaker {
    pub fn new(thread: Thread) -> Self {
        Self(thread)
    }
    fn wake(&self) {
        self.0.unpark();
    }
}
