use super::NodeDescriptor;

#[derive(Debug, Default)]
pub struct CandidateSet { pub nodes: Vec<NodeDescriptor> }

impl CandidateSet {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
    pub fn add(&mut self, node: NodeDescriptor) { self.nodes.push(node); }
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}
