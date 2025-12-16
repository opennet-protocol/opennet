//! # OpenNet Trust
//!
//! Trust graph and weight dynamics for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod graph;
pub mod weight;
pub mod decay;
pub mod thresholds;
pub mod events;
pub mod sybil;
pub mod error;

mod evaluator;

pub use evaluator::{TrustEvaluator, TrustEvent};
pub use error::{TrustError, Result};

/// Default initial trust weight.
pub const INITIAL_TRUST: f64 = 0.01;

/// Default decay constant.
pub const DEFAULT_LAMBDA: f64 = 0.05;
