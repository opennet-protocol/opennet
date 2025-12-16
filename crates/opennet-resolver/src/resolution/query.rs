use opennet_core::ServiceId;
use crate::candidates::CandidateSet;

pub struct ResolutionQuery { pub service_id: ServiceId }

impl ResolutionQuery {
    pub fn new(service_id: ServiceId) -> Self { Self { service_id } }
    pub async fn execute(&self) -> CandidateSet { CandidateSet::new() }
}
