//! Complete node identity management.

use opennet_core::{NodeId, Epoch, EpochId};
use opennet_core::types::PublicKey;
use crate::keypair::KeyPair;
use crate::error::{IdentityError, Result};

/// Complete node identity.
pub struct NodeIdentity {
    /// Immutable NodeId.
    node_id: NodeId,
    /// Current keypair.
    keypair: KeyPair,
    /// Current epoch.
    epoch: Epoch,
    /// Epoch history (for verification).
    epoch_history: Vec<EpochId>,
}

impl NodeIdentity {
    /// Create a new identity from a keypair.
    pub fn new(keypair: KeyPair, start_time: u64) -> Self {
        let public_key = keypair.public_key();
        let node_id = NodeId::from_public_key(public_key.as_bytes());
        
        let mut key_hash = [0u8; 32];
        key_hash.copy_from_slice(node_id.as_bytes());
        
        let epoch = Epoch::new(1, start_time, key_hash);
        
        Self {
            node_id,
            keypair,
            epoch,
            epoch_history: vec![1],
        }
    }

    /// Get the NodeId.
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Get the current public key.
    pub fn public_key(&self) -> PublicKey {
        self.keypair.public_key()
    }

    /// Get the current epoch.
    pub fn epoch(&self) -> &Epoch {
        &self.epoch
    }

    /// Get current epoch ID.
    pub fn epoch_id(&self) -> EpochId {
        self.epoch.id
    }

    /// Sign a message.
    pub fn sign(&self, message: &[u8]) -> opennet_core::types::Signature {
        self.keypair.sign(message)
    }

    /// Rotate to a new keypair.
    pub fn rotate(&mut self, new_keypair: KeyPair, rotation_time: u64) -> Result<()> {
        let new_epoch_id = self.epoch.id + 1;
        let new_public_key = new_keypair.public_key();
        
        let mut key_hash = [0u8; 32];
        key_hash.copy_from_slice(new_public_key.as_bytes());
        
        let new_epoch = Epoch::new(new_epoch_id, rotation_time, key_hash);
        
        // Validate transition
        Epoch::validate_transition(&self.epoch, &new_epoch)
            .map_err(|e| IdentityError::RotationFailed(e.to_string()))?;
        
        self.keypair = new_keypair;
        self.epoch = new_epoch;
        self.epoch_history.push(new_epoch_id);
        
        Ok(())
    }

    /// Check if epoch is in history.
    pub fn has_epoch(&self, epoch_id: EpochId) -> bool {
        self.epoch_history.contains(&epoch_id)
    }
}
