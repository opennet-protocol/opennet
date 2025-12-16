use crate::error::Result;

pub struct TcpServer { running: bool }

impl TcpServer {
    pub fn new() -> Self { Self { running: false } }
    pub async fn bind(&mut self, _addr: &str) -> Result<()> { self.running = true; Ok(()) }
}

impl Default for TcpServer {
    fn default() -> Self { Self::new() }
}
