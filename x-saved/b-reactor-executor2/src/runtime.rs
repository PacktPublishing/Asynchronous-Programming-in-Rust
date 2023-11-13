use mio::{net::TcpStream, Events, Interest, Poll, Registry, Token};

use crate::future::{Future, PollState, Waker};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, OnceLock,
    },
    thread::{self, Thread},
};

static REACTOR: OnceLock<Reactor> = OnceLock::new();

pub fn reactor() -> &'static Reactor {
    REACTOR.get().expect("Called outside an executor context")
}

pub struct Reactor {
    wakers: Arc<Mutex<HashMap<usize, Waker>>>,
    registry: Registry,
    next_id: AtomicUsize,
}

impl Reactor {
    pub fn start() {
        let wakers = Arc::new(Mutex::new(HashMap::new()));
        let wakers_clone = wakers.clone();

        let mut poll = Poll::new().unwrap();
        let registry = poll.registry().try_clone().unwrap();
        let next_id = AtomicUsize::new(1);
        let reactor = Self {
            wakers,
            registry,
            next_id,
        };
        REACTOR.set(reactor).ok().unwrap();

        thread::spawn(move || {
            let mut events = Events::with_capacity(100);
            loop {
                poll.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    let Token(id) = e.token();
                    wakers_clone
                        .lock()
                        .map(|w| {
                            // if we removed it from the list we don't want to respond to
                            // notifications anymore
                            if let Some(waker) = w.get(&id) {
                                waker.wake();
                            }
                        })
                        .unwrap();
                }
            }
        });
    }

    pub fn register(&self, stream: &mut TcpStream, interest: Interest, waker: Waker, id: usize) {
        let is_new = self
            .wakers
            .lock()
            .map(|mut w| w.insert(id, waker).is_none())
            .unwrap();

        if is_new {
            self.registry.register(stream, Token(id), interest).unwrap();
        }
    }

    pub fn deregister(&self, stream: &mut TcpStream, id: usize) {
        self.registry.deregister(stream).unwrap();
        self.wakers.lock().map(|mut w| w.remove(&id)).unwrap();
    }

    pub fn next_id(&self) -> usize {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

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
