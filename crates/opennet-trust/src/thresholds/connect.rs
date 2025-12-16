//! Connect threshold.

pub const DEFAULT_CONNECT_THRESHOLD: f64 = 0.3;

pub fn check_connect(weight: f64) -> bool {
    weight >= DEFAULT_CONNECT_THRESHOLD
}
