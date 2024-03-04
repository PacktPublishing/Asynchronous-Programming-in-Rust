//! # FIXES:
//! The number is identical to the number in the GitHub issue tracker
//!
//! ## FIX ISSUE #5
//! See: https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/5
//! Readers reported wrong results when running the example on ARM64 instruction set
//! (aarch64). The reason turned out to be that the `Event` struct is only `repr(packed)`
//! on `x86-64` systems due to backwards compatibility. Fixed by conditionally
//! compiling the #[repr(packed)] attribute.

pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C)]
// FIX #5
#[cfg_attr(target_arch = "x86_64", repr(packed))]
pub struct Event {
    pub(crate) events: u32,
    // Token to identify event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
