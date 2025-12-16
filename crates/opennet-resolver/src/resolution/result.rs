use crate::candidates::NodeDescriptor;

#[derive(Debug)]
pub struct ResolutionResult {
    pub candidates: Vec<NodeDescriptor>,
    pub from_cache: bool,
}
