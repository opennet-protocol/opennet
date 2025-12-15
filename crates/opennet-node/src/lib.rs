//! # OpenNet Node
//!
//! Full-node implementation for the OpenNet protocol.
//!
//! ## Node States (FSM)
//!
//! An OpenNet full node MUST be in exactly one state:
//!
//! ```text
//! BOOTSTRAP → SYNCING → ACTIVE ↔ DEGRADED → QUARANTINED
//! ```
//!
//! | State | Description |
//! |-------|-------------|
//! | BOOTSTRAP | Initial startup, no trusted peers |
//! | SYNCING | Synchronizing trust graph, epochs, revocations |
//! | ACTIVE | Fully operational |
//! | DEGRADED | Reduced capacity, temporary failures |
//! | QUARANTINED | Isolated, revoked or unsafe |
//!
//! ## State Transitions
//!
//! | From | To | Condition |
//! |------|-----|-----------|
//! | BOOTSTRAP | SYNCING | ≥1 valid peer discovered |
//! | SYNCING | ACTIVE | Synchronization completed |
//! | ACTIVE | DEGRADED | Trust < T_warn |
//! | DEGRADED | ACTIVE | Trust ≥ T_ok |
//! | ACTIVE | QUARANTINED | Trust < T_min |
//! | DEGRADED | QUARANTINED | SecurityViolation event |
//!
//! **Critical**: Any transition not defined MUST panic.
//!
//! ## FSM Events
//!
//! - `PeerDiscovered`
//! - `SyncCompleted`
//! - `TrustBelowWarn`
//! - `TrustRecovered`
//! - `TrustCritical`
//! - `SecurityViolation`
//!
//! ## RFC Reference
//!
//! This crate implements:
//! - Full-Node State Machine RFC
//! - Full-Node Behavior & Integration RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod fsm;
pub mod states;
pub mod events;
pub mod integration;
pub mod peer;
pub mod sync;
pub mod error;

mod config;

pub use config::NodeConfig;
pub use error::{NodeError, Result};
