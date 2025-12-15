# OpenNet Trust Weight Dynamics RFC

## Status
Draft

## Abstract
This document specifies the Trust Weight Dynamics of OpenNet, defining how trust scores are computed, decayed, propagated, and consumed by the Resolver and Transport layers. It formalizes the mathematical foundation that makes OpenNet economically and cryptographically resistant to Sybil, Eclipse, and reputation laundering attacks.

## 1. Terminology
- **Trust Weight (TW)**: Continuous score representing confidence in a node
- **Trust Edge**: Directed weighted relationship between nodes
- **Decay Function**: Mathematical function reducing trust over time
- **Epoch Age**: Difference between current epoch and edge creation epoch
- **Threshold**: Minimum trust required for actions

RFC 2119 keywords (MUST, SHOULD, MAY) apply.

## 2. Design Goals
- Deterministic computation
- Sybil-cost amplification
- Gradual forgiveness, rapid punishment
- No hard trust resets

## 3. Trust Weight Domain
- TW ∈ [0.0, 1.0]
- 0.0 = no trust
- 1.0 = maximal trust

Trust weights MUST be represented as fixed-point or float with deterministic rounding.

## 4. Initial Trust Assignment
- New nodes start with TW = ε (default 0.01)
- No implicit trust from identity alone

## 5. Trust Edge Formation
A trust edge e(i → j) is created when:
- Node i successfully interacts with j
- Interaction passes protocol validation

Edge attributes:
- weight w ∈ (0,1]
- epoch_created
- last_updated

## 6. Trust Propagation Model
Trust propagates multiplicatively:

```
TW(j) += TW(i) × w(i,j) × decay(age)
```

No path amplification beyond 1 hop is allowed by default.

## 7. Decay Functions
Default decay function (exponential):

```
decay(t) = e^(−λt)
```

Where:
- t = epoch age
- λ = decay constant (default 0.05)

Resolvers MUST use identical decay parameters.

## 8. Epoch-Aware Trust
- Each trust edge is bound to an epoch
- Older epochs decay faster
- Revoked epochs immediately set decay = 0

## 9. Revocation Impact
Upon revocation:

- All edges from revoked epoch collapse
- Downstream trust reduced proportionally
- Recovery epochs start with reduced TW

## 10. Negative Trust Events
Negative events reduce trust:

```
TW = TW × penalty_factor
```

Penalty factor default = 0.2

## 11. Trust Saturation
- TW is capped at 1.0
- Marginal trust gain diminishes near cap

## 12. Resolver Thresholds
Resolvers MUST define thresholds:

- Resolve threshold (default 0.2)
- Connect threshold (default 0.3)
- Relay threshold (default 0.4)

Nodes below threshold are ignored.

## 13. Transport Enforcement
Transport MUST:
- Refuse handshakes below connect threshold
- Drop sessions when TW falls below minimum

## 14. Sybil Resistance Analysis
- Cost to raise TW grows exponentially
- Trust cannot be rapidly accumulated
- Identity splitting resets accumulated TW

## 15. Eclipse Resistance
- Resolver selects nodes from diverse trust clusters
- Trust diversity weighting is REQUIRED

## 16. Trust Decay Stability
- Long-lived nodes stabilize
- Dormant nodes gradually fade
- Burst behavior penalized

## 17. Determinism Requirements
- Same inputs MUST yield same TW
- Floating-point nondeterminism MUST be avoided

## 18. Implementation Guidance
- Use fixed-point arithmetic where possible
- Cache trust computations with epoch keys

## 19. Security Considerations
- Prevents reputation laundering
- Limits long-range trust attacks
- Discourages spam identities

## 20. Compatibility
Compatible with:

- Trust Graph RFC
- Identity Lifecycle RFC
- Revocation & Recovery RFC
- Time & Epoch Semantics RFC

## 21. IANA Considerations
None.

## 22. References
- RFC 2119
- OpenNet Trust Graph RFC
- OpenNet Time & Epoch Semantics RFC
