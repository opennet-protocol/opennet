# 📜 OpenNet Identity Lifecycle RFC

**Title:** OpenNet Identity Lifecycle – Node Identity, Key Management, and Trust Continuity  
**Status:** Draft  
**Category:** Standards Track  
**Intended Status:** Proposed Standard  
**Updates:** OpenNet Core RFC, Trust Graph RFC, Security Extensions RFC

---

## 1. Introduction

This document defines the **lifecycle of node identities** in the OpenNet protocol.

Identity is a **long-lived security primitive** that directly affects:
- Trust graph stability
- Sybil resistance
- Network continuity
- Upgrade safety

Without a formal identity lifecycle, decentralized networks suffer from **trust rot**, identity inflation, and irreversible fragmentation. This RFC closes that gap.

---

## 2. Terminology

The key words **MUST**, **MUST NOT**, **SHOULD**, **MAY** are to be interpreted as described in RFC 2119.

- **Node Identity**: Cryptographic identity of a node
- **NodeId**: Canonical identifier derived from a public key
- **Identity Epoch**: A bounded validity period of a keypair
- **Key Rotation**: Replacement of an active keypair
- **Identity Compromise**: Loss of private key confidentiality

---

## 3. Identity Model Overview

### 3.1 Immutable Identity, Mutable Keys

OpenNet separates:
- **Identity continuity** (stable)
- **Key material** (rotatable)

```
NodeId = HASH(canonical_public_key)
```

The NodeId is **immutable for its lifetime**.

---

## 4. Identity Creation (Birth)

### 4.1 Key Generation

A node **MUST** generate a cryptographic keypair using an approved algorithm.

Mandatory:
- Ed25519 (baseline)

Optional / Future:
- Post-quantum schemes (see Security Extensions RFC)

---

### 4.2 NodeId Derivation

The NodeId **MUST** be derived as:

```
NodeId = SHA-256(canonical_public_key_bytes)
```

Implementations **MUST NOT** introduce salting or randomness.

---

### 4.3 Identity Announcement

Upon first network participation, a node **MUST** announce:
- NodeId
- Public key
- Identity epoch = 0

Announcement messages **MUST** be signed.

---

## 5. Identity Epochs

### 5.1 Epoch Definition

An **identity epoch** represents a continuous validity interval of a keypair.

Each epoch is defined by:
- Epoch number (monotonic)
- Public key
- Signature binding to previous epoch

---

### 5.2 Epoch Invariants

- Epoch numbers **MUST** increase monotonically
- Epoch rollback **MUST NOT** be accepted
- Parallel epochs **MUST NOT** exist

Violation results in identity rejection.

---

## 6. Key Rotation

### 6.1 Rotation Trigger

Key rotation **MAY** occur due to:
- Scheduled rotation
- Cryptographic hygiene
- Suspected compromise

---

### 6.2 Rotation Procedure

To rotate keys, a node **MUST** broadcast a **KeyRotation message** containing:

- NodeId
- Old epoch number
- New epoch number
- New public key
- Signature by old private key

---

### 6.3 Trust Continuity

If rotation is valid:
- Trust graph **MUST** preserve trust score
- No trust reset occurs

If rotation is invalid:
- Identity **MUST** be rejected

---

## 7. Identity Compromise

### 7.1 Compromise Declaration

If a private key is compromised, the node **MUST** emit a **CompromiseNotice**:
- Signed by the compromised key (if possible)
- Or countersigned by trusted peers

---

### 7.2 Trust Impact

Upon compromise:
- Trust score **MUST** decay aggressively
- Resolver **SHOULD** deprioritize the node
- Transport **MAY** refuse new connections

---

## 8. Identity Decay and Retirement

### 8.1 Inactivity Decay

If a node remains inactive beyond a threshold:
- Trust **MUST** decay exponentially
- Identity remains valid but unattractive

---

### 8.2 Identity Retirement

A node **MAY** voluntarily retire an identity.

Retired identities:
- MUST NOT be reused
- MUST NOT be resurrected

---

## 9. Identity Merge and Split

### 9.1 Identity Merge (Restricted)

Identity merge:
- **MUST NOT** be supported automatically
- **MAY** occur only via explicit protocol extension

Rationale: prevents trust laundering.

---

### 9.2 Identity Split

Identity split:
- Creates a new NodeId
- Trust **MUST NOT** transfer automatically

Split identities start with zero trust.

---

## 10. Trust Graph Integration

### 10.1 Epoch-Aware Trust

Trust edges **MUST** bind to:
- NodeId
- Epoch number

Old epochs **MUST NOT** accumulate trust.

---

### 10.2 Rotation Effects

Valid rotations:
- Preserve inbound trust edges

Invalid rotations:
- Invalidate trust edges

---

## 11. Security Considerations

This RFC ensures:
- Key compromise containment
- Anti-Sybil reinforcement
- Long-term trust stability

Identity continuity is treated as a **first-class security property**.

---

## 12. Compliance Requirements

An implementation is compliant if it:

1. Correctly derives NodeId
2. Enforces epoch monotonicity
3. Validates key rotations
4. Applies trust decay rules
5. Rejects invalid merges

---

## 13. Relationship to Other RFCs

This document is normative with respect to:
- OpenNet Core RFC
- Trust Graph RFC
- Resolver Behavior RFC
- Security Extensions RFC

---

## 14. Conclusion

The Identity Lifecycle defined here prevents long-term decay of decentralized trust.

Without this RFC, OpenNet would remain functional but fragile.

With it, OpenNet becomes **durable, evolvable, and secure over decades**.

---

**End of RFC**

