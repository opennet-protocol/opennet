//! # OpenNet Resolver
//!
//! Resolver daemon for the OpenNet protocol.
//!
//! ## Overview
//!
//! The OpenNet Resolver Daemon (ORD) replaces traditional DNS resolution:
//!
//! ```text
//! Application → URI Handler → IPC → Resolver Daemon → OpenNet P2P Network
//! ```
//!
//! ## Resolution Workflow
//!
//! 1. URI Parsing and Validation
//! 2. Domain and ServiceID Normalization
//! 3. OpenNet Network Query
//! 4. Scope Filtering
//! 5. Deterministic Ranking
//! 6. Result Return
//!
//! ## Node Ranking
//!
//! ```text
//! RankScore = TW × ScopeAffinity × DiversityFactor
//! ```
//!
//! Ranking MUST be stable and deterministic. For tie-breaking:
//!
//! ```text
//! Hash(NodeId || EpochId || ContextSalt)
//! ```
//!
//! ## OS Integration
//!
//! | Platform | Integration |
//! |----------|-------------|
//! | Linux | systemd service, UNIX domain socket |
//! | Windows | Background service, named pipe IPC |
//! | macOS | launchd service, XPC IPC |
//!
//! ## RFC Reference
//!
//! This crate implements: Resolver Daemon RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod daemon;
pub mod resolution;
pub mod ranking;
pub mod scope;
pub mod candidates;
pub mod error;

pub use error::{ResolverError, Result};
