# OpenNet Time & Epoch Semantics RFC

## Status
Draft

## Abstract
This document defines the OpenNet Time & Epoch Semantics, specifying how time, epochs, clocks, and replay windows are handled in a fully decentralized environment. It ensures deterministic behavior across Resolver, Trust Graph, Identity Lifecycle, and Transport layers without reliance on centralized time authorities.

## 1. Terminology
- **Epoch**: Monotonically increasing logical time unit bound to identity keys
- **Epoch Duration**: Maximum validity window of an epoch
- **Logical Time**: Network-derived time independent of wall-clock accuracy
- **Drift**: Difference between local clock and network-observed time
- **Replay Window**: Acceptable temporal window for message validity

RFC 2119 keywords (MUST, SHOULD, MAY) apply.

## 2. Design Principles
- No trusted global clock
- Time derived from network observation
- Epochs define security boundaries
- Time errors degrade trust, not liveness

## 3. Time Sources
Nodes MAY use:
- System clock
- Monotonic clock
- Network median time (NMT)

Nodes MUST NOT rely solely on a single external time source.

## 4. Network Median Time (NMT)
Each node computes NMT as:

```
NMT = median(peer_time_samples)
```

- Samples are collected during authenticated handshakes
- Outliers beyond drift tolerance are discarded

## 5. Drift Tolerance
- Maximum acceptable drift: ±Δ
- Default Δ = 5 minutes
- Drift beyond Δ results in trust decay

## 6. Epoch Structure
An epoch is defined as:

- epoch_id (u64)
- start_time
- max_duration
- key_material_hash

Epoch IDs MUST increase monotonically per NodeId.

## 7. Epoch Validity Rules
An epoch is valid if:

- epoch_id > previous_epoch_id
- current_time ∈ [start_time, start_time + max_duration]
- key hash matches identity declaration

## 8. Epoch Expiry
Expired epochs:

- MUST NOT authenticate new sessions
- MAY validate historical data
- MUST decay trust weight

## 9. Replay Protection
Each message includes:

- epoch_id
- message_timestamp
- nonce

Messages are rejected if:

- Timestamp outside replay window
- Nonce reused within window

## 10. Replay Window Parameters
- Default replay window: 120 seconds
- Resolver MAY apply stricter windows
- Transport MUST enforce window

## 11. Offline Nodes
Offline nodes:

- MAY miss epochs
- MUST resume with next epoch_id
- Cannot reuse expired epochs

## 12. Clock Skew Attacks
Mitigations:

- Median-based time
- Drift-based trust decay
- Epoch rejection

## 13. Interaction with Trust Graph
- Trust edges are epoch-scoped
- Older epochs decay faster
- Future epochs rejected

## 14. Resolver Behavior
Resolvers MUST:

- Reject future epochs
- Penalize excessive drift
- Prefer nodes with stable epoch cadence

## 15. Transport Behavior
Transport MUST:

- Bind sessions to epoch
- Terminate sessions on epoch expiry

## 16. Key Rotation Timing
- Rotation SHOULD occur before expiry
- Overlapping epochs MAY be announced
- Resolver prefers newest valid epoch

## 17. Security Considerations
- Prevents replay attacks
- Limits clock manipulation
- Avoids centralized time trust

## 18. Compatibility
Compatible with:

- Identity Lifecycle RFC
- Revocation & Recovery RFC
- Trust Graph RFC
- Resolver RFC

## 19. IANA Considerations
None.

## 20. References
- RFC 2119
- OpenNet Identity Lifecycle RFC
- OpenNet Revocation & Recovery RFC
