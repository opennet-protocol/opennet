# OpenNet Implementation & Deployment RFC

## Reference Node Implementation, Resolver Daemon, Test Vectors, and Internet-Draft Packaging

Category: Standards Track (Implementation & Deployment)
Status: Draft

---

## Abstract

This document defines the practical implementation layer of the OpenNet protocol family. It specifies (1) a working Rust reference node aligned with all prior OpenNet RFCs, (2) compliance test vectors, (3) an operating-system–level OpenNet resolver daemon, (4) canonical CBOR and TLV schema definitions, and (5) guidance for splitting the OpenNet specification set into formal IETF Internet-Draft documents. This document is normative unless otherwise stated.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Rust Reference Node (Executable Specification)

### 2.1 Design Goals

The Rust reference node serves as an executable specification.

It MUST:
- Implement the OpenNet wire format
- Enforce state machines
- Validate signatures
- Participate in gossip, routing, and streams
- Remain minimal and auditable

---

### 2.2 Crate Structure

```
opennet-node/
 ├─ src/
 │   ├─ main.rs
 │   ├─ wire.rs
 │   ├─ crypto.rs
 │   ├─ peer.rs
 │   ├─ service.rs
 │   ├─ resolver.rs
 │   ├─ trust.rs
 │   ├─ stream.rs
 │   └─ state.rs
 └─ Cargo.toml
```

---

### 2.3 Node State Machine

```
INIT → HANDSHAKE → DISCOVERY → ACTIVE → SHUTDOWN
```

State transitions MUST be explicit and validated.

---

### 2.4 Core Event Loop (Rust)

```rust
loop {
    let frame = wire::read(&socket)?;
    crypto::verify(&frame)?;

    match frame.msg_type {
        Msg::NodeHello => peer::handle_hello(frame),
        Msg::ServiceJoin => service::handle_join(frame),
        Msg::RouteRequest => resolver::handle_route(frame),
        Msg::StreamData => stream::handle_data(frame),
        _ => {}
    }
}
```

---

### 2.5 Public API Surface

```rust
pub trait OpenNetNode {
    fn start(&mut self);
    fn join_service(&self, service: ServiceId);
    fn resolve(&self, uri: OpenNetUri) -> NodeId;
    fn open_stream(&self, node: NodeId) -> Stream;
}
```

---

## 3. Compliance Test Vectors

### 3.1 Test Categories

Implementations MUST pass all tests in the following categories:

- Frame parsing
- Signature verification
- Resolver determinism
- Trust decay correctness
- State machine enforcement

---

### 3.2 Example Vector (CBOR)

SERVICE_JOIN payload:

```
{
  1: "service_id",
  2: "chat.opennet",
  3: "node_pubkey",
  4: 1700000000
}
```

Expected result: ACCEPT

---

### 3.3 Negative Vectors

- Invalid signature → REJECT
- Expired TTL → DROP
- Invalid state transition → ABORT

---

## 4. OpenNet Resolver Daemon (OS-Level)

### 4.1 Purpose

The resolver daemon provides OpenNet resolution as a system service.

It MUST:
- Register open:// and service:// URI schemes
- Perform resolution and node ranking
- Expose local IPC APIs

---

### 4.2 Architecture

```
Application
   ↓ IPC
OpenNet Resolver Daemon
   ↓
OpenNet Network
```

---

### 4.3 Resolver Workflow

1. Receive URI
2. Validate syntax
3. Resolve domain
4. Rank nodes
5. Return best node or node list

---

### 4.4 OS Integration

- Linux: systemd service + UNIX socket
- Windows: background service + named pipe
- macOS: launchd + XPC

---

## 5. Canonical CBOR / TLV Schema Files

### 5.1 CBOR Schema Rules

- Numeric keys only
- Canonical ordering
- Deterministic encoding

---

### 5.2 TLV Registry (Excerpt)

| ID | Name |
|----|------|
| 0x0001 | NODE_ID |
| 0x0002 | SERVICE_ID |
| 0x0003 | DOMAIN |
| 0x0004 | SCOPE |
| 0x00FF | SIGNATURE |

---

### 5.3 Distribution

Schema files MUST be published as:
- JSON (human-readable)
- CBOR (machine)

---

## 6. Internet-Draft Packaging

### 6.1 Document Split

The OpenNet specification set SHOULD be published as:

1. opennet-core
2. opennet-wire
3. opennet-resolver
4. opennet-security
5. opennet-governance
6. opennet-implementation

---

### 6.2 Naming Convention

```
draft-opennet-core-00.txt
draft-opennet-wire-00.txt
draft-opennet-resolver-00.txt
```

---

### 6.3 Versioning

Each draft MUST follow IETF versioning rules.

---

## 7. Conclusion

This document transforms OpenNet from a theoretical protocol suite into a deployable system. With a reference node, resolver daemon, test vectors, and draft structure, OpenNet is now suitable for real-world experimentation and formal standardization.

