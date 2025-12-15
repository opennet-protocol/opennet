# 🦀 OpenNet Rust Reference Implementation – Freeze Plan

> **Status:** Normative (Executable Specification)

This document defines the formal freeze process for the OpenNet Rust reference implementation. Once applied, the Rust implementation becomes the **authoritative executable specification** for all OpenNet RFCs.

---

## 🎯 1. Objective

The Rust reference implementation SHALL:

- Act as the **single source of truth** for protocol behavior
- Encode RFC semantics in executable form
- Serve as the **golden reference** for other language implementations
- Produce canonical test vectors

> After freeze, RFC text MUST conform to Rust behavior, not vice versa.

---

## 📁 2. Repository Structure (LOCKED)

The following repository layout is normative and MUST NOT change without a major protocol version bump:

```text
opennet-node/
├─ crates/
│  ├─ opennet-core        # Core types, NodeId, ServiceId, Epoch
│  ├─ opennet-wire        # CBOR / TLV canonical encoding
│  ├─ opennet-identity    # Key lifecycle, rotation, merge/split
│  ├─ opennet-revocation  # Revocation & recovery logic
│  ├─ opennet-time        # Epochs, monotonic network time (NMT)
│  ├─ opennet-trust       # Trust graph & weight dynamics
│  ├─ opennet-resolver    # open:// URI resolution
│  ├─ opennet-transport   # QUIC / TCP bindings
│  ├─ opennet-node        # Full-node state machine
│  └─ opennet-tests       # Compliance & integration tests
│
├─ specs/                 # RFC-aligned generated specs
├─ test-vectors/          # Canonical CBOR / JSON vectors
├─ fuzz/                  # Optional fuzz harnesses
└─ Cargo.toml
```

---

## 📜 3. Executable Specification Rules

### ❌ Forbidden

The following constructs MUST NOT be used:

- Floating point arithmetic (`f32`, `f64`)
- Non-deterministic collections (`HashMap`, `HashSet`)
- Direct system time access
- Unseeded randomness

### ✅ Mandatory

- Fixed-point arithmetic for trust math
- Canonical CBOR encoding
- Stable sorting (lexicographic)
- Explicit error codes (no silent failure)

> Determinism is a hard requirement.

---

## 🔗 4. RFC ↔ Code Mapping (Normative)

Each crate maps directly to a specific RFC:

| Crate | Governing RFC |
|---|---|
| `opennet-identity` | Identity Lifecycle RFC |
| `opennet-revocation` | Revocation & Recovery RFC |
| `opennet-time` | Time & Epoch Semantics RFC |
| `opennet-trust` | Trust Weight Dynamics RFC |
| `opennet-resolver` | Resolver RFC |
| `opennet-transport` | Transport RFC |
| `opennet-node` | Full-Node Behavior RFC |

All public functions MUST reference their governing RFC section:

```rust
/// RFC: Trust Weight Dynamics §7.2
pub fn decay(epoch_age: EpochDelta) -> FixedPoint { ... }
```

---

## 🔄 5. Node State Machine (LOCKED)

The full-node state machine is normative:

```text
BOOTSTRAP
  ↓
SYNCING
  ↓
ACTIVE
  ↘
DEGRADED
  ↘
QUARANTINED
```

### Rules

- State transitions MUST be explicit and table-driven
- Illegal transitions MUST panic and fail tests
- State evolution MUST be a pure function of inputs

---

## 🧪 6. Test Requirements (FREEZE GATE)

Freeze SHALL NOT occur unless all test layers pass:

### Unit Tests
- Epoch monotonicity
- Trust decay math
- Resolver determinism

### Compliance Tests
- Canonical CBOR vectors
- Revocation propagation
- Replay window enforcement

### Integration Tests
- Resolver → Trust → Transport pipeline
- Multi-node (3–5 nodes)
- NAT-diverse topology

> Test failure == RFC violation

---

## 🏷️ 7. Versioning & Freeze Policy

- `v1.0.0-alpha` – Architecture frozen
- `v1.0.0-beta` – Interoperability validated
- `v1.0.0` – Internet-Draft reference implementation

Breaking changes:
- ONLY allowed in `v2.0.0`
- MUST be accompanied by new Internet-Draft revisions

---

## 🌍 8. Impact on Other Implementations

The Rust implementation:

- Produces canonical binary outputs
- Defines deterministic behavior
- Acts as golden test vector source

Other languages (Go, C++, etc.):
- MUST match Rust outputs byte-for-byte

---

## 🔚 9. Conclusion

With this freeze plan applied, OpenNet transitions from protocol design to **standard-grade implementation**.

The Rust node becomes:

- Executable RFC
- Interoperability anchor
- Long-term protocol stabilizer

---

**End of Document**
