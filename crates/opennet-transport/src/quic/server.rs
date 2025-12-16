use super::config::QuicConfig;
use crate::error::Result;

pub struct QuicServer { config: QuicConfig, running: bool }

impl QuicServer {
    pub fn new(config: QuicConfig) -> Self { Self { config, running: false } }
    pub async fn bind(&mut self, _addr: &str) -> Result<()> { self.running = true; Ok(()) }
    pub async fn accept(&self) -> Result<super::connection::QuicConnection> {
        Ok(super::connection::QuicConnection::new())
    }
}
