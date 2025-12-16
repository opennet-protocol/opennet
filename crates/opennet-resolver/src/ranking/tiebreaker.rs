use opennet_core::NodeId;
use sha2::{Sha256, Digest};

/// Deterministic tiebreaker: Hash(NodeId || EpochId || ContextSalt)
pub fn tiebreak(a: &NodeId, b: &NodeId, salt: &[u8]) -> std::cmp::Ordering {
    let hash_a = hash_for_tiebreak(a, salt);
    let hash_b = hash_for_tiebreak(b, salt);
    hash_a.cmp(&hash_b)
}

fn hash_for_tiebreak(node_id: &NodeId, salt: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(node_id.as_bytes());
    hasher.update(salt);
    let result = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&result);
    arr
}
