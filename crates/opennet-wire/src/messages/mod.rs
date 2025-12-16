//! Protocol message definitions.

pub mod node_hello;
pub mod node_welcome;
pub mod service_join;
pub mod service_leave;
pub mod stream_open;
pub mod stream_data;
pub mod stream_close;
pub mod revocation;

pub use node_hello::NodeHello;
pub use node_welcome::NodeWelcome;
pub use service_join::ServiceJoin;
pub use service_leave::ServiceLeave;
pub use stream_open::StreamOpen;
pub use stream_data::StreamData;
pub use stream_close::StreamClose;
pub use revocation::RevocationMessage;

/// Message type identifiers.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    NodeHello = 0x0001,
    NodeWelcome = 0x0002,
    ServiceJoin = 0x0010,
    ServiceLeave = 0x0011,
    StreamOpen = 0x0020,
    StreamData = 0x0021,
    StreamClose = 0x0022,
    Revocation = 0x0030,
}
