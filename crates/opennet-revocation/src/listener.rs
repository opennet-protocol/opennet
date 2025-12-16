use opennet_core::NodeId;

#[derive(Debug, Clone)]
pub enum RevocationEvent {
    LocalRevocation { node_id: NodeId, epoch: u64 },
    RemoteRevocation { node_id: NodeId, epoch: u64, issuer_trust: f64 },
    RecoveryInitiated { node_id: NodeId },
}

pub struct RevocationListener {
    pending: Vec<RevocationEvent>,
}

impl RevocationListener {
    pub fn new() -> Self { Self { pending: Vec::new() } }
    pub fn drain_events(&mut self) -> Vec<RevocationEvent> { std::mem::take(&mut self.pending) }
}

impl Default for RevocationListener {
    fn default() -> Self { Self::new() }
}
