use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, Thread},
};

use crate::future::{Future, PollState};

type Task = Box<dyn Future<Output = String>>;

thread_local! {
    static CURRENT_EXEC: RefCell<Option<Rc<ExecutorCore>>> = RefCell::new(None);
}

fn current() -> Rc<ExecutorCore> {
    CURRENT_EXEC.with(|e| e.borrow().as_ref().expect("Executor not started").clone())
}

#[derive(Default)]
struct ExecutorCore {
    tasks: RefCell<HashMap<usize, Task>>,
    ready_queue: Arc<Mutex<Vec<usize>>>,
    next_id: Cell<usize>,
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = String> + 'static,
{
    let e = current();
    let id = e.next_id.get();
    e.tasks.borrow_mut().insert(id, Box::new(future));
    e.ready_queue.lock().map(|mut q| q.push(id)).unwrap();
    e.next_id.set(id + 1);
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        CURRENT_EXEC.set(Some(Rc::new(ExecutorCore::default())));
        Self {}
    }

    fn pop_ready(&self) -> Option<usize> {
        current().ready_queue.lock().map(|mut q| q.pop()).unwrap()
    }

    fn get_future(&self, id: usize) -> Task {
        current().tasks.borrow_mut().remove(&id).unwrap()
    }

    fn get_waker(&self, id: usize) -> Waker {
        Waker {
            id,
            thread: thread::current(),
            ready_queue: current().ready_queue.clone(),
        }
    }

    fn insert_task(&self, id: usize, task: Task) {
        current().tasks.borrow_mut().insert(id, task);
    }

    fn task_count(&self) -> usize {
        current().tasks.borrow().len()
    }

    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = String> + 'static,
    {
        spawn(future);

        loop {
            while let Some(id) = self.pop_ready() {
                let mut future = self.get_future(id);
                let waker = self.get_waker(id);

                match future.poll(&waker) {
                    PollState::NotReady => self.insert_task(id, future),
                    PollState::Ready(_) => continue,
                }
            }

            let task_count = self.task_count();
            if task_count > 0 {
                println!("{task_count} pending tasks. Sleep until notified.\n");
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
    pub fn wake(&self) {
        self.ready_queue
            .lock()
            .map(|mut q| q.push(self.id))
            .unwrap();
        self.thread.unpark();
    }
}
