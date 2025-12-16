pub mod trust_sync;
pub mod revocation_sync;
pub mod epoch_sync;

pub use trust_sync::TrustSync;
pub use revocation_sync::RevocationSync;
pub use epoch_sync::EpochSync;

pub struct SyncManager {
    trust: TrustSync,
    revocation: RevocationSync,
    epoch: EpochSync,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            trust: TrustSync::new(),
            revocation: RevocationSync::new(),
            epoch: EpochSync::new(),
        }
    }

    pub async fn sync_trust(&mut self) -> bool { self.trust.sync().await }
    pub async fn sync_revocations(&mut self) -> bool { self.revocation.sync().await }
    pub async fn sync_epochs(&mut self) -> bool { self.epoch.sync().await }
}

impl Default for SyncManager {
    fn default() -> Self { Self::new() }
}
