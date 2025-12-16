//! # OpenNet Node
//!
//! Full-node implementation for the OpenNet protocol.

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
