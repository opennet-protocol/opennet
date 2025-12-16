//! Trust thresholds.

pub mod resolve;
pub mod connect;
pub mod relay;

/// All thresholds.
pub struct Thresholds {
    pub resolve: f64,
    pub connect: f64,
    pub relay: f64,
    pub warn: f64,
    pub critical: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            resolve: 0.2,
            connect: 0.3,
            relay: 0.4,
            warn: 0.15,
            critical: 0.05,
        }
    }
}
