use crate::error::Result;

pub struct QuicConnection { open: bool }

impl QuicConnection {
    pub fn new() -> Self { Self { open: true } }
    pub fn is_open(&self) -> bool { self.open }
    pub async fn close(&mut self) { self.open = false; }
    pub async fn open_stream(&self) -> Result<super::stream::QuicStream> {
        Ok(super::stream::QuicStream::new(0))
    }
}

impl Default for QuicConnection {
    fn default() -> Self { Self::new() }
}
