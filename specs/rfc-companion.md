# OpenNet Companion RFCs

## Starting from Section 16 – Extended Specifications

Category: Standards Track
Status: Draft

---

## 16. OpenNet Wire Format RFC

### 16.1 Scope and Purpose

This document defines the wire-level representation of OpenNet protocol messages. It specifies binary framing, encoding formats, and cryptographic signature placement rules. Compliance with this section is REQUIRED for interoperability between independent implementations.

---

### 16.2 Binary Framing Format

All OpenNet protocol messages MUST be encapsulated in binary frames.

Each frame consists of the following ordered fields:

1. Frame Length (uint32)
2. Protocol Version (uint16)
3. Message Type (uint16)
4. Flags (uint16)
5. Payload Length (uint32)
6. Payload (variable length)

Frames MUST be length-prefixed. Streaming transports MUST support frame reassembly. Frames with invalid lengths or versions MUST be discarded.

---

### 16.3 Message Encoding Formats

Nodes MUST support at least one encoding format and MUST advertise supported formats during handshake.

#### 16.3.1 TLV Encoding

- Type: uint16
- Length: uint32
- Value: opaque

TLV encoding is RECOMMENDED for low-level and embedded nodes.

#### 16.3.2 CBOR Encoding

CBOR encoding SHOULD be supported by all general-purpose implementations. Canonical CBOR MUST be used for signed payloads.

#### 16.3.3 Protobuf Encoding

Protobuf MAY be used as an optional encoding. Protobuf payloads MUST be encapsulated within OpenNet frames and signed identically to other encodings.

---

### 16.4 Signature Placement Rules

Every OpenNet message MUST include a cryptographic signature.

- Signatures MUST cover all header fields and the payload
- Signatures MUST be embedded, not detached
- Signature verification MUST precede message processing

Messages failing verification MUST be rejected.

---

## 17. OpenNet Resolver Behavior RFC

### 17.1 Resolver Responsibilities

Resolvers operate entirely on the client side and MUST make independent decisions without relying on centralized signals.

Resolvers MUST:
- Resolve domains via gossip
- Validate ServiceIDs
- Select nodes deterministically

---

### 17.2 Deterministic Node Ranking Algorithm

Resolvers MUST apply deterministic ranking based on local metrics:

- Round-trip latency
- Recent availability history
- Trust score
- Route depth

Random selection MUST NOT be used except as a final tie-breaker.

---

### 17.3 Scope Conflict Resolution

When multiple scopes apply:

1. Explicit user-defined scope MUST be honored
2. Resolver-local policy MUST be applied
3. Automatic fallback MUST be deterministic

User interaction SHOULD be avoided.

---

### 17.4 Cache Invalidation Rules

Resolvers MAY cache resolution data. All cache entries MUST:

- Include a TTL
- Be invalidated on signature mismatch
- Be purged on topology change events

Persistent caching MUST be bounded.

---

## 18. OpenNet Security Extensions RFC

### 18.1 Trust Graph Model

Trust is modeled as a directed weighted graph:

- Nodes gain trust via successful interactions
- Trust decays exponentially over time
- Transitive trust MUST be capped

Trust computation MUST be local.

---

### 18.2 Anti-Eclipse Protections

Nodes MUST defend against eclipse attacks by:

- Maintaining peer diversity
- Enforcing peer rotation
- Limiting trust concentration
- Detecting routing monocultures

---

### 18.3 Post-Quantum Cryptography

OpenNet MUST support cryptographic agility.

- Signature algorithms MUST be replaceable
- Hybrid classical + post-quantum schemes SHOULD be supported
- Protocol versioning MUST allow cryptographic transitions

---

## 19. OpenNet Governance and Evolution RFC

### 19.1 Version Negotiation

Nodes MUST negotiate protocol versions during handshake.

- Minor versions MUST be backward compatible
- Major versions MAY introduce breaking changes

---

### 19.2 Backward Compatibility Guarantees

- Older nodes MUST NOT be forcibly disconnected
- Feature negotiation MUST be explicit
- Deprecated features MUST have extended deprecation periods

---

### 19.3 No-Authority Upgrade Model

OpenNet has no central governance authority.

- No entity controls upgrades
- Adoption occurs organically
- Multiple protocol versions MAY coexist

---

## 20. Reference Implementation Specification

### 20.1 Mandatory APIs

Implementations SHOULD expose APIs for:

- Node lifecycle control
- Service registration
- Resolver queries
- Stream management
- Trust evaluation

---

### 20.2 State Machines

Implementations MUST define explicit state machines for:

- Peer connections
- Service membership
- Stream lifecycle
- Trust state updates

State transitions MUST be deterministic and testable.

---

### 20.3 Compliance and Test Vectors

A compliant implementation MUST pass:

- Message decoding tests
- Signature verification tests
- Resolver determinism tests
- Forwarding behavior tests
- Failure recovery tests

Test vectors MUST be public and reproducible.

---

## 21. Final Notes

This document extends the core OpenNet RFC by defining wire-level behavior, resolver logic, security extensions, governance rules, and implementation requirements. Together, these specifications enable independent, interoperable, and future-proof OpenNet deployments.
