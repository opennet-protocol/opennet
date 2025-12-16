//! Full-Node State Machine (FSM).
//!
//! RFC: Full-Node State Machine RFC

pub mod state;
pub mod event;
pub mod transition;
pub mod table;
pub mod invariants;

pub use state::NodeState;
pub use event::StateEvent;
pub use transition::Transition;
