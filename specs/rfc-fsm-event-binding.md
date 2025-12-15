# 🔐 OpenNet Identity, Epoch & Revocation → FSM Event Binding RFC

> **Status:** Normative – Behavioral Binding Layer

This document defines how **Identity Lifecycle**, **Epoch/Time semantics**, and **Revocation mechanisms** produce deterministic events that drive the OpenNet Full-Node State Machine (FSM).

This RFC is mandatory for long-lived network correctness.

RFC 2119 keywords apply.

---

## 1. Purpose

The OpenNet FSM is event-driven. This document specifies:

- Which subsystems MAY generate FSM events
- Exact conditions for event emission
- Event prioritization and ordering
- Rust-level binding rules for `opennet-node`

Without this binding, the protocol is incomplete.

---

## 2. Event Producers (Normative)

The following subsystems are authorized FSM event producers:

| Producer | Responsibility |
|---|---|
| IdentityWatcher | Key validity, rotation, compromise |
| EpochMonitor | Time drift, epoch validity |
| RevocationListener | Revocation propagation |
| TrustEvaluator | Trust score dynamics |

No other component MAY emit FSM events.

---

## 3. Authorized FSM Events

FSM events are defined in the Full-Node State Machine RFC and reused verbatim:

- `PeerDiscovered`
- `SyncCompleted`
- `TrustBelowWarn`
- `TrustRecovered`
- `TrustCritical`
- `SecurityViolation`

No new FSM events may be introduced here.

---

## 4. Identity → FSM Event Mapping

### 4.1 Key Creation

- Initial key generation MUST NOT emit any FSM event

### 4.2 Key Rotation (Successful)

- Condition: New key validated and linked
- FSM Event: `TrustRecovered`

### 4.3 Key Compromise Detected

- Condition: Invalid signature or double-use proof
- FSM Event: `SecurityViolation`

### 4.4 Identity Merge / Split

- Merge success: `TrustRecovered`
- Split anomaly: `TrustBelowWarn`

---

## 5. Revocation → FSM Event Mapping

### 5.1 Local Revocation

- Immediate FSM Event: `SecurityViolation`

### 5.2 Remote Revocation (Trusted)

- If issuer trust ≥ T_warn → `TrustBelowWarn`
- If issuer trust < T_warn → no event

---

## 6. Epoch / Time → FSM Event Mapping

### 6.1 Epoch Drift

- Drift > D_warn → `TrustBelowWarn`
- Drift > D_critical → `TrustCritical`

### 6.2 Replay Window Violation

- Immediate FSM Event: `SecurityViolation`

---

## 7. Event Priority & Ordering

When multiple events are produced in the same processing window:

Priority order (highest first):

1. `SecurityViolation`
2. `TrustCritical`
3. `TrustBelowWarn`
4. `TrustRecovered`

Events MUST be processed in priority order.

---

## 8. Determinism Rules

- Event generation MUST be pure
- Inputs MUST be explicit
- Ordering MUST be stable
- No wall-clock time permitted

---

## 9. Rust Binding Requirements

In `opennet-node`:

- Each producer runs in isolation
- Events are pushed to a single deterministic queue
- FSM transition is applied synchronously

Pseudo-structure:

```text
IdentityWatcher ─┐
EpochMonitor ────┼─> EventQueue → FSM
RevocationListener ─┘
```

---

## 10. Compliance Criteria

A node is compliant if:

- All FSM events originate from authorized producers
- Event conditions match this RFC exactly
- No undocumented events exist

---

## 11. Security Considerations

Incorrect bindings may:
- Delay quarantine
- Cause false recovery
- Enable long-term trust erosion

Implementations MUST adhere strictly.

---

## 12. Conclusion

This RFC completes the behavioral linkage between protocol subsystems and the OpenNet node FSM.

It is a **hard requirement** for network longevity.

---

**End of Document**
