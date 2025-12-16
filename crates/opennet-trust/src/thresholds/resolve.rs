//! Resolve threshold.

pub const DEFAULT_RESOLVE_THRESHOLD: f64 = 0.2;

pub fn check_resolve(weight: f64) -> bool {
    weight >= DEFAULT_RESOLVE_THRESHOLD
}
