use thiserror::Error;
pub type Result<T> = std::result::Result<T, TransportError>;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("session invalid")]
    SessionInvalid,
    #[error("trust too low")]
    TrustTooLow,
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
