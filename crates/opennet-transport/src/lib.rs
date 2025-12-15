//! # OpenNet Transport
//!
//! Transport layer for the OpenNet protocol (QUIC/TCP).
//!
//! ## Supported Protocols
//!
//! - **QUIC**: Preferred protocol (UDP-based, low latency, built-in encryption)
//! - **TCP**: Fallback (used when QUIC is not supported)
//!
//! ## Session Binding
//!
//! Each transport session is bound to:
//!
//! ```text
//! (NodeId, Epoch)
//! ```
//!
//! This ensures:
//! - Automatic session termination on epoch change
//! - Prevention of communication with revoked keys
//! - Protection against replay attacks
//!
//! ## Transport Responsibilities
//!
//! The Transport layer MUST enforce:
//! - Accept only Resolver-approved nodes
//! - Reject connections below ConnectThreshold
//! - Bind sessions to (NodeId, Epoch) pair
//! - Terminate sessions on trust/epoch invalidation
//! - Apply backpressure and flow control
//!
//! ## RFC Reference
//!
//! This crate implements: Transport RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod quic;
pub mod tcp;
pub mod session;
pub mod admission;
pub mod handshake;
pub mod error;

mod backpressure;

pub use backpressure::BackpressureController;
pub use error::{TransportError, Result};
