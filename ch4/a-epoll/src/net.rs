use std::{io::{Result, Write}, os::fd::AsRawFd};

use crate::ffi;

pub struct TcpStream {
    inner: std::net::TcpStream,
}

impl TcpStream {
    pub fn write(addr: &str, data: &[u8]) -> Result<Self> {
        // Shortcut: The connect and write is blocking!
        let mut ts = std::net::TcpStream::connect(addr)?;
        ts.write(data)?;
        ts.set_nonblocking(true)?;
        Ok(Self { inner: ts })
    }
    
    /// If size is 0 there is no data to read. If size is < buffer.len()
    /// There is no more data to read. If size == buffer.len() there might
    /// be more data to read.
    pub fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        let fd = self.as_raw_fd();
        let buf = buffer.as_mut_ptr();
        let count = buffer.len();
        let res = unsafe { ffi::read(fd, buf, count)};
        
        if res < 0 {
            // NB! see: https://man7.org/linux/man-pages/man2/read.2.html
            // Several reported error codes must be expected and handled
            // like EAGAIN, EWOULDBLOCK
            return Err(std::io::Error::last_os_error());
        };
        
        Ok(res as usize)
    }
    
    pub fn as_raw_fd(&self) -> i32 {
        self.inner.as_raw_fd()
    }
}