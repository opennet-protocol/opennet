use super::config::QuicConfig;
use crate::error::Result;

pub struct QuicClient { config: QuicConfig }

impl QuicClient {
    pub fn new(config: QuicConfig) -> Self { Self { config } }
    pub async fn connect(&self, _addr: &str) -> Result<super::connection::QuicConnection> {
        Ok(super::connection::QuicConnection::new())
    }
}
