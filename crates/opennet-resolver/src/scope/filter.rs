use opennet_core::Scope;
use crate::candidates::NodeDescriptor;

pub struct ScopeFilter { scope: Scope }

impl ScopeFilter {
    pub fn new(scope: Scope) -> Self { Self { scope } }
    pub fn filter(&self, candidates: Vec<NodeDescriptor>) -> Vec<NodeDescriptor> {
        candidates.into_iter().filter(|c| c.scope.matches(&self.scope)).collect()
    }
}
