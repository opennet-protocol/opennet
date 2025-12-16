//! Protocol constants for OpenNet.
//!
//! RFC: OpenNet Core Protocol

/// Protocol version number.
pub const PROTOCOL_VERSION: u16 = 1;

/// Wire format version.
pub const WIRE_VERSION: u16 = 1;

/// NodeId byte length (SHA-256 hash).
pub const NODE_ID_LEN: usize = 32;

/// ServiceId byte length.
pub const SERVICE_ID_LEN: usize = 32;

/// Maximum epoch duration in seconds (default: 90 days).
pub const MAX_EPOCH_DURATION: u64 = 90 * 24 * 60 * 60;

/// Default initial trust weight for new nodes.
pub const INITIAL_TRUST_WEIGHT: &str = "0.01";

/// Trust decay constant (λ).
pub const TRUST_DECAY_LAMBDA: &str = "0.05";

/// Resolve threshold (minimum trust to include in candidates).
pub const THRESHOLD_RESOLVE: &str = "0.2";

/// Connect threshold (minimum trust for connection).
pub const THRESHOLD_CONNECT: &str = "0.3";

/// Relay threshold (minimum trust for traffic relay).
pub const THRESHOLD_RELAY: &str = "0.4";

/// Default replay window in seconds.
pub const REPLAY_WINDOW_SECS: u64 = 120;

/// Default drift tolerance in seconds (±5 minutes).
pub const DRIFT_TOLERANCE_SECS: u64 = 300;

/// URI scheme for OpenNet.
pub const URI_SCHEME_OPEN: &str = "open";

/// URI scheme for services.
pub const URI_SCHEME_SERVICE: &str = "service";
