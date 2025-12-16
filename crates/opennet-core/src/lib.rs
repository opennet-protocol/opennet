//! # OpenNet Core
//!
//! Core types and primitives for the OpenNet protocol.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod types;

mod node_id;
mod service_id;
mod epoch;
mod scope;
mod uri;
mod constants;

pub use node_id::NodeId;
pub use service_id::ServiceId;
pub use epoch::{Epoch, EpochId};
pub use scope::{Scope, PrivacyLevel};
pub use uri::{OpenNetUri, ServiceUri};
pub use constants::*;
pub use error::{CoreError, Result};
