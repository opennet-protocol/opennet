use opennet_core::NodeId;
use opennet_trust::weight::TrustWeight;

#[derive(Debug)]
pub struct TrustExchange {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: TrustWeight,
}
