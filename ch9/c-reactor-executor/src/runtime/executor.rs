use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, Thread}, cell::RefCell,
};

use crate::future::{Future, PollState};

thread_local! {
    static CURRENT_EXEC: ExecutorCore = ExecutorCore::new();
}

struct ExecutorCore {
    tasks: RefCell<HashMap<usize, Box<dyn Future<Output = String>>>>,
    ready_queue: Arc<Mutex<Vec<usize>>>,
    next_id:  RefCell<usize>,
}

impl ExecutorCore {
    fn new() -> Self {
        Self {
            tasks: RefCell::new(HashMap::new()),
            ready_queue: Arc::new(Mutex::new(vec![])),
            next_id: RefCell::new(1),
        }
    }

    fn get_id(&self) -> usize {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    fn spawn<F>(&self, future: F)
    where
    F: Future<Output = String> + 'static,
    {
        let id = self.get_id();
        self.tasks.borrow_mut().insert(id, Box::new(future));
        self.ready_queue.lock().map(|mut q| q.push(id));
    }

    fn pop_ready(&self) -> Option<usize> {
        self.ready_queue.lock().map(|mut q| q.pop()).unwrap()
    }
}

pub fn spawn<F>(future: F) -> JoinHandle
where
    F: Future<Output = String> + 'static,
{
    CURRENT_EXEC.with(|e| e.spawn(future));
    JoinHandle {}
}

pub struct Executor;

pub struct JoinHandle {}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = String> + 'static + Send + Sync + Sized,
    {
        spawn(future);

        //let mut future = future;
        // let waker = Waker::new(thread::current());
        loop {
            while let Some(id) = CURRENT_EXEC.with(|q| q.pop_ready()) {
                let mut future = CURRENT_EXEC.with(|q| q.tasks.borrow_mut().remove(&id).unwrap());
                let waker = Waker::new(id, thread::current(), CURRENT_EXEC.with(|q| q.ready_queue.clone()));
                match future.poll(&waker) {
                    PollState::NotReady => {
                        CURRENT_EXEC.with(|q| q.tasks.borrow_mut().insert(id, future));
                    },

                    PollState::Ready(_) => (),
                }
            }
            if CURRENT_EXEC.with(|q| q.tasks.borrow().len() > 0){
                println!("Schedule other tasks\n");
                thread::park();
            } else {
                println!("All tasks are finished");
                break;
            }
        }
    }
}

#[derive(Clone)]
pub struct Waker {
    thread: Thread,
    id: usize,
    ready_queue: Arc<Mutex<Vec<usize>>>,
}

impl Waker {
    pub fn new(id: usize, thread: Thread, ready_queue: Arc<Mutex<Vec<usize>>>) -> Self
    {
        Self { id, thread, ready_queue}
    }

    pub fn wake(&self) {
        self.ready_queue
            .lock()
            .map(|mut q| q.push(self.id)).unwrap();
        self.thread.unpark();
    }
}
