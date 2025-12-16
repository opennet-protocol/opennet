use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVector {
    pub id: String,
    pub description: String,
    pub input: Vec<u8>,
    pub expected: super::result::ExpectedResult,
}
