use super::format::TestVector;
use std::path::Path;

pub struct VectorLoader;

impl VectorLoader {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Vec<TestVector> {
        let _ = path;
        Vec::new()
    }

    pub fn load_cbor_vectors() -> Vec<TestVector> { Vec::new() }
    pub fn load_trust_vectors() -> Vec<TestVector> { Vec::new() }
    pub fn load_fsm_vectors() -> Vec<TestVector> { Vec::new() }
}
