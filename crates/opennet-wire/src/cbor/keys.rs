//! CBOR integer key registry.
//!
//! All CBOR maps use integer keys for compactness.

/// CBOR map keys for protocol messages.
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CborKey {
    // Common fields (0-9)
    Version = 0,
    Type = 1,
    Timestamp = 2,
    NodeId = 3,
    Signature = 4,
    EpochId = 5,
    Nonce = 6,

    // Identity fields (10-19)
    PublicKey = 10,
    KeyHash = 11,
    PreviousEpoch = 12,
    RotationProof = 13,

    // Trust fields (20-29)
    TrustWeight = 20,
    EdgeSource = 21,
    EdgeTarget = 22,
    EdgeCreated = 23,
    DecayLambda = 24,

    // Service fields (30-39)
    ServiceId = 30,
    Domain = 31,
    Scope = 32,
    Path = 33,

    // Revocation fields (40-49)
    RevokedEpoch = 40,
    ReasonCode = 41,
    QuorumSignatures = 42,

    // Transport fields (50-59)
    SessionId = 50,
    StreamId = 51,
    SequenceNum = 52,
    Payload = 53,
}

impl CborKey {
    /// Get the integer value.
    pub fn value(self) -> i64 {
        self as i64
    }

    /// Try to parse from integer.
    pub fn from_int(v: i64) -> Option<Self> {
        match v {
            0 => Some(Self::Version),
            1 => Some(Self::Type),
            2 => Some(Self::Timestamp),
            3 => Some(Self::NodeId),
            4 => Some(Self::Signature),
            5 => Some(Self::EpochId),
            6 => Some(Self::Nonce),
            10 => Some(Self::PublicKey),
            11 => Some(Self::KeyHash),
            12 => Some(Self::PreviousEpoch),
            13 => Some(Self::RotationProof),
            20 => Some(Self::TrustWeight),
            21 => Some(Self::EdgeSource),
            22 => Some(Self::EdgeTarget),
            23 => Some(Self::EdgeCreated),
            24 => Some(Self::DecayLambda),
            30 => Some(Self::ServiceId),
            31 => Some(Self::Domain),
            32 => Some(Self::Scope),
            33 => Some(Self::Path),
            40 => Some(Self::RevokedEpoch),
            41 => Some(Self::ReasonCode),
            42 => Some(Self::QuorumSignatures),
            50 => Some(Self::SessionId),
            51 => Some(Self::StreamId),
            52 => Some(Self::SequenceNum),
            53 => Some(Self::Payload),
            _ => None,
        }
    }
}
