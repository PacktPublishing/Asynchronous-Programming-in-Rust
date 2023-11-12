pub use executor::{spawn, Waker, Executor};
pub use reactor::reactor;

mod executor;
mod reactor;

pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}
