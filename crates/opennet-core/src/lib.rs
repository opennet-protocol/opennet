//! # OpenNet Core
//!
//! Core types and primitives for the OpenNet protocol.
//!
//! ## Overview
//!
//! This crate provides the foundational types used throughout the OpenNet protocol:
//!
//! - [`NodeId`] - Unique, immutable node identifier (SHA-256 hash of public key)
//! - [`ServiceId`] - Cryptographic service identifier
//! - [`Epoch`] - Bounded validity period for key pairs
//! - [`Scope`] - Logical network partitions (geographic, privacy, test)
//! - [`OpenNetUri`] - Parser for `open://` URI scheme
//!
//! ## Design Principles
//!
//! - **No floating-point**: All arithmetic uses fixed-point types
//! - **Deterministic**: Same inputs always produce same outputs
//! - **Minimal dependencies**: Only essential cryptographic primitives
//!
//! ## RFC Reference
//!
//! This crate implements: OpenNet Core Protocol RFC
//!
//! ## Example
//!
//! ```rust,ignore
//! use opennet_core::{NodeId, Epoch, OpenNetUri};
//!
//! // Parse an open:// URI
//! let uri = OpenNetUri::parse("open://chat.opennet/room/123")?;
//!
//! // Create a NodeId from public key
//! let node_id = NodeId::from_public_key(&public_key);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod types;

mod node_id;
mod service_id;
mod epoch;
mod scope;
mod uri;
mod constants;

pub use node_id::NodeId;
pub use service_id::ServiceId;
pub use epoch::{Epoch, EpochId};
pub use scope::Scope;
pub use uri::{OpenNetUri, ServiceUri};
pub use constants::*;
pub use error::{CoreError, Result};

/// Protocol version
pub const PROTOCOL_VERSION: u16 = 1;

/// Wire format version
pub const WIRE_VERSION: u16 = 1;
