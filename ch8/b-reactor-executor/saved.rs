
enum MyOneStageFut<'a> {
    Start,
    Wait1(
        Box<dyn Future<Output = String>>,
        String,
        Option<Formatter<'a>>,
    ),
    Wait2(
        Box<dyn Future<Output = String>>,
        String,
        Option<Formatter<'a>>,
    ),
    Resolved,
}

impl<'a> MyOneStageFut<'a> {
    fn new() -> Self {
        Self::Start
    }
}

impl<'a> Future for MyOneStageFut<'a> {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        let mut this = std::mem::replace(self, Self::Resolved);
        match this {
            Self::Start => {
                println!("Program starting");
                let buffer = String::new();
                let fut = Box::new(Http::get("/1000/HelloWorld1"));
                *self = MyOneStageFut::Wait1(fut, buffer, None);
                if let Self::Wait1(_, ref mut buff, ref mut formatter) = self {
                    let buff: *mut String = buff;
                    // We rely on the pointer to buff has the same address or else
                    // we dereference random data -> segfault -> UB -> BAD!
                    let buff = unsafe { &mut *buff };
                    *formatter = Some(Formatter::new(buff));
                }
                PollState::NotReady
            }

            Self::Wait1(mut fut, buffer, mut formatter) => {
                let txt = match fut.poll() {
                    PollState::Ready(s) => s,
                    PollState::NotReady => {
                        *self = MyOneStageFut::Wait1(fut, buffer, None);
                        if let Self::Wait1(_, ref mut buff, ref mut formatter) = self {
                            let buff: *mut String = buff;
                            let buff = unsafe { &mut *buff };
                            *formatter = Some(Formatter::new(buff));
                        }
                        return PollState::NotReady;
                    }
                };
                formatter.as_mut().unwrap().format(txt);

                let fut2 = Box::new(Http::get("/600/HelloWorld2"));
                *self = Self::Wait2(fut2, buffer, formatter);
                PollState::NotReady
            }

            Self::Wait2(ref mut fut, ref mut buffer, ref mut formatter) => {
                let txt2 = match fut.poll() {
                    PollState::Ready(s) => s,
                    PollState::NotReady => {
                        *self = this;
                        return PollState::NotReady;
                    }
                };
                formatter.as_mut().unwrap().format(txt2);
                *self = Self::Resolved;
                PollState::Ready(())
            }

            Self::Resolved => panic!("Polled a resolved future"),
        }
    }
}

pub trait Future {
    type Output;
    fn poll(&mut self) -> PollState<Self::Output>;
}

pub enum PollState<T> {
    Ready(T),
    NotReady,
}

fn async_main() -> impl Future<Output = ()> {
    MyOneStageFut::new()
}

fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(s) => break s,
        }
    }
}

mod runtime {
    use super::*;

    pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
        let futures = futures.into_iter().map(|f| (false, f)).collect();
        JoinAll {
            futures,
            finished_count: 0,
        }
    }

    pub struct JoinAll<F: Future> {
        futures: Vec<(bool, F)>,
        finished_count: usize,
    }

    impl<F: Future> Future for JoinAll<F> {
        type Output = ();

        fn poll(&mut self) -> PollState<Self::Output> {
            for (finished, fut) in self.futures.iter_mut() {
                if *finished {
                    continue;
                }

                match fut.poll() {
                    PollState::Ready(_) => {
                        *finished = true;
                        self.finished_count += 1;
                    }

                    PollState::NotReady => continue,
                }
            }

            if self.finished_count == self.futures.len() {
                PollState::Ready(())
            } else {
                PollState::NotReady
            }
        }
    }
}
