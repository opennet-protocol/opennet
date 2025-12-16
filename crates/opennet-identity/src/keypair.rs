//! Ed25519 keypair management.

use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier};
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;
use opennet_core::types::{PublicKey, Signature};
use crate::error::{IdentityError, Result};

/// Ed25519 keypair wrapper.
pub struct KeyPair {
    signing_key: SigningKey,
}

impl KeyPair {
    /// Generate a new random keypair with given seed.
    pub fn generate(seed: &[u8; 32]) -> Self {
        let mut rng = ChaCha20Rng::from_seed(*seed);
        let signing_key = SigningKey::generate(&mut rng);
        Self { signing_key }
    }

    /// Create from existing secret key bytes.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes);
        Ok(Self { signing_key })
    }

    /// Get the public key.
    pub fn public_key(&self) -> PublicKey {
        let vk = self.signing_key.verifying_key();
        PublicKey::from_bytes(vk.to_bytes())
    }

    /// Get the secret key bytes (be careful!).
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Sign a message.
    pub fn sign(&self, message: &[u8]) -> Signature {
        let sig = self.signing_key.sign(message);
        Signature::from_bytes(sig.to_bytes())
    }

    /// Verify a signature with this keypair's public key.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<()> {
        let sig = ed25519_dalek::Signature::from_bytes(signature.as_bytes());
        self.signing_key.verifying_key()
            .verify(message, &sig)
            .map_err(|_| IdentityError::SignatureVerificationFailed)
    }
}

/// Verify a signature with a public key.
pub fn verify_signature(public_key: &PublicKey, message: &[u8], signature: &Signature) -> Result<()> {
    let vk = VerifyingKey::from_bytes(public_key.as_bytes())
        .map_err(|e| IdentityError::InvalidKey(e.to_string()))?;
    let sig = ed25519_dalek::Signature::from_bytes(signature.as_bytes());
    vk.verify(message, &sig)
        .map_err(|_| IdentityError::SignatureVerificationFailed)
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        // Zeroize would be applied here in production
    }
}
