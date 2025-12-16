//! Relay threshold.

pub const DEFAULT_RELAY_THRESHOLD: f64 = 0.4;

pub fn check_relay(weight: f64) -> bool {
    weight >= DEFAULT_RELAY_THRESHOLD
}
