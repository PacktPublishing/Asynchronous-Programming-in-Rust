use crate::future::Future;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock};

thread_local! {
    static TASKS: RefCell<VecDeque<Arc<Task>>> = RefCell::new(VecDeque::new());
    static CURR_ID: RefCell<usize> = RefCell::new(0);
}

pub fn spawn<F>(f: F)
where
    F: Future<Output = String> + 'static,
{
    let task = Arc::new(Task::new(f));
    TASKS.with(|tasks| tasks.borrow_mut().push_back(task));
}

fn next_id() -> usize {
    CURR_ID.with(|id| {
        let mut id = id.borrow_mut();
        *id += 1;
        *id
    })
}

pub struct Task {
    future: Arc<dyn Future<Output = String>>,
    id: usize,
}

impl Task {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = String> + 'static,
    {
        Self {
            future: Arc::new(future),
            id: next_id(),
        }
    }
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn block_on<F>(&self, future: F)
    where
        F: Future<Output = String> + 'static,
    {
        spawn(future);
        self.run();
    }

    fn run(&self) {
        todo!()
    }
}

// async fn foo() {
//     println!("Hello");
//     executor::spawn(bar(1));
//     println!("after");
// }

// async fn bar(i: i32) {
//     println!("{i}");
// }
// fn test() {
//     let executor = Executor::new();
//     let fut = foo();
//     executor.block_on(fut);
// }

// use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}}};

// use mio::{net::TcpStream, Token, Interest, Events};

// use crate::future::{Future, PollState};

// thread_local! {
//     static E: Executor = Executor::new();
// }

// static EXECUTOR: ExecWrapper = ExecWrapper;

// struct ExecWrapper;

// impl ExecWrapper {
//     pub fn set_ready(&self, id: usize) {
//         E.with(|executor| executor.set_ready(id))
//     }
// }

// pub struct Executor {
//     ready: Arc<Mutex<Vec<Box<dyn Future<Output = String>>>>>,
//     pending: Arc<Mutex<HashMap<usize, Box<dyn Future<Output = String>>>>>,
//     task_counter: AtomicUsize,
// }

// impl Executor {
//     pub fn new() -> Self {
//         Self {
//             ready: Arc::new(Mutex::new(vec![])),
//             pending: Arc::new(Mutex::new(HashMap::new())),
//             task_counter: AtomicUsize::new(0),
//         }
//     }

//     pub fn spawn<F: Future<Output = String> + 'static>(&self, task: F) {
//         self.ready.lock().map(|mut r| r.push(Box::new(task))).unwrap();
//         self.task_counter.fetch_add(1, Ordering::Relaxed);
//     }

//     pub fn run(&self) {
//         while self.task_counter.load(Ordering::Relaxed) > 0 {
//             while let Some(mut task) = self.ready.lock().map(|mut r| r.pop()).unwrap() {
//                 match task.poll() {
//                     PollState::NotReady => {
//                         // task is responsible for wakeup
//                         self.pending.lock().map(|mut p| p.insert(k, v))
//                         println!("NotReady");

//                     }

//                     PollState::Ready(_) => {
//                         self.task_counter.fetch_sub(1, Ordering::Relaxed);
//                     }
//                 }
//             }
//         }

//         // sleep on reactor.poll()
//     }

//     pub fn set_ready(&self, id: usize) {
//         let task = self.pending.lock().map(|mut p| p.remove(&id).unwrap()).unwrap();
//         self.ready.lock().map(|mut r| r.push(task)).unwrap();
//     }
// }

// pub struct Reactor {
//     poll: mio::Poll,
//     next_id: usize,
// }

// impl Reactor {
//     pub fn new() -> Self {
//         Self {
//             poll: mio::Poll::new().unwrap(),
//             next_id: 1,
//         }
//     }

//     pub fn poll(&mut self) {
//         let mut events = Events::with_capacity(10);
//         self.poll.poll(&mut events, None).unwrap();

//         for event in &events {
//             let Token(id) = event.token();
//             EXECUTOR.set_ready(id);
//         }
//     }

//     pub fn register(&mut self, source: &mut TcpStream) -> usize {
//         let id = self.next_id;
//         self.poll.registry().register(source, Token(id), Interest::READABLE).unwrap();
//         self.next_id += 1;
//         id
//     }

//     pub fn registry(&self) -> mio::Registry {
//         self.poll.registry().try_clone().unwrap()
//     }
// }
