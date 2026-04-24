use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

use crate::ffi;

type Events = Vec<ffi::Kevent>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::kqueue() };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Makes a blocking call to the OS parking the calling thread. It will wake up
    /// when one or more events we've registered interest in have occurred or
    /// the timeout duration has elapsed, whichever occurs first.
    ///
    /// # Note
    /// If the number of events returned is 0, the wakeup was due to an elapsed
    /// timeout
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.raw_fd;

        // Timeout differs in kqueue from epoll in that it is a timespec struct
        // Instead of sending -1 for no timeout we send the null pointer.
        let timeout = timeout.map(|t| ffi::Timespec {
            tv_sec: t as isize / 1000,
            tv_nsec: (t as usize % 1000) * 1_000_000,
        });

        let timeout_ptr = match timeout {
            Some(ref t) => t as *const ffi::Timespec,
            None => std::ptr::null(),
        };

        let max_events = events.capacity() as i32;

        // To poll with kevent we need to call kevent
        // with a null changelist, and 0 changes.
        // Critically we provide an out pointer to the
        // eventlist where kevent will store the events
        // for which we're being notified.
        let res = unsafe {
            ffi::kevent(
                fd,
                std::ptr::null(),
                0,
                events.as_mut_ptr(),
                max_events,
                timeout_ptr,
            )
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        };

        // This is safe because kevent ensures that `res` events are assigned.
        unsafe { events.set_len(res as usize) };
        Ok(())
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let flags = interests as u16;

        let event = ffi::Kevent {
            ident: source.as_raw_fd() as u64,
            filter: ffi::EVFILT_READ,
            flags,
            fflags: 0,
            data: 0,
            udata: token as u64,
        };

        // To register an event we need to call kevent
        // with a Kevent struct, 1 change and null eventlist
        // and timeout.
        // This is analagous to the epoll_ctl call in the original.
        let res = unsafe {
            ffi::kevent(
                self.raw_fd,
                &event,
                1,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            )
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };

        if res < 0 {
            let err = io::Error::last_os_error();
            println!("ERROR: {err:?}");
        }
    }
}
