//! Positive trust events.

use opennet_core::NodeId;

#[derive(Debug, Clone)]
pub enum PositiveEvent {
    SuccessfulInteraction { node_id: NodeId },
    ValidMessage { node_id: NodeId },
    ServiceProvided { node_id: NodeId },
}
