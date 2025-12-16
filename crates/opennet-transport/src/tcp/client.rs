use crate::error::Result;

pub struct TcpClient;

impl TcpClient {
    pub fn new() -> Self { Self }
    pub async fn connect(&self, _addr: &str) -> Result<super::connection::TcpConnection> {
        Ok(super::connection::TcpConnection::new())
    }
}

impl Default for TcpClient {
    fn default() -> Self { Self::new() }
}
