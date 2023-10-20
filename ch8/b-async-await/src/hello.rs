use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::{Future, PollState};
use crate::http::Http;




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


// struct Coroutine {
//     state: State,
// }

// enum State {
//     Start,
//     Wait1(Box<dyn Future<Output = String>>),
//     Wait2(Box<dyn Future<Output = String>>),
//     Resolved,
// }

// impl Coroutine {
//     fn new() -> Self {
//         Self {
//             state: State::Start,
//         }
//     }
// }

// impl Future for Coroutine {
//     type Output = ();

//     fn poll(&mut self) -> PollState<Self::Output> {
//         match self.state {
//             State::Start => {
//                 println!("Program starting");
//                 let fut = Box::new(Http::get("/1000/HelloWorld1"));
//                 self.state = State::Wait1(fut);
//                 PollState::NotReady
//             }
            
//             State::Wait1(ref mut fut) => {
//                 match fut.poll() {
//                     PollState::Ready(txt) => {
//                         println!("{txt}");
//                         let fut2 = Box::new(Http::get("/600/HelloWorld2"));
//                         self.state = State::Wait2(fut2);
//                         PollState::NotReady
//                     }
                    
//                     PollState::NotReady => PollState::NotReady,
//                 }
//             }
            
//             State::Wait2(ref mut fut2) => {
//                 match fut2.poll() {
//                     PollState::Ready(txt2) => {
//                         println!("{txt2}");
//                         self.state = State::Resolved;
//                         PollState::Ready(())
//                     }
                    
//                     PollState::NotReady => PollState::NotReady,
//                 }
//             }
            
//             State::Resolved => panic!("Polled a resolved future"),
//         }
//     }
// }


// fn async_main() -> impl Future<Output = ()> {
//     Coroutine::new()
// }


    enum Steps {
        Start,
        Step1(Box<dyn Future<Output = String>>),
Step2(Box<dyn Future<Output = String>>),
Resolved,
}


    struct Coroutine {
        steps: Steps,
    }
    