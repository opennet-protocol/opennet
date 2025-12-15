# OpenNet CBOR & TLV Schemas RFC

## Canonical Binary Encoding and Wire Compatibility for OpenNet

Category: Standards Track
Status: Draft

---

## Abstract

This document defines the **canonical binary encoding rules** for the OpenNet protocol using **CBOR** and **TLV** schemas. It specifies exact field identifiers, encoding constraints, canonicalization rules, and registry management required to guarantee **wire-level compatibility** across all OpenNet implementations. Any deviation from this RFC results in a non-compliant implementation.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Design Goals

The encoding layer MUST guarantee:

- Deterministic binary representation
- Cross-language compatibility
- Forward extensibility
- Zero ambiguity at wire level

Human readability is NOT a goal.

---

## 3. Encoding Model Overview

OpenNet uses a **hybrid model**:

- CBOR for structured payloads
- TLV for low-level framing and extensions

```
[ TLV(Frame) ] → contains → [ CBOR(Payload) ]
```

---

## 4. Canonical CBOR Rules (Normative)

All OpenNet CBOR payloads MUST:

1. Use definite-length encoding
2. Use integer map keys ONLY
3. Sort map keys in ascending order
4. Reject duplicate keys
5. Use minimal integer encoding
6. Disallow floating point values

Non-canonical CBOR MUST be rejected.

---

## 5. Global CBOR Key Registry

### 5.1 Reserved Key Space

| Range | Usage |
|------|------|
| 0–31 | Core protocol |
| 32–127 | Wire / transport |
| 128–255 | Resolver |
| 256–511 | Security |
| 512+ | Experimental |

---

### 5.2 Core CBOR Keys

| Key | Name | Type |
|----|------|------|
| 0 | msg_type | uint |
| 1 | node_id | bytes |
| 2 | service_id | text |
| 3 | domain | text |
| 4 | scope | text |
| 5 | timestamp | uint |
| 6 | ttl | uint |

---

## 6. TLV Framing Layer

### 6.1 TLV Structure

```
TLV := [ Type (uint16) | Length (uint32) | Value (bytes) ]
```

- Network byte order (big-endian)
- Length MUST match Value size exactly

---

### 6.2 TLV Type Registry

| Type | Name |
|------|------|
| 0x0001 | FRAME_HEADER |
| 0x0002 | CBOR_PAYLOAD |
| 0x0003 | SIGNATURE |
| 0x0004 | NODE_ID |
| 0x00FF | EXTENSION |

---

## 7. Frame Composition Rules

A valid OpenNet frame MUST contain TLVs in this order:

1. FRAME_HEADER
2. CBOR_PAYLOAD
3. SIGNATURE

Any deviation MUST be rejected.

---

## 8. Signature Placement and Coverage

### 8.1 Signed Data

The SIGNATURE TLV MUST cover:

```
FRAME_HEADER || CBOR_PAYLOAD
```

---

### 8.2 Algorithm Binding

Signature algorithm identifiers MUST be included in FRAME_HEADER.

---

## 9. Extensibility Rules

- Unknown CBOR keys MUST be ignored if marked optional
- Unknown TLV types MUST be skipped unless critical
- Critical extensions MUST be explicitly flagged

---

## 10. Versioning

### 10.1 Schema Version Field

FRAME_HEADER MUST include:

- wire_version
- schema_version

---

### 10.2 Backward Compatibility

New schemas MUST NOT reinterpret existing keys.

---

## 11. Error Handling

| Condition | Action |
|---------|--------|
| Invalid CBOR | REJECT |
| Invalid TLV length | ABORT |
| Unknown critical TLV | ABORT |

---

## 12. Testability Requirements

- All schemas MUST have reference test vectors
- Canonical encoding MUST round-trip

---

## 13. Reference Schema Files

Schema definitions MUST be published as:

- JSON (human-readable)
- CBOR (machine canonical)

---

## 14. Security Considerations

Canonical encoding prevents:

- Signature malleability
- Ambiguous parsing attacks
- Downgrade vectors

---

## 15. Relationship to Other OpenNet RFCs

This RFC:

- Defines the wire layer for OpenNet Core RFC
- Is consumed by Rust Reference Node RFC
- Is validated by Compliance Test Vectors RFC

---

## 16. Conclusion

The CBOR & TLV Schemas RFC locks the OpenNet wire format into a precise, deterministic, and future-proof binary specification. This document ensures that all OpenNet nodes speak exactly the same language at the byte level, eliminating ambiguity and enabling long-term protocol stability.
