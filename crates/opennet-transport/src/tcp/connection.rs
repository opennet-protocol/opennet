pub struct TcpConnection { open: bool }

impl TcpConnection {
    pub fn new() -> Self { Self { open: true } }
    pub fn is_open(&self) -> bool { self.open }
    pub async fn close(&mut self) { self.open = false; }
}

impl Default for TcpConnection {
    fn default() -> Self { Self::new() }
}
