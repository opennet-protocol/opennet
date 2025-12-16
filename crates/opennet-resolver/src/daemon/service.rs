use crate::error::Result;

pub struct ResolverService { running: bool }

impl ResolverService {
    pub fn new() -> Self { Self { running: false } }
    pub async fn start(&mut self) -> Result<()> { self.running = true; Ok(()) }
    pub async fn stop(&mut self) { self.running = false; }
}

impl Default for ResolverService {
    fn default() -> Self { Self::new() }
}
