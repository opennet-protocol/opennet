use opennet_core::NodeId;
use opennet_node::fsm::NodeState;

pub struct TestNode {
    pub node_id: NodeId,
    pub state: NodeState,
}

impl TestNode {
    pub fn new() -> Self {
        Self {
            node_id: NodeId::from_bytes([0u8; 32]),
            state: NodeState::Bootstrap,
        }
    }

    pub fn with_state(mut self, state: NodeState) -> Self {
        self.state = state;
        self
    }
}

impl Default for TestNode {
    fn default() -> Self { Self::new() }
}

pub struct TestNodeBuilder {
    node: TestNode,
}

impl TestNodeBuilder {
    pub fn new() -> Self { Self { node: TestNode::new() } }
    pub fn state(mut self, state: NodeState) -> Self { self.node.state = state; self }
    pub fn build(self) -> TestNode { self.node }
}

impl Default for TestNodeBuilder {
    fn default() -> Self { Self::new() }
}
