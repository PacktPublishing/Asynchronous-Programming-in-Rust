```rust
use std::net::TcpStream;

use io::Result;

pub struct Queue {}

impl Queue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn submitter(&self) -> Submit {
        todo!()
    }

    pub fn wait(&self, timeout: Option<i32>) -> Result<()> {
        todo!()
    }
}

pub struct Submit {}

impl Submit {
    pub fn read(stream: &TcpStream) -> Result<()> {
        todo!()
    }
}

```