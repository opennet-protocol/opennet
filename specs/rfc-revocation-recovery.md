# OpenNet Revocation & Recovery RFC

## Status
Draft

## Abstract
This document specifies the OpenNet Revocation & Recovery mechanism, defining how node identities are revoked, quarantined, and optionally recovered after key compromise or malicious behavior. It complements the Identity Lifecycle RFC and Trust Graph RFC, ensuring long-term network integrity without centralized authorities.

## 1. Terminology
- **NodeId**: Stable logical identifier of a node
- **Key Epoch**: Time-bounded cryptographic key validity period
- **Revocation Event**: Signed declaration that a key or identity is compromised
- **Quorum**: Minimum set of trusted peers required to validate a revocation
- **Recovery Identity**: New keyset bound to an existing NodeId

RFC 2119 keywords (MUST, SHOULD, MAY) apply.

## 2. Threat Model
Revocation addresses:
- Private key compromise
- Forced key disclosure
- Long-term silent impersonation
- Trust laundering via identity reset

Revocation does NOT attempt to recover anonymity lost prior to detection.

## 3. Revocation Triggers
A node MUST initiate revocation if:
- A private key is exposed
- Unauthorized signatures are detected
- Trust graph decay exceeds threshold

Peers MAY initiate revocation votes if:
- Byzantine behavior is observed
- Conflicting epochs are broadcast

## 4. Revocation Objects
A revocation is a signed CBOR object:

- type = REVOCATION
- node_id
- revoked_epoch
- reason_code
- timestamp
- signatures[]

## 5. Revocation Propagation
- Revocations MUST be gossip-propagated
- Transport layer MUST prioritize revocation messages
- Resolver MUST immediately exclude revoked epochs

## 6. Quorum Validation
Revocation is considered valid when:

- Node self-signs OR
- A quorum of trusted peers co-sign

Quorum size is determined by trust weight, not raw count.

## 7. Trust Graph Impact
Upon revocation:

- Trust score for revoked epoch decays to zero
- Descendant trust edges are attenuated
- Resolver ranking excludes revoked identity

## 8. Recovery Process
Recovery is OPTIONAL and proceeds as follows:

1. Node generates new keypair
2. Recovery declaration references revoked epoch
3. Trusted peers co-sign recovery
4. Trust graph links new epoch with reduced weight

Recovery does NOT restore full historical trust.

## 9. Identity Continuity Rules
- NodeId remains constant
- Epoch numbers MUST increase monotonically
- Multiple concurrent recoveries are forbidden

## 10. Offline & Delayed Revocation
Nodes offline during compromise:
- MUST revoke upon reconnection
- Peers MAY retroactively penalize trust

## 11. Resolver Behavior
Resolvers MUST:
- Reject revoked epochs
- Prefer recovered identities with sufficient quorum
- Cache revocation state with expiry

## 12. Transport Behavior
Transport MUST:
- Drop sessions authenticated by revoked keys
- Refuse new handshakes from revoked epochs

## 13. Security Considerations
- Prevents silent long-term impersonation
- Limits blast radius of compromise
- Resists false revocation via quorum enforcement

## 14. Compatibility
Fully compatible with:
- Identity Lifecycle RFC
- Trust Graph RFC
- Resolver RFC

## 15. Future Extensions
- Social recovery mechanisms
- Hardware-backed recovery attestations
- Post-quantum revocation signatures

## 16. IANA Considerations
None.

## 17. References
- OpenNet Identity Lifecycle RFC
- OpenNet Trust Graph RFC
- RFC 2119
