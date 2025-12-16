use crate::error::Result;

pub struct QuicStream { id: u64 }

impl QuicStream {
    pub fn new(id: u64) -> Self { Self { id } }
    pub fn id(&self) -> u64 { self.id }
    pub async fn send(&self, _data: &[u8]) -> Result<()> { Ok(()) }
    pub async fn recv(&self) -> Result<Vec<u8>> { Ok(Vec::new()) }
}
