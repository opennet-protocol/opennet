use crate::revocation::object::RevocationObject;
use crate::error::Result;

pub struct QuorumValidator { threshold: f64 }

impl QuorumValidator {
    pub fn new(threshold: f64) -> Self { Self { threshold } }
    pub fn validate(&self, _obj: &RevocationObject) -> Result<bool> { Ok(true) }
}

impl Default for QuorumValidator {
    fn default() -> Self { Self::new(0.51) }
}
