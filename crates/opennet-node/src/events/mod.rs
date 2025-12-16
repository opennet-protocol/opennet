//! Event queue and processing.

pub mod queue;
pub mod priority;
pub mod producer;

pub use queue::EventQueue;
pub use priority::EventPriority;
