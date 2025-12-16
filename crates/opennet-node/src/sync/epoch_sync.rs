pub struct EpochSync { synced: bool }

impl EpochSync {
    pub fn new() -> Self { Self { synced: false } }
    pub async fn sync(&mut self) -> bool { self.synced = true; true }
}

impl Default for EpochSync {
    fn default() -> Self { Self::new() }
}
