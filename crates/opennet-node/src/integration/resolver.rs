//! Resolver integration.

use opennet_core::OpenNetUri;
use opennet_resolver::candidates::CandidateSet;

pub struct ResolverIntegration;

impl ResolverIntegration {
    pub fn new() -> Self { Self }
    
    pub async fn resolve(&self, _uri: &OpenNetUri) -> CandidateSet {
        CandidateSet::new()
    }
}

impl Default for ResolverIntegration {
    fn default() -> Self { Self::new() }
}
