# OpenNet Resolver–Trust–Transport Integration RFC

## Status
Draft

## Abstract
This document specifies the binding semantics between the OpenNet Resolver, Trust Weight Dynamics, and Transport layers. It defines the end-to-end decision pipeline that transforms an `open://service.domain` request into authenticated, trust-aware, and policy-compliant network connections. This RFC represents the final architectural layer required to make OpenNet a complete, deployable decentralized internet protocol.

## 1. Scope
This RFC defines:
- Resolver → Trust → Transport execution order
- Deterministic node selection rules
- Admission control thresholds
- Failure and fallback behavior

This document does NOT redefine Trust computation, wire formats, or transport protocols.

RFC 2119 keywords apply.

## 2. Architectural Overview

```
open://service.domain
        ↓
Resolver (name → candidate set)
        ↓
Trust Evaluation (weights, decay, revocation)
        ↓
Node Ranking & Filtering
        ↓
Transport Admission (QUIC/TCP)
        ↓
Secure Session
```

Each layer is authoritative only for its own domain.

## 3. Resolver Responsibilities
The Resolver MUST:
- Parse `open://` URIs
- Determine service scope and namespace
- Produce an unordered candidate node set
- Perform no trust filtering

Resolver output:

```
CandidateSet = { NodeDescriptor }
```

## 4. Trust Evaluation Interface
For each candidate node, the Trust subsystem MUST return:

- Trust Weight (TW)
- Epoch validity
- Revocation status
- Trust cluster identifier

Trust evaluation MUST be deterministic.

## 5. Node Ranking Algorithm
Nodes are ranked by:

```
RankScore = TW × ScopeAffinity × DiversityFactor
```

Where:
- ScopeAffinity ∈ [0,1]
- DiversityFactor penalizes same-cluster selection

Sorting MUST be stable and deterministic.

## 6. Threshold Enforcement
The following thresholds MUST be enforced:

- ResolveThreshold: minimum TW to consider node
- ConnectThreshold: minimum TW to attempt transport
- RelayThreshold: minimum TW to relay traffic

Nodes below ResolveThreshold are discarded immediately.

## 7. Transport Admission
Transport MUST:
- Receive only ranked, filtered nodes
- Reject connections below ConnectThreshold
- Bind sessions to (NodeId, Epoch)

Transport MUST NOT perform trust computation.

## 8. Failure Handling
If connection fails:

1. Mark node as temporarily degraded
2. Retry next ranked node
3. Apply exponential backoff

Resolver MUST NOT reshuffle ranking within the same request.

## 9. Fallback Behavior
If no node meets thresholds:

- Resolver MAY widen scope
- Resolver MAY relax diversity constraints
- Resolver MUST NOT relax trust thresholds

## 10. Determinism Guarantees
Given identical inputs:

- Resolver output MUST be identical
- Node ranking MUST be identical
- Transport attempt order MUST be identical

## 11. Security Properties
This integration ensures:
- Sybil resistance via trust thresholds
- Eclipse resistance via diversity
- Revocation enforcement end-to-end

## 12. Privacy Considerations
- Resolver does not reveal trust graph
- Trust evaluation is local
- Transport sees only admitted peers

## 13. Implementation Guidance
- Use immutable request contexts
- Separate pure functions from I/O
- Cache trust evaluations per epoch

## 14. Interaction with Resolver Daemon RFC
The OS-level Resolver Daemon MUST invoke this pipeline atomically per request.

## 15. Compatibility
Compatible with:
- Resolver RFC
- Trust Weight Dynamics RFC
- Transport RFC
- Identity Lifecycle RFC
- Revocation & Recovery RFC

## 16. IANA Considerations
None.

## 17. References
- RFC 2119
- OpenNet Resolver RFC
- OpenNet Trust Weight Dynamics RFC
- OpenNet Transport RFC
