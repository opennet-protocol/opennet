use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpectedResult {
    Accept,
    Drop,
    Reject,
    Abort,
}
