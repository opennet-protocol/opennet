//! # OpenNet Wire Format
//!
//! CBOR/TLV canonical encoding for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cbor;
pub mod tlv;
pub mod frame;
pub mod messages;
pub mod error;

mod validation;

pub use validation::{validate_frame, validate_message_type};
pub use error::{WireError, Result};
