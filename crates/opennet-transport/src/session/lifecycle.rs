#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState { Connecting, Active, Closing, Closed }
