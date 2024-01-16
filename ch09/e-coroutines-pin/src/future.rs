// NEW
use crate::runtime::Waker;
use std::pin::Pin;

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, waker: &Waker) -> PollState<Self::Output>;
}

pub enum PollState<T> {
    Ready(T),
    NotReady,
}

// =======================================================
// Optional - we won't use this going forward
// =======================================================
#[allow(dead_code)]
pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, Box::pin(f))).collect();
    JoinAll {
        futures,
        finished_count: 0,
    }
}

pub struct JoinAll<F: Future> {
    futures: Vec<(bool, Pin<Box<F>>)>,
    finished_count: usize,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, waker: &Waker) -> PollState<Self::Output> {
        let Self {
            futures,
            finished_count,
        } = &mut *self;
        for (finished, fut) in futures.iter_mut() {
            if *finished {
                continue;
            }

            match fut.as_mut().poll(waker) {
                PollState::Ready(_) => {
                    *finished = true;
                    *finished_count += 1;
                }

                PollState::NotReady => continue,
            }
        }

        if self.finished_count == self.futures.len() {
            PollState::Ready(String::new())
        } else {
            PollState::NotReady
        }
    }
}
