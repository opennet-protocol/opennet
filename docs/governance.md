# OpenNet Governance & Internet-Draft RFC

## Standards Governance, Versioning, and Internet-Draft Publication Model for OpenNet

Category: Standards Track (Process & Governance)
Status: Draft

---

## Abstract

This document defines the **governance, evolution, and standardization process** for the OpenNet protocol suite. It specifies how OpenNet specifications are authored, versioned, published as IETF Internet-Drafts, evolved without central authority, and maintained for long-term interoperability. This RFC ensures that OpenNet remains decentralized not only at the network level, but also at the standards governance level.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Governance Principles

OpenNet governance is founded on the following non-negotiable principles:

1. No central authority
2. Open participation
3. Deterministic evolution
4. Backward compatibility by default
5. Implementation-driven validation

Any governance mechanism violating these principles is non-compliant.

---

## 3. Document Taxonomy

### 3.1 Specification Classes

OpenNet specifications are classified as:

- **Core RFCs** (protocol semantics)
- **Wire RFCs** (binary encoding)
- **Behavior RFCs** (resolver, routing, trust)
- **Implementation RFCs** (reference nodes)
- **Test RFCs** (compliance and vectors)
- **Governance RFCs** (this document)

---

### 3.2 Normative vs Informational

- Normative RFCs define required behavior
- Informational RFCs provide guidance or examples

---

## 4. Internet-Draft Structure

### 4.1 Naming Convention

All OpenNet drafts MUST follow:

```
draft-opennet-<topic>-<version>.txt
```

Example:

```
draft-opennet-wire-00.txt
```

---

### 4.2 Versioning Rules

- Draft versions increment sequentially
- Breaking changes require a new major topic draft

---

## 5. Versioning Model

### 5.1 Protocol Versions

Each OpenNet protocol component MUST expose:

- protocol_version
- wire_version
- schema_version

---

### 5.2 Semantic Meaning

- Major: incompatible change
- Minor: backward-compatible extension
- Patch: editorial or clarification

---

## 6. Backward Compatibility Rules

- Existing message semantics MUST NOT change
- Deprecated features MUST remain supported for at least one major cycle
- Silent behavior changes are forbidden

---

## 7. No-Authority Upgrade Model

### 7.1 Upgrade Proposal Flow

1. Proposal drafted as Internet-Draft
2. Reference implementation updated
3. Compliance tests updated
4. Ecosystem adoption

No approval body exists.

---

### 7.2 Adoption as Authority

A feature is considered "standard" when:

- Multiple independent implementations support it
- Compliance tests pass
- Network usage is observable

---

## 8. Fork and Divergence Handling

- Forks are permitted
- Divergent behavior MUST change protocol identifiers
- Silent forks are prohibited

---

## 9. Reference Implementation Role

The Rust Reference Node:

- Is NOT authoritative
- Acts as behavioral baseline
- Detects spec ambiguity

---

## 10. Compliance as Governance

Passing compliance tests is the only valid claim of compatibility.

Marketing or claims without test results are meaningless.

---

## 11. Deprecation Policy

- Deprecations MUST be explicit
- Removal requires major version bump

---

## 12. Registry Management

### 12.1 CBOR / TLV Registries

Registries are maintained as:

- Public version-controlled repositories
- Immutable history

No private registry authority exists.

---

## 13. Security Incident Handling

- Vulnerabilities MUST be disclosed publicly
- Fixes MUST include compliance updates

---

## 14. Relationship to IETF

### 14.1 Internet-Draft Submission

OpenNet documents MAY be submitted to the IETF as:

- Independent submissions
- Research Group drafts

---

### 14.2 RFC Publication

IETF RFC publication does NOT grant control over OpenNet.

---

## 15. Intellectual Property

- All specifications MUST be royalty-free
- No patent encumbrance is permitted

---

## 16. Security Considerations

Decentralized governance reduces:

- Capture risk
- Political pressure
- Protocol ossification

---

## 17. Long-Term Sustainability

OpenNet evolution is driven by:

- Real deployments
- Open research
- Independent implementations

---

## 18. Conclusion

This Governance and Internet-Draft RFC completes the OpenNet specification framework. By defining a no-authority, implementation-driven, and transparent standards process, it ensures that OpenNet can evolve into a global, censorship-resistant, and vendor-neutral internet protocol suite.

