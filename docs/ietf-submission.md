# OpenNet Internet-Draft Finalization RFC

## Status
Draft

## Abstract
This document defines the official plan to transition the OpenNet protocol suite into IETF Internet-Draft form. It specifies document partitioning, normative versus informative classification, versioning strategy, submission workflow, and long-term evolution rules. This RFC represents the final step from protocol design to formal standardization.

RFC 2119 keywords apply.

---

## 1. Objectives
The goals of this document are:
- Prepare OpenNet for IETF Internet-Draft submission
- Ensure modular, reviewable documents
- Preserve protocol coherence across drafts
- Define a no-authority evolution model

---

## 2. Draft Set Overview
The OpenNet protocol SHALL be submitted as a coordinated draft set:

### 2.1 Core Drafts (Normative)

1. **draft-opennet-core-protocol**
   - Architecture overview
   - Node definition
   - Message taxonomy

2. **draft-opennet-wire-format**
   - CBOR / TLV schemas
   - Canonical encoding rules

3. **draft-opennet-identity**
   - Identity lifecycle
   - Epoch semantics

4. **draft-opennet-revocation**
   - Revocation & recovery

5. **draft-opennet-trust**
   - Trust graph
   - Trust weight dynamics

6. **draft-opennet-resolver**
   - open:// URI semantics
   - Resolver behavior

7. **draft-opennet-transport**
   - QUIC / TCP binding
   - Admission rules

8. **draft-opennet-integration**
   - Resolver–Trust–Transport binding
   - Full-node behavior

---

## 3. Supporting Drafts (Normative)

- **draft-opennet-compliance**
  - Compliance criteria
  - Test vectors

- **draft-opennet-interoperability**
  - Cross-language determinism

---

## 4. Informative Drafts

- **draft-opennet-governance**
  - No-authority upgrade model

- **draft-opennet-deployment**
  - Deployment profiles

- **draft-opennet-security-considerations**
  - Threat analysis summary

---

## 5. Normative vs Informative Rules

- All MUST/SHOULD/MAY statements appear only in normative drafts
- Informative drafts MUST NOT introduce protocol requirements

---

## 6. Versioning Strategy

### 6.1 Protocol Versioning
- Major version increments indicate breaking changes
- Minor versions add backward-compatible extensions

### 6.2 Draft Versioning
- Draft revisions increment independently
- Draft set versions are logically grouped

---

## 7. Backward Compatibility

- Backward-incompatible changes MUST be explicit
- Nodes MUST reject unknown major versions

---

## 8. Submission Workflow

1. Internal review freeze
2. Reference implementation validation
3. Draft formatting per IETF guidelines
4. Initial Internet-Draft submission
5. Working group discussion (if applicable)
6. Iterative revisions

---

## 9. Change Control

- No single entity controls the protocol
- Changes require:
  - Draft update
  - Reference implementation alignment
  - Updated test vectors

---

## 10. Security Disclosure Policy

- Coordinated disclosure recommended
- Draft updates precede public deployment

---

## 11. Long-Term Evolution

- Experimental extensions allowed via draft suffixes
- Stable core evolves slowly

---

## 12. Exit Criteria

OpenNet is considered standard-ready when:
- All core drafts reach stable revision
- Two independent interoperable implementations exist
- Compliance test suite passes

---

## 13. IANA Considerations
None.

---

## 14. References
- RFC 2119
- IETF Internet-Draft Guidelines

