use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    threads: Vec<Thread>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut threads = vec![];
        for _ in 0..size {
            let (tx, rx) = channel();
            let is_ready = Arc::new(AtomicBool::new(true));
            let is_ready_clone = is_ready.clone();
            let handle = thread::spawn(move || handle_tasks(rx, is_ready_clone));
            threads.push(Thread {
                handle,
                tx,
                is_ready,
            });
        }

        Self { threads }
    }

    pub fn spawn<F>(&self, op: F) -> bool
    where
        F: FnOnce() -> () + Send + 'static,
    {
        match self.get_available() {
            Some(idx) => {
                self.threads[idx].tx.send(Task::Do(Box::new(op))).expect("Channel failed");
                true
            }
            None => false,
        }
    }

    fn get_available(&self) -> Option<usize> {
        for (idx, thread) in self.threads.iter().enumerate() {
            let is_ready = thread.is_ready.compare_exchange(
                true,
                false,
                Ordering::AcqRel,
                Ordering::Acquire,
            );

            if let Ok(true) = is_ready {
                return Some(idx);
            }
        }

        None
    }

    pub fn join(self) {
        for thread in &self.threads {
            thread.tx.send(Task::Close).expect("Channel failure");
        }

        for thread in self.threads {
            thread.handle.join().expect("Thread failure");
        }
    }
}

fn handle_tasks(rx: Receiver<Task>, is_ready: Arc<AtomicBool>) {
    for task in rx.recv() {
        match task {
            Task::Close => break,
            Task::Do(f) => {
                is_ready.store(false, Ordering::Release);
                f();
                is_ready.store(true, Ordering::Release);
            }
        }
    }
}

struct Thread {
    handle: JoinHandle<()>,
    tx: Sender<Task>,
    is_ready: Arc<AtomicBool>,
}

enum Task {
    Close,
    Do(Box<dyn FnOnce() -> () + Send + 'static>),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn flag_changes() {
        let pool = ThreadPool::new(4);
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        let res = pool.spawn(move || {
            flag_clone.store(true, Ordering::Release);
        });
        let mut count = 0;
        while !flag.load(Ordering::Acquire) {
            count += 1;
            if count == 100 {
                panic!("Flag didn't change in 100 tries");
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
