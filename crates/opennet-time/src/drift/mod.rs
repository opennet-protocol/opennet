//! Drift detection and tolerance.

pub mod detector;
pub mod tolerance;

pub use detector::DriftDetector;
pub use tolerance::DriftTolerance;
