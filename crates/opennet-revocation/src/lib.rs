//! # OpenNet Revocation
//!
//! Revocation and recovery mechanisms for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod revocation;
pub mod quorum;
pub mod recovery;
pub mod error;

mod listener;

pub use listener::{RevocationListener, RevocationEvent};
pub use error::{RevocationError, Result};
