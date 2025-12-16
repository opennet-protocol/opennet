//! Request pipeline: Resolver → Trust → Transport.

use opennet_core::OpenNetUri;
use super::{resolver::ResolverIntegration, trust::TrustIntegration, transport::TransportIntegration};

/// Integrated request pipeline.
pub struct RequestPipeline {
    resolver: ResolverIntegration,
    trust: TrustIntegration,
    transport: TransportIntegration,
}

impl RequestPipeline {
    pub fn new() -> Self {
        Self {
            resolver: ResolverIntegration::new(),
            trust: TrustIntegration::new(),
            transport: TransportIntegration::new(),
        }
    }

    /// Process a request through the pipeline.
    pub async fn process(&mut self, uri: &OpenNetUri) -> Result<(), String> {
        // 1. Resolve URI to candidates
        let candidates = self.resolver.resolve(uri).await;
        
        if candidates.is_empty() {
            return Err("no candidates found".into());
        }

        // 2. Filter by trust threshold
        let trusted: Vec<_> = candidates.nodes.iter()
            .filter(|c| self.trust.get_trust(&c.node_id) >= 0.3)
            .collect();

        if trusted.is_empty() {
            return Err("no trusted candidates".into());
        }

        // 3. Connect to best candidate
        let best = &trusted[0];
        if !self.transport.connect(&best.node_id).await {
            return Err("connection failed".into());
        }

        Ok(())
    }
}

impl Default for RequestPipeline {
    fn default() -> Self { Self::new() }
}
