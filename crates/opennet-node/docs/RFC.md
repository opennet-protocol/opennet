# 🧠 OpenNet Full-Node State Machine RFC

> **Status:** Normative – Executable Specification

This document defines the authoritative full-node state machine for OpenNet and its **one-to-one Rust implementation** in the `opennet-node` crate.

RFC 2119 keywords apply.

---

## 1. Scope

This RFC specifies:
- All valid node states
- Allowed state transitions
- Transition triggers
- Failure handling semantics
- Rust reference implementation

Any behavior not defined here MUST be considered invalid.

---

## 2. Node States (Finite State Machine)

An OpenNet full node SHALL exist in exactly one of the following states:

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

### 2.1 BOOTSTRAP
Initial state on node startup.

**Entry conditions:**
- No trusted peers
- No local trust graph

**Exit conditions:**
- At least one valid peer discovered

---

### 2.2 SYNCING
The node is synchronizing identity, trust, and service metadata.

**Entry conditions:**
- Valid bootstrap peer exists

**Exit conditions:**
- Trust graph reaches minimum completeness threshold

---

### 2.3 ACTIVE
The node is fully operational.

**Capabilities:**
- Resolve open:// URIs
- Forward traffic
- Participate in trust propagation

---

### 2.4 DEGRADED
The node operates with reduced trust or connectivity.

**Triggers:**
- Peer loss
- Trust score decay
- Transport instability

---

### 2.5 QUARANTINED
The node is isolated due to security concerns.

**Triggers:**
- Identity revocation
- Trust score below critical threshold
- Consensus-level rejection

---

## 3. State Transition Rules (Normative)

| From | To | Condition |
|---|---|---|
| BOOTSTRAP | SYNCING | ≥1 valid peer |
| SYNCING | ACTIVE | Sync complete |
| ACTIVE | DEGRADED | Trust < T_warn |
| DEGRADED | ACTIVE | Trust ≥ T_ok |
| ACTIVE | QUARANTINED | Trust < T_min |
| DEGRADED | QUARANTINED | Security event |

Any other transition MUST panic.

---

## 4. Transition Determinism

State transitions MUST:
- Be pure functions
- Depend only on explicit inputs
- Produce no side effects

---

## 5. Error Semantics

- Illegal transitions = fatal error
- State corruption = immediate quarantine

---

## 6. Rust Reference Implementation

### 6.1 State Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Bootstrap,
    Syncing,
    Active,
    Degraded,
    Quarantined,
}
```

---

### 6.2 Transition Events

```rust
pub enum StateEvent {
    PeerDiscovered,
    SyncCompleted,
    TrustBelowWarn,
    TrustRecovered,
    TrustCritical,
    SecurityViolation,
}
```

---

### 6.3 Transition Function (Authoritative)

```rust
impl NodeState {
    /// RFC: Full-Node State Machine §3
    pub fn transition(self, event: StateEvent) -> NodeState {
        use NodeState::*;
        use StateEvent::*;

        match (self, event) {
            (Bootstrap, PeerDiscovered) => Syncing,
            (Syncing, SyncCompleted) => Active,
            (Active, TrustBelowWarn) => Degraded,
            (Degraded, TrustRecovered) => Active,
            (Active, TrustCritical) => Quarantined,
            (Degraded, SecurityViolation) => Quarantined,

            _ => panic!("Illegal state transition: {:?} -> {:?}", self, event),
        }
    }
}
```

---

## 7. State Machine Invariants

The following invariants MUST hold:

- QUARANTINED is terminal unless manual recovery
- ACTIVE implies resolver + transport enabled
- DEGRADED limits forwarding capacity

---

## 8. Test Requirements

Each transition MUST have:
- Unit test
- Invalid transition test (panic expected)

Example:

```rust
#[test]
fn bootstrap_to_syncing() {
    assert_eq!(
        NodeState::Bootstrap.transition(StateEvent::PeerDiscovered),
        NodeState::Syncing
    );
}
```

---

## 9. Compliance

A node implementation is compliant if:
- It implements this FSM exactly
- It rejects undefined transitions

---

## 10. Conclusion

This state machine defines the **behavioral core** of an OpenNet node.

Any deviation breaks protocol correctness.

---

**End of RFC**

