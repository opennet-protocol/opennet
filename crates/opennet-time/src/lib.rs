//! # OpenNet Time
//!
//! Time and epoch semantics for the OpenNet protocol.
//!
//! ## Overview
//!
//! Provides consistent time semantics without centralized time authorities:
//!
//! - **Network Median Time (NMT)**: Median of peer time samples
//! - **Drift Tolerance**: Maximum acceptable drift ±5 minutes
//! - **Replay Protection**: 120 second window with nonce tracking
//!
//! ## Time Sources
//!
//! OpenNet nodes MAY use:
//! - System Clock
//! - Monotonic Clock
//! - Network Median Time (NMT)
//!
//! No node should rely solely on a single external time source.
//!
//! ## NMT Calculation
//!
//! ```text
//! NMT = median(peer_time_samples)
//! ```
//!
//! Samples are collected during authenticated handshakes.
//! Outliers beyond drift tolerance are discarded.
//!
//! ## Replay Protection
//!
//! Each message includes:
//! - `epoch_id`: Message's epoch
//! - `message_timestamp`: Message timestamp
//! - `nonce`: Unique number
//!
//! ## RFC Reference
//!
//! This crate implements: Time & Epoch Semantics RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod nmt;
pub mod epoch;
pub mod drift;
pub mod replay;
pub mod error;

mod monitor;
mod clock;

pub use monitor::EpochMonitor;
pub use clock::MonotonicClock;
pub use error::{TimeError, Result};

/// Default replay window in seconds
pub const DEFAULT_REPLAY_WINDOW: u64 = 120;

/// Default drift tolerance in seconds
pub const DEFAULT_DRIFT_TOLERANCE: u64 = 300; // 5 minutes
