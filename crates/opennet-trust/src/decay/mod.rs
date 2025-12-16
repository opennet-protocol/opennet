//! Trust decay functions.

pub mod exponential;
pub mod parameters;

pub use exponential::exponential_decay;
pub use parameters::DecayParameters;
