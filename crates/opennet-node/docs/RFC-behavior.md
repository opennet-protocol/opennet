# OpenNet Full-Node Behavior & Integration RFC

## Status
Draft

## Abstract
This document specifies the complete, normative behavior of an OpenNet full node. It defines node states, state transitions, integration between Resolver, Trust, Time, Identity, and Transport layers, and the required behavior under normal operation, failure, and attack conditions. This RFC answers the question: "Is this node behaving correctly?"

RFC 2119 keywords (MUST, SHOULD, MAY) apply.

---

## 1. Node Definition
An OpenNet Full Node is an implementation that:
- Participates in Resolver queries
- Maintains a Trust Graph
- Manages Identity epochs
- Enforces Time & Epoch semantics
- Establishes Transport connections

Partial implementations MUST NOT claim full-node compliance.

---

## 2. Node State Machine

### 2.1 Global Node States
A node MUST be in exactly one of the following states:

- **BOOTSTRAP** – initial startup
- **SYNCING** – trust, epoch, and revocation sync
- **ACTIVE** – serving and resolving requests
- **DEGRADED** – temporary failures detected
- **QUARANTINED** – revoked or unsafe state
- **SHUTDOWN** – orderly termination

State transitions MUST be deterministic.

---

## 3. Bootstrap Behavior
During BOOTSTRAP:
- Node loads identity and last epoch
- Node validates local clock sanity
- Node establishes initial peer set
- Resolver daemon registers node availability

Failure to validate identity or time MUST abort bootstrap.

---

## 4. Syncing Phase
During SYNCING:
- Trust graph snapshots are exchanged
- Revocation objects are verified
- Epoch continuity is validated

A node MUST NOT serve open:// requests while SYNCING.

---

## 5. Active Operation
In ACTIVE state, a node MUST:
- Resolve incoming open:// requests
- Apply Resolver → Trust → Transport pipeline
- Enforce trust thresholds
- Bind sessions to (NodeId, Epoch)

---

## 6. Failure Detection
A node MUST monitor:
- Transport failures
- Clock drift violations
- Trust decay anomalies
- Repeated resolver mismatches

Failures are classified as transient or critical.

---

## 7. Degraded State
In DEGRADED state:
- Node MAY continue serving existing sessions
- Node MUST reduce trust advertisements
- Node SHOULD attempt recovery actions

Persistent DEGRADED state MUST escalate.

---

## 8. Quarantine State
A node enters QUARANTINED if:
- Its current epoch is revoked
- Severe protocol violations occur
- Trust weight drops below survival threshold

In QUARANTINED state:
- All transport sessions are terminated
- Resolver advertisements cease
- Manual or quorum-based recovery is required

---

## 9. Identity & Epoch Integration
Nodes MUST:
- Rotate keys before epoch expiry
- Reject future epochs
- Apply revocation immediately

Epoch transitions MUST be atomic.

---

## 10. Trust Integration
- Trust weights MUST be recalculated on epoch change
- Negative events MUST immediately reduce trust
- Trust updates MUST be monotonic within an epoch

---

## 11. Resolver Integration
Resolver MUST:
- Operate deterministically
- Cache results per request
- Never bypass trust thresholds

---

## 12. Transport Integration
Transport MUST:
- Accept only Resolver-approved nodes
- Enforce backpressure and flow control
- Terminate sessions on trust or epoch invalidation

---

## 13. Integration Failure Scenarios

### 13.1 Resolver–Trust Mismatch
If resolver candidate set conflicts with trust evaluation:
- Trust decision prevails
- Resolver cache MUST be invalidated

### 13.2 Trust–Transport Mismatch
If transport fails after trust approval:
- Node marked temporarily degraded
- Retry with next ranked node

---

## 14. Multi-Node Interaction
Full nodes MUST:
- Exchange trust summaries
- Propagate revocations
- Validate peer epoch chains

---

## 15. Determinism Requirements
Given identical inputs:
- Node state transitions MUST be identical
- Resolver outcomes MUST be identical
- Failure escalation MUST be identical

---

## 16. Compliance Criteria
A node is compliant if:
- All state transitions follow this RFC
- All thresholds are enforced
- All invalid states are rejected

---

## 17. Testing Requirements
Implementations MUST provide:
- State machine tests
- Failure injection tests
- Epoch edge-case tests

---

## 18. Security Considerations
- Prevents undefined behavior exploitation
- Limits cascading failures
- Ensures predictable recovery

---

## 19. Compatibility
Compatible with:
- Resolver–Trust–Transport Integration RFC
- Identity Lifecycle RFC
- Revocation & Recovery RFC
- Time & Epoch Semantics RFC

---

## 20. IANA Considerations
None.

---

## 21. References
- RFC 2119
- OpenNet Core Protocol RFC
