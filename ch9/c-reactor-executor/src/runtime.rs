pub use executor::{Executor, Waker, spawn};
pub use reactor::reactor;


mod reactor;
mod executor;


pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}


