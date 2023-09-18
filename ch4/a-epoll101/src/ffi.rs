
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLONESHOT: i32 = 0x40000000;

#[link(name = "c")]
extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    // Token to identify event
    pub(crate) epoll_data: usize,
}
