# OpenNet Rust Reference Node RFC

## Executable Specification for OpenNet Nodes

Category: Informational / Experimental
Status: Draft

---

## Abstract

This document defines the **OpenNet Rust Reference Node**, a minimal but fully functional implementation of the OpenNet protocol suite. The reference node acts as an *executable specification*: any compliant OpenNet implementation MUST be behaviorally equivalent to this document. This RFC intentionally evolves faster than core protocol documents and is maintained separately.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Purpose and Scope

The Rust Reference Node serves four primary purposes:

1. Provide a canonical implementation for developers
2. Validate protocol correctness
3. Act as a compliance baseline
4. Enable rapid experimentation without protocol forks

This document does **not** redefine protocol semantics; it concretely implements them.

---

## 3. Design Principles

### 3.1 Minimalism

The node MUST implement only what is required by the OpenNet RFC set.

### 3.2 Determinism

Given identical inputs, the node MUST produce identical outputs.

### 3.3 Auditability

The codebase MUST be readable, small, and dependency-minimized.

### 3.4 Safety

The implementation MUST use Rust’s memory safety guarantees and avoid `unsafe` unless strictly required.

---

## 4. High-Level Architecture

```
┌────────────┐
│ Application│
└─────┬──────┘
      │ API
┌─────▼──────┐
│ OpenNet Node│
├────────────┤
│ State Mach.│
│ Wire Layer │
│ Crypto     │
│ Resolver   │
│ Trust Graph│
└─────┬──────┘
      │
┌─────▼──────┐
│ Network I/O│
└────────────┘
```

---

## 5. Node Lifecycle State Machine

### 5.1 States

```
INIT
  ↓
KEYGEN
  ↓
HANDSHAKE
  ↓
DISCOVERY
  ↓
ACTIVE
  ↓
SHUTDOWN
```

### 5.2 Transition Rules

- Transitions MUST be explicit
- Invalid transitions MUST abort the peer session
- State MUST be persisted across restarts where applicable

---

## 6. Crate Layout

```
opennet-node/
 ├─ src/
 │   ├─ main.rs        // entry point
 │   ├─ state.rs       // state machine
 │   ├─ wire.rs        // framing + encoding
 │   ├─ crypto.rs      // signatures, hashing
 │   ├─ peer.rs        // peer management
 │   ├─ resolver.rs    // name & service resolution
 │   ├─ trust.rs       // trust graph logic
 │   ├─ stream.rs      // multiplexed streams
 │   └─ service.rs     // service registration
 └─ Cargo.toml
```

---

## 7. Wire Layer Implementation

### 7.1 Frame Structure

Each OpenNet frame is encoded as:

```
[ length | type | flags | payload | signature ]
```

The wire module MUST:
- Enforce maximum frame size
- Reject malformed frames
- Canonicalize CBOR encoding

---

## 8. Cryptography Module

### 8.1 Key Material

- Node identities MUST use Ed25519
- Service identities MAY use separate keys

### 8.2 Verification Flow

1. Decode frame
2. Extract unsigned payload
3. Verify signature
4. Pass to dispatcher

---

## 9. Event Loop (Normative Pseudocode)

```rust
loop {
    let frame = wire::read()?;
    crypto::verify(&frame)?;

    state::assert_allowed(frame.msg_type)?;

    match frame.msg_type {
        Msg::NodeHello => peer::on_hello(frame),
        Msg::ServiceAnnounce => service::on_announce(frame),
        Msg::ResolveRequest => resolver::on_request(frame),
        Msg::StreamData => stream::on_data(frame),
        _ => drop(frame)
    }
}
```

---

## 10. Public Node API

```rust
pub trait OpenNetNode {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self);
    fn join_service(&self, svc: ServiceId) -> Result<()>;
    fn resolve(&self, uri: OpenNetUri) -> Result<Vec<NodeId>>;
    fn open_stream(&self, node: NodeId) -> Result<Stream>;
}
```

---

## 11. Error Handling Rules

- Protocol violations MUST terminate peer connection
- Transient errors MAY retry
- Cryptographic failures MUST be fatal

---

## 12. Configuration

Nodes SHOULD support configuration via:
- Environment variables
- TOML configuration file

---

## 13. Logging and Observability

- All state transitions MUST be logged
- Wire errors SHOULD be traceable
- Logs MUST NOT leak private keys

---

## 14. Security Considerations

The reference node is NOT hardened for hostile environments.

It is intended for:
- Testing
- Research
- Early deployments

---

## 15. Evolution Policy

This document MAY change faster than other OpenNet RFCs.

Breaking changes MUST:
- Bump node version
- Update compliance tests

---

## 16. Relationship to Other OpenNet RFCs

This RFC:
- Implements OpenNet Core RFC
- Depends on OpenNet Wire RFC
- Is validated by OpenNet Test Vector RFC

---

## 17. Conclusion

The OpenNet Rust Reference Node provides a concrete, auditable, and executable foundation for the OpenNet ecosystem. It transforms abstract protocol definitions into a living system and serves as the ultimate correctness oracle for all implementations.

