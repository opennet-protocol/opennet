//! Negative trust events (penalties).

use opennet_core::NodeId;
use crate::weight::TrustWeight;

#[derive(Debug, Clone)]
pub struct NegativeEvent {
    pub node_id: NodeId,
    pub penalty: TrustWeight,
    pub reason: PenaltyReason,
}

#[derive(Debug, Clone)]
pub enum PenaltyReason {
    ProtocolViolation,
    InvalidSignature,
    TimeoutExceeded,
    MalformedMessage,
}

/// Default penalty factor.
pub const DEFAULT_PENALTY_FACTOR: f64 = 0.2;
