//! # OpenNet Identity
//!
//! Identity lifecycle management for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod rotation;
pub mod error;

mod keypair;
mod node_identity;
mod announcement;
mod compromise;
mod storage;
mod watcher;

pub use keypair::{KeyPair, verify_signature};
pub use node_identity::NodeIdentity;
pub use announcement::IdentityAnnouncement;
pub use compromise::{CompromiseDetector, CompromiseEvidence};
pub use storage::{SecureStorage, FileStorage};
pub use watcher::{IdentityWatcher, IdentityEvent};
pub use error::{IdentityError, Result};
