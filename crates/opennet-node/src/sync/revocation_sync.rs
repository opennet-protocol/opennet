pub struct RevocationSync { synced: bool }

impl RevocationSync {
    pub fn new() -> Self { Self { synced: false } }
    pub async fn sync(&mut self) -> bool { self.synced = true; true }
}

impl Default for RevocationSync {
    fn default() -> Self { Self::new() }
}
