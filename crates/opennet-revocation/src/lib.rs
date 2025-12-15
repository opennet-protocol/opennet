//! # OpenNet Revocation
//!
//! Revocation and recovery mechanisms for the OpenNet protocol.
//!
//! ## Threat Model
//!
//! The revocation mechanism addresses:
//! - Private key compromise
//! - Forced key disclosure
//! - Long-term silent impersonation
//! - Trust laundering through identity reset
//!
//! ## Revocation Triggers
//!
//! A node MUST initiate revocation when:
//! - Private key is exposed
//! - Unauthorized signatures are detected
//! - Trust graph decay threshold is exceeded
//!
//! ## Quorum Validation
//!
//! Revocation is valid when:
//! - Node self-signs, OR
//! - A quorum of trusted peers co-sign
//!
//! Quorum size is determined by trust weight, not raw count.
//!
//! ## Recovery Process
//!
//! Recovery is optional and proceeds:
//! 1. Node generates new keypair
//! 2. Recovery declaration references revoked epoch
//! 3. Trusted peers co-sign recovery
//! 4. Trust graph links new epoch with reduced weight
//!
//! **Important**: Recovery does NOT restore full historical trust.
//!
//! ## RFC Reference
//!
//! This crate implements: Revocation & Recovery RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod revocation;
pub mod quorum;
pub mod recovery;
pub mod error;

mod listener;

pub use listener::RevocationListener;
pub use error::{RevocationError, Result};
