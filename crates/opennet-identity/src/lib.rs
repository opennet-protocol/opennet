//! # OpenNet Identity
//!
//! Identity lifecycle management for the OpenNet protocol.
//!
//! ## Overview
//!
//! OpenNet separates identity continuity from key material:
//!
//! - **NodeId**: Immutable, permanent throughout lifetime
//! - **Key Pair**: Rotatable, Ed25519 mandatory
//!
//! ## Epoch Concept
//!
//! An epoch represents a continuous validity interval of a key pair:
//!
//! - `epoch_id`: Monotonically increasing number (u64)
//! - `start_time`: Epoch start time
//! - `max_duration`: Maximum validity duration
//! - `key_material_hash`: Hash of associated key material
//!
//! ## Key Rotation
//!
//! Key rotation preserves identity while updating cryptographic material:
//!
//! 1. New key pair is generated
//! 2. KeyRotation message is created
//! 3. Message is signed with old private key
//! 4. Broadcast to network
//!
//! ## RFC Reference
//!
//! This crate implements: Identity Lifecycle RFC

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

pub use keypair::KeyPair;
pub use node_identity::NodeIdentity;
pub use announcement::IdentityAnnouncement;
pub use compromise::CompromiseDetector;
pub use storage::SecureStorage;
pub use watcher::IdentityWatcher;
pub use error::{IdentityError, Result};
