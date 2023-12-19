use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd};

use crate::ffi;

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::epoll_create(1) };
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
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        };

        // This is safe because epol_wait ensures that `res` events are assigned.
        unsafe { events.set_len(res as usize) };
        Ok(())
    }
}


pub struct Registry {
    raw_fd: i32,
}

impl Registry {

    // NB! Mio inverts this, and `source` owns the register implementation
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };

        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event)
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
            // Note! Mio logs the error but does not panic!
            let err = io::Error::last_os_error();
            println!("ERROR: {err:?}");
        }
    }
}