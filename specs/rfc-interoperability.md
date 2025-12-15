# OpenNet Cross-Implementation Interoperability RFC

## Status
Draft

## Abstract
This document specifies the interoperability requirements for multiple independent OpenNet implementations (e.g., Rust, Go, C++). It defines deterministic behavior, binary compatibility rules, canonical encoding, and cross-language conformance tests to ensure that different implementations of OpenNet behave identically under identical inputs.

RFC 2119 keywords (MUST, SHOULD, MAY) apply.

---

## 1. Goals
This RFC ensures that:
- Different language implementations interoperate seamlessly
- Identical inputs produce identical outputs (determinism)
- Binary messages are bit-for-bit compatible
- Resolver, Trust, and Transport decisions match across implementations

---

## 2. Scope
This RFC applies to:
- Full nodes
- Resolver daemons
- Reference libraries

It does NOT mandate implementation language or internal architecture.

---

## 3. Canonical Binary Encoding

### 3.1 Encoding Rules
All implementations MUST:
- Use canonical CBOR encoding
- Enforce deterministic map key ordering
- Reject non-canonical encodings

### 3.2 Integer & Numeric Types
- Signed integers MUST be encoded as minimal-length CBOR ints
- Floating-point numbers SHOULD NOT be used
- Trust weights MUST use fixed-point representation

---

## 4. Deterministic Sorting & Selection

The following operations MUST be deterministic:
- Resolver candidate ordering
- Trust weight sorting
- Node ranking and fallback order

Tie-breaking MUST use:

```
Hash(NodeId || EpochId || ContextSalt)
```

---

## 5. Trust Weight Conformance
Given identical:
- Trust graph
- Epoch states
- Decay parameters

All implementations MUST compute identical trust weights within tolerance ε = 0.

---

## 6. Resolver Conformance
Resolvers MUST:
- Produce identical candidate sets
- Apply identical scope resolution
- Enforce identical thresholds

Resolver output MUST be serialized deterministically for testing.

---

## 7. Transport Conformance
Transport implementations MUST:
- Bind sessions to identical (NodeId, Epoch)
- Enforce identical admission thresholds
- Produce identical failure classifications

---

## 8. Error Code Canonicalization
All implementations MUST expose standardized error codes:

- ERR_INVALID_EPOCH
- ERR_REVOKED_IDENTITY
- ERR_TRUST_TOO_LOW
- ERR_NON_CANONICAL_ENCODING
- ERR_PROTOCOL_VIOLATION

Error mapping MUST be lossless.

---

## 9. Compliance Test Vectors

### 9.1 Binary Test Vectors
- Canonical CBOR objects
- Invalid encoding cases

### 9.2 Resolver Test Vectors
- Input URI → ordered node list

### 9.3 Trust Test Vectors
- Graph snapshots → trust scores

### 9.4 Transport Test Vectors
- Handshake attempts → accept/reject

---

## 10. Golden Test Harness
Implementations MUST support a golden test harness:

- Input fixtures
- Expected outputs
- Deterministic replay

---

## 11. Cross-Language Test Procedure

1. Execute identical test vectors
2. Serialize outputs
3. Byte-compare results
4. Fail on any divergence

---

## 12. Floating-Point Prohibition
Floating-point arithmetic MUST NOT be used in:
- Trust computation
- Ranking
- Threshold enforcement

---

## 13. Time & Epoch Conformance
- Epoch validity decisions MUST match exactly
- Replay window enforcement MUST be identical

---

## 14. Backward Compatibility
Implementations MUST:
- Reject unsupported protocol versions
- Explicitly negotiate features

---

## 15. Certification Criteria
An implementation is interoperable if:
- All compliance vectors pass
- All golden tests match reference outputs

---

## 16. Security Considerations
- Prevents consensus divergence
- Eliminates implementation-specific attacks
- Reduces fingerprinting

---

## 17. Compatibility
Compatible with:
- Full-Node Behavior & Integration RFC
- Wire Format RFC
- Trust Weight Dynamics RFC
- Resolver RFC

---

## 18. IANA Considerations
None.

---

## 19. References
- RFC 2119
- OpenNet CBOR/TLV Schemas RFC
