pub use self::executor::Executor;
pub use reactor::reactor;


mod reactor;
mod executor;


pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}


