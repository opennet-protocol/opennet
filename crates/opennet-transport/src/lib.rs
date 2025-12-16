//! # OpenNet Transport
//!
//! Transport layer for the OpenNet protocol (QUIC/TCP).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod quic;
pub mod tcp;
pub mod session;
pub mod admission;
pub mod handshake;
pub mod error;

mod backpressure;

pub use backpressure::BackpressureController;
pub use error::{TransportError, Result};
