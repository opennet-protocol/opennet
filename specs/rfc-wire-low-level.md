# OpenNet Low-Level Specifications

## Binary Schemas, Trust Graph Mathematics, and Minimal Rust Reference Node

Category: Standards Track (Companion RFC)
Status: Draft

---

## Abstract

This document defines the low-level technical specifications required to implement interoperable OpenNet nodes. It extends the core OpenNet RFC and Companion RFCs by providing (1) exact binary message schemas, (2) a formal trust graph model with mathematical definitions, and (3) a minimal Rust-based reference node architecture including state machines and wire handling. All definitions in this document are fully compatible with prior OpenNet specifications.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Exact Binary Schemas

### 2.1 Binary Frame Header

All OpenNet messages MUST be encapsulated in a binary frame with the following fixed header:

| Field | Size | Description |
|------|------|-------------|
| frame_len | uint32 | Total frame length |
| version | uint16 | Protocol version |
| msg_type | uint16 | Message type ID |
| flags | uint16 | Control flags |
| payload_len | uint32 | Payload length |

All integers MUST be encoded in network byte order (big-endian).

---

### 2.2 Message Type Registry (Partial)

| Message Type | ID (uint16) |
|-------------|------------|
| NODE_HELLO | 0x0001 |
| NODE_WELCOME | 0x0002 |
| SERVICE_JOIN | 0x0101 |
| SERVICE_LEAVE | 0x0102 |
| DNS_LOOKUP | 0x0201 |
| DNS_RESPONSE | 0x0202 |
| STREAM_OPEN | 0x0301 |
| STREAM_DATA | 0x0302 |
| STREAM_CLOSE | 0x0303 |

Unrecognized message types MUST be ignored.

---

### 2.3 TLV Schema

TLV elements are encoded as:

| Field | Size |
|------|------|
| type | uint16 |
| length | uint32 |
| value | variable |

#### Common TLV Types

| TLV | ID |
|-----|----|
| NODE_ID | 0x0001 |
| SERVICE_ID | 0x0002 |
| DOMAIN | 0x0003 |
| SCOPE | 0x0004 |
| TIMESTAMP | 0x0005 |
| SIGNATURE | 0x00FF |

TLV ordering MUST be deterministic for signed messages.

---

### 2.4 CBOR Canonical Maps

CBOR payloads MUST use canonical CBOR encoding.

Example: SERVICE_JOIN

```
{
  1: service_id,
  2: domain,
  3: node_id,
  4: timestamp
}
```

CBOR keys are fixed numeric identifiers.

---

### 2.5 Signature Encoding

Signatures MUST be embedded as the final payload element.

- Algorithm: Ed25519 (baseline)
- Signature covers: frame header + payload (excluding signature field)

Future algorithms MUST follow the same placement rules.

---

## 3. Trust Graph Mathematics

### 3.1 Trust Graph Definition

The OpenNet trust model is defined as a directed weighted graph:

```
G = (V, E)
```

Where:
- V is the set of nodes
- E is the set of trust edges

Each edge e(u, v) has weight w ∈ [0,1].

---

### 3.2 Trust Accumulation

For each successful interaction:

```
w(u, v) = min(1, w(u, v) + α)
```

Where α is a small constant trust increment.

---

### 3.3 Trust Decay

Trust decays exponentially over time:

```
w_t(u, v) = w_0(u, v) · e^(−λt)
```

Where:
- λ is the decay constant
- t is elapsed time

---

### 3.4 Transitive Trust Cap

Transitive trust is computed as:

```
T(u, v) = Σ (w(u, n) · w(n, v))
```

With the constraint:

```
T(u, v) ≤ τ
```

Where τ is a global cap preventing Sybil amplification.

---

### 3.5 Node Selection Score

Final node ranking score:

```
S = a·L + b·A + c·T − d·R
```

Where:
- L = normalized latency
- A = availability
- T = trust score
- R = route depth

Coefficients (a–d) are resolver-local.

---

## 4. Minimal Rust Reference Node

### 4.1 Architecture Overview

The minimal Rust node consists of the following modules:

- wire (frame parsing)
- crypto (sign/verify)
- peer (connection handling)
- service (service registry)
- resolver (ranking & selection)
- state (state machines)

---

### 4.2 Node State Machine

```
INIT → HANDSHAKE → SYNC → ACTIVE → SHUTDOWN
```

Invalid transitions MUST be rejected.

---

### 4.3 Stream State Machine

```
IDLE → OPEN → ESTABLISHED → CLOSING → CLOSED
```

Streams MUST enforce ordering and integrity.

---

### 4.4 Wire Handling (Rust Pseudocode)

```rust
loop {
    let frame = read_frame(socket)?;
    verify_signature(&frame)?;
    match frame.msg_type {
        MSG_NODE_HELLO => handle_hello(frame),
        MSG_SERVICE_JOIN => handle_service_join(frame),
        MSG_STREAM_DATA => handle_stream_data(frame),
        _ => {}
    }
}
```

---

### 4.5 Minimal API Surface

```rust
pub trait OpenNetNode {
    fn join_service(&self, service: ServiceId);
    fn leave_service(&self, service: ServiceId);
    fn resolve(&self, uri: OpenNetUri) -> NodeId;
    fn open_stream(&self, node: NodeId) -> Stream;
}
```

---

## 5. Compliance Requirements

A compliant implementation MUST:

- Parse all defined binary schemas
- Verify all signatures
- Enforce trust decay and caps
- Implement state machines exactly

Non-compliant nodes MUST be isolated locally.

---

## 6. Conclusion

This document provides the final low-level foundation required to implement OpenNet in a fully interoperable manner. Together with the core and companion RFCs, it enables independent teams to build secure, compatible, and censorship-resistant OpenNet nodes and resolvers.

