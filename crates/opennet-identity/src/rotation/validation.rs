//! Key rotation validation.

use super::request::RotationRequest;
use crate::keypair::verify_signature;
use crate::error::{IdentityError, Result};
use opennet_core::types::PublicKey;

/// Validate a rotation request.
pub fn validate_rotation(
    request: &RotationRequest,
    current_public_key: &PublicKey,
) -> Result<()> {
    // Check epoch increment
    if request.new_epoch != request.current_epoch + 1 {
        return Err(IdentityError::RotationFailed(
            "epoch must increment by exactly 1".into()
        ));
    }

    // Verify old key signature
    let old_signing_bytes = request.old_key_signing_bytes();
    verify_signature(current_public_key, &old_signing_bytes, &request.old_key_signature)?;

    // Verify new key signature
    let new_signing_bytes = request.new_key_signing_bytes();
    verify_signature(&request.new_public_key, &new_signing_bytes, &request.new_key_signature)?;

    Ok(())
}
