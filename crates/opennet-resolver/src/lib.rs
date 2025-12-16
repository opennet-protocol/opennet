//! # OpenNet Resolver
//!
//! Resolver daemon for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod daemon;
pub mod resolution;
pub mod ranking;
pub mod scope;
pub mod candidates;
pub mod error;

pub use error::{ResolverError, Result};
