//! Secure key storage.

use crate::keypair::KeyPair;
use crate::error::{IdentityError, Result};
use std::path::Path;

/// Secure key storage interface.
pub trait SecureStorage {
    /// Store a keypair.
    fn store(&mut self, name: &str, keypair: &KeyPair) -> Result<()>;
    
    /// Load a keypair.
    fn load(&self, name: &str) -> Result<KeyPair>;
    
    /// Delete a keypair.
    fn delete(&mut self, name: &str) -> Result<()>;
    
    /// Check if keypair exists.
    fn exists(&self, name: &str) -> bool;
}

/// File-based storage (for development).
pub struct FileStorage {
    base_path: std::path::PathBuf,
}

impl FileStorage {
    /// Create new file storage.
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
}

impl SecureStorage for FileStorage {
    fn store(&mut self, name: &str, keypair: &KeyPair) -> Result<()> {
        let path = self.base_path.join(format!("{}.key", name));
        std::fs::write(&path, keypair.secret_bytes())
            .map_err(|e| IdentityError::StorageError(e.to_string()))?;
        Ok(())
    }

    fn load(&self, name: &str) -> Result<KeyPair> {
        let path = self.base_path.join(format!("{}.key", name));
        let bytes = std::fs::read(&path)
            .map_err(|e| IdentityError::StorageError(e.to_string()))?;
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        KeyPair::from_bytes(&arr)
    }

    fn delete(&mut self, name: &str) -> Result<()> {
        let path = self.base_path.join(format!("{}.key", name));
        std::fs::remove_file(&path)
            .map_err(|e| IdentityError::StorageError(e.to_string()))?;
        Ok(())
    }

    fn exists(&self, name: &str) -> bool {
        let path = self.base_path.join(format!("{}.key", name));
        path.exists()
    }
}
