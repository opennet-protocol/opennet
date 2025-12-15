//! # OpenNet Trust
//!
//! Trust graph and weight dynamics for the OpenNet protocol.
//!
//! ## Mathematical Model
//!
//! The trust graph is a directed weighted graph:
//!
//! ```text
//! G = (V, E) where V = nodes, E = trust edges
//! ```
//!
//! ## Trust Weight
//!
//! Trust weight (TW) is in the range [0.0, 1.0]:
//! - 0.0 = No trust
//! - 1.0 = Maximum trust
//! - New nodes start with ε = 0.01
//!
//! ## Exponential Decay
//!
//! Trust decays exponentially over time:
//!
//! ```text
//! decay(t) = e^(−λt)  where λ = 0.05 (default)
//! ```
//!
//! ## Thresholds
//!
//! | Threshold | Default | Description |
//! |-----------|---------|-------------|
//! | ResolveThreshold | 0.2 | Minimum to include in candidate list |
//! | ConnectThreshold | 0.3 | Minimum for connection attempt |
//! | RelayThreshold | 0.4 | Minimum for traffic relay |
//!
//! ## Sybil Resistance
//!
//! - Cost to raise TW grows exponentially
//! - Trust cannot be rapidly accumulated
//! - Identity splitting resets accumulated trust
//!
//! ## RFC Reference
//!
//! This crate implements: Trust Weight Dynamics RFC

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

pub use evaluator::TrustEvaluator;
pub use error::{TrustError, Result};

/// Default initial trust weight for new nodes
pub const INITIAL_TRUST: f64 = 0.01;

/// Default decay constant (λ)
pub const DEFAULT_LAMBDA: f64 = 0.05;
