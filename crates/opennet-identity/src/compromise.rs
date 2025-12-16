//! Key compromise detection.

use opennet_core::NodeId;
use opennet_core::types::Signature;
use std::collections::BTreeMap;

/// Compromise detector tracks potential key misuse.
pub struct CompromiseDetector {
    /// Seen message hashes per node.
    seen_messages: BTreeMap<NodeId, Vec<[u8; 32]>>,
    /// Duplicate signature detection.
    seen_signatures: BTreeMap<NodeId, Vec<Signature>>,
}

impl CompromiseDetector {
    /// Create new detector.
    pub fn new() -> Self {
        Self {
            seen_messages: BTreeMap::new(),
            seen_signatures: BTreeMap::new(),
        }
    }

    /// Record a message from a node.
    pub fn record_message(&mut self, node_id: NodeId, message_hash: [u8; 32]) {
        self.seen_messages
            .entry(node_id)
            .or_default()
            .push(message_hash);
    }

    /// Record a signature from a node.
    pub fn record_signature(&mut self, node_id: NodeId, signature: Signature) {
        self.seen_signatures
            .entry(node_id)
            .or_default()
            .push(signature);
    }

    /// Check for duplicate signatures (potential key sharing).
    pub fn check_duplicate_signatures(&self, node_id: &NodeId) -> Option<CompromiseEvidence> {
        if let Some(sigs) = self.seen_signatures.get(node_id) {
            // Simple check - in production would be more sophisticated
            let mut seen = std::collections::HashSet::new();
            for sig in sigs {
                if !seen.insert(sig.as_bytes()) {
                    return Some(CompromiseEvidence::DuplicateSignature);
                }
            }
        }
        None
    }
}

impl Default for CompromiseDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Evidence of potential compromise.
#[derive(Debug, Clone)]
pub enum CompromiseEvidence {
    /// Same signature used twice.
    DuplicateSignature,
    /// Impossible timing (messages from different locations).
    ImpossibleTiming,
    /// Conflicting epoch claims.
    ConflictingEpoch,
}
