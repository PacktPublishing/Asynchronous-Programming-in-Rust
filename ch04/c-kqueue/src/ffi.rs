/// FFI Functions for interacting with kqueue.
/// Details on constants, functions, and parameters can be found on the
/// kqueue(2) manual page.
/// https://man.freebsd.org/cgi/man.cgi?query=kevent&sektion=2&manpath=macOS+15.7

pub const EVFILT_READ: i16 = -1; // Analagous to the EPOLLIN flag in the original.
pub const EV_ADD: u16 = 0x1; // Add the event to the queue, analagous to EPOLL_CTL_ADD in the original.
pub const EV_CLEAR: u16 = 0x20; // Reset the state after the user retrieves it, similar to EPOLLET.

#[derive(Debug)]
#[repr(C)]
// Timespec struct is a required input to the kevent syscall.
// Used for setting timeout values.
pub struct Timespec {
    pub tv_sec: isize,
    pub tv_nsec: usize,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
// We use a Kevent for kqueue instead of the Event struct for epoll.
// Consistent with Issue #5 in the repository this is not a packed struct on ARM Macs.
pub struct Kevent {
    pub ident: u64,
    pub filter: i16,
    pub flags: u16,
    pub fflags: u32,
    pub data: isize,
    pub udata: u64,
}

impl Kevent {
    pub fn token(&self) -> Option<usize> {
        // udata stores the token, analagous to the epoll_data field in the original.
        Some(self.udata as usize)
    }
}

#[link(name = "c")]
extern "C" {
    // Creates the event queue, analagous to epoll_create in the original.
    pub fn kqueue() -> i32;
    // Registers events with the queue and also used to check for pending events.
    // This performs the functionality of epoll_ctl and epoll_wait in the original,
    // depending on the parameters. See poll.rs for usage.
    pub fn kevent(
        kq: i32,
        changelist: *const Kevent,
        nchanges: i32,
        eventlist: *mut Kevent,
        nevents: i32,
        timeout: *const Timespec,
    ) -> i32;
    // Closes the event queue, analagous to close in the original.
    pub fn close(d: i32) -> i32;
}
