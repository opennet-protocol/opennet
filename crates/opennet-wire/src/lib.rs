//! # OpenNet Wire Format
//!
//! CBOR/TLV canonical encoding for the OpenNet protocol.
//!
//! ## Overview
//!
//! This crate provides wire-level binary encoding:
//!
//! - **CBOR**: For structured payloads (canonical encoding)
//! - **TLV**: For low-level framing and extensions
//!
//! ## Hybrid Model
//!
//! ```text
//! [ TLV(Frame) ] → contains → [ CBOR(Payload) ]
//! ```
//!
//! ## Canonical CBOR Rules
//!
//! All CBOR payloads MUST enforce:
//! 1. Use definite-length encoding
//! 2. Use ONLY integer map keys
//! 3. Sort map keys in ascending order
//! 4. Reject duplicate keys
//! 5. Use minimal integer encoding
//! 6. Disallow floating-point values
//!
//! ## TLV Frame Structure
//!
//! ```text
//! TLV := [ Type (uint16) | Length (uint32) | Value (bytes) ]
//! ```
//!
//! ## RFC Reference
//!
//! This crate implements: CBOR & TLV Schemas RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cbor;
pub mod tlv;
pub mod frame;
pub mod messages;
pub mod error;

mod validation;

pub use validation::validate_frame;
pub use error::{WireError, Result};
