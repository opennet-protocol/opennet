# 🧪 OpenNet FSM Exhaustive Test Suite RFC

> **Status:** Normative – Verification & Correctness

This document defines the **mandatory exhaustive test suite** for the OpenNet Full-Node State Machine (FSM). It specifies test categories, required coverage, and failure conditions that MUST be satisfied by any compliant implementation.

RFC 2119 keywords apply.

---

## 1. Purpose

The purpose of this RFC is to answer the question:

> **“Is it possible for an OpenNet node to behave incorrectly?”**

This test suite is designed to reduce that answer to **NO** within defined protocol bounds.

---

## 2. Scope

This RFC covers:
- All valid FSM state transitions
- All invalid (illegal) transitions
- Event ordering and priority handling
- Interaction with Resolver, Trust, and Transport layers
- Stress and adversarial scenarios

---

## 3. Test Philosophy

The FSM test suite MUST:

- Be deterministic
- Be exhaustive over the state/event space
- Treat any undefined behavior as failure
- Prefer false negatives over false positives

---

## 4. State Coverage Requirements

Each FSM state MUST be covered:

- BOOTSTRAP
- SYNCING
- ACTIVE
- DEGRADED
- QUARANTINED

For each state:
- All valid outgoing transitions MUST be tested
- All invalid transitions MUST be asserted to fail

---

## 5. Transition Matrix Testing

The full Cartesian product of:

```
States × Events
```

MUST be tested.

### 5.1 Valid Transitions

Each valid transition MUST:
- Reach the correct next state
- Produce no side effects

### 5.2 Invalid Transitions

Each invalid transition MUST:
- Trigger a panic or fatal error
- Leave no persistent state change

---

## 6. Event Ordering & Priority Tests

### 6.1 Priority Enforcement

When multiple events occur in the same processing window, tests MUST verify:

Priority order:
1. SecurityViolation
2. TrustCritical
3. TrustBelowWarn
4. TrustRecovered

Lower-priority events MUST NOT override higher-priority outcomes.

---

## 7. Burst & Storm Scenarios

The FSM MUST be tested under:

- Event bursts (≥100 events)
- Mixed event types
- Repeated identical events

Expected behavior:
- Deterministic final state
- No deadlock
- No panic unless specified

---

## 8. Resolver / Transport Interaction Tests

The following MUST be tested:

- Resolver failure → TrustBelowWarn
- Transport crypto failure → SecurityViolation
- Success path produces no FSM event

FSM state MUST remain stable across successful requests.

---

## 9. Time & Epoch Boundary Tests

Test cases MUST include:

- Epoch rollover during ACTIVE
- Epoch drift exceeding thresholds
- Replay window violations

FSM transitions MUST match the Identity/Epoch binding RFC.

---

## 10. Quarantine Behavior Tests

Tests MUST verify:

- QUARANTINED is terminal
- No transition out without manual recovery
- All incoming events are ignored or logged

---

## 11. Determinism Tests

The FSM MUST produce identical results when:

- Replayed with identical event sequences
- Executed on different architectures

Byte-for-byte equivalence is REQUIRED.

---

## 12. Failure Classification

A test failure is classified as:

- **Critical:** FSM reached invalid state
- **Major:** Wrong transition taken
- **Minor:** Incorrect error signaling

Any Critical or Major failure invalidates compliance.

---

## 13. Automation Requirements

The test suite MUST:

- Be fully automated
- Run in CI
- Fail fast on invariant violations

Manual testing is NOT sufficient.

---

## 14. Compliance Criteria

An implementation is FSM-compliant if:

- All tests defined in this RFC pass
- No undefined behavior is observed

---

## 15. Security Considerations

Incomplete FSM testing can allow:

- Silent trust erosion
- State desynchronization
- Permanent network degradation

This test suite is security-critical.

---

## 16. Conclusion

This RFC defines the **final verification layer** for OpenNet node correctness.

If this test suite passes, the FSM is considered behaviorally sound.

---

**End of Document**
