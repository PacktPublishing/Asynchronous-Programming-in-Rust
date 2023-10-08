use std::{io::{Result, Write, Read}, os::fd::AsRawFd};

pub struct TcpStream {
    inner: std::net::TcpStream,
}

impl TcpStream {

    pub fn from_std(stream: std::net::TcpStream) -> Self {
        Self { inner: stream }
    }

    pub fn as_raw_fd(&self) -> i32 {
        self.inner.as_raw_fd()
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}
impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}