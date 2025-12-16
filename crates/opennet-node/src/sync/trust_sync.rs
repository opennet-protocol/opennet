pub struct TrustSync { synced: bool }

impl TrustSync {
    pub fn new() -> Self { Self { synced: false } }
    pub async fn sync(&mut self) -> bool { self.synced = true; true }
}

impl Default for TrustSync {
    fn default() -> Self { Self::new() }
}
