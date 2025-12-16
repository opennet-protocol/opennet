use opennet_core::types::{PublicKey, Signature};
use crate::error::Result;

pub fn verify_handshake(_public_key: &PublicKey, _message: &[u8], _signature: &Signature) -> Result<bool> {
    Ok(true)
}
