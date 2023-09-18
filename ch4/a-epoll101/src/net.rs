use std::{io::{Result, Write}, os::fd::AsRawFd};

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
    
    pub fn read(&self, buffer: &mut [u8]) -> Result<()> {
        todo!()
    }
    
    pub fn as_raw_fd(&self) -> i32 {
        self.inner.as_raw_fd()
    }
}