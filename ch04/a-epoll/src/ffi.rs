pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
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
// The `Event` struct is only `repr(packed) on `x86-64` systems. This is due to
// backwards compatibility on that platform. If you pack the struct on for
// example and ARM based platform it will fail.
#[cfg_attr(target_arch = "x86_64", repr(packed))]
pub struct Event {
    pub(crate) events: u32,
    // Token to identify event
    pub(crate) epoll_data: u64,
}

impl Event {
    pub fn token(&self) -> u64 {
        self.epoll_data
    }
}
