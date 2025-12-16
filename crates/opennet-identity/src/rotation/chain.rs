//! Epoch chain verification.

use opennet_core::EpochId;
use std::collections::BTreeMap;
use crate::error::{IdentityError, Result};

/// Tracks epoch chain for a node.
pub struct EpochChain {
    /// Epoch transitions: old_epoch -> new_epoch.
    transitions: BTreeMap<EpochId, EpochId>,
    /// Current head epoch.
    head: EpochId,
}

impl EpochChain {
    /// Create new chain starting at epoch 1.
    pub fn new() -> Self {
        Self {
            transitions: BTreeMap::new(),
            head: 1,
        }
    }

    /// Add a transition.
    pub fn add_transition(&mut self, from: EpochId, to: EpochId) -> Result<()> {
        if to != from + 1 {
            return Err(IdentityError::EpochChainBroken(
                "epochs must be consecutive".into()
            ));
        }
        if from != self.head {
            return Err(IdentityError::EpochChainBroken(
                "transition must be from current head".into()
            ));
        }
        self.transitions.insert(from, to);
        self.head = to;
        Ok(())
    }

    /// Get current head epoch.
    pub fn head(&self) -> EpochId {
        self.head
    }

    /// Check if epoch exists in chain.
    pub fn contains(&self, epoch: EpochId) -> bool {
        epoch == 1 || self.transitions.values().any(|&e| e == epoch)
    }

    /// Validate chain integrity.
    pub fn validate(&self) -> Result<()> {
        let mut current = 1;
        while let Some(&next) = self.transitions.get(&current) {
            if next != current + 1 {
                return Err(IdentityError::EpochChainBroken(
                    format!("gap at epoch {}", current)
                ));
            }
            current = next;
        }
        Ok(())
    }
}

impl Default for EpochChain {
    fn default() -> Self {
        Self::new()
    }
}
