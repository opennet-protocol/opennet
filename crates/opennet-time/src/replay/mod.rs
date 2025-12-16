//! Replay protection.

pub mod window;
pub mod nonce;

pub use window::ReplayWindow;
pub use nonce::NonceTracker;
