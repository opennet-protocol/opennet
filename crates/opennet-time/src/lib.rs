//! # OpenNet Time
//!
//! Time and epoch semantics for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod nmt;
pub mod epoch;
pub mod drift;
pub mod replay;
pub mod error;

mod monitor;
mod clock;

pub use monitor::{EpochMonitor, TimeEvent};
pub use clock::{MonotonicClock, SystemClock, MockClock};
pub use error::{TimeError, Result};

/// Default replay window in seconds.
pub const DEFAULT_REPLAY_WINDOW: u64 = 120;

/// Default drift tolerance in seconds.
pub const DEFAULT_DRIFT_TOLERANCE: u64 = 300;
