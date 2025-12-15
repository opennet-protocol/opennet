# OpenNet Compliance Test Vectors RFC

## Protocol Correctness and Conformance Validation

Category: Informational / Standards Support
Status: Draft

---

## Abstract

This document defines the **OpenNet Compliance Test Vectors**, a normative test specification used to verify correctness, determinism, and protocol conformance of OpenNet implementations. These test vectors answer the question: *“Is this node a correct OpenNet node?”* Implementations claiming OpenNet compatibility MUST pass all mandatory tests defined herein.

This RFC is non-executable but machine-consumable and is designed to be used alongside the OpenNet Rust Reference Node RFC.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Scope and Purpose

The Compliance Test Vector suite validates:

- Wire format correctness
- Cryptographic verification
- Deterministic resolver behavior
- Trust graph computation
- State machine enforcement

This document does NOT define protocol behavior; it verifies it.

---

## 3. Conformance Levels

### 3.1 Mandatory Compliance

An implementation MUST:
- Pass all tests marked **MANDATORY**
- Reject all invalid vectors
- Produce deterministic outputs

### 3.2 Optional Compliance

Tests marked **OPTIONAL** MAY be skipped by constrained implementations but MUST be declared.

---

## 4. Test Vector Format

### 4.1 Canonical Structure

Each test vector is defined as a canonical CBOR object:

```
{
  0: test_id,
  1: description,
  2: input_frames,
  3: expected_result,
  4: flags
}
```

All numeric keys MUST be canonical and sorted.

---

### 4.2 Result Codes

| Code | Meaning |
|-----|--------|
| ACCEPT | Input valid, state advanced |
| DROP | Input ignored |
| REJECT | Input invalid |
| ABORT | Fatal protocol violation |

---

## 5. Wire Format Test Vectors

### 5.1 Valid Frame (MANDATORY)

**Input:**

```
{
  1: 0x01,         // NODE_HELLO
  2: <valid CBOR payload>,
  3: <valid signature>
}
```

**Expected Result:** ACCEPT

---

### 5.2 Oversized Frame (MANDATORY)

- Payload exceeds max frame size

**Expected Result:** REJECT

---

### 5.3 Non-Canonical CBOR (MANDATORY)

- Keys out of order

**Expected Result:** DROP

---

## 6. Cryptographic Test Vectors

### 6.1 Valid Signature (MANDATORY)

- Ed25519 signature matches payload

**Expected Result:** ACCEPT

---

### 6.2 Invalid Signature (MANDATORY)

- Signature mismatch

**Expected Result:** ABORT

---

### 6.3 Wrong Public Key (MANDATORY)

- Payload signed by unknown key

**Expected Result:** REJECT

---

## 7. State Machine Compliance Tests

### 7.1 Invalid Transition (MANDATORY)

- STREAM_DATA before HANDSHAKE

**Expected Result:** ABORT

---

### 7.2 Duplicate Handshake (OPTIONAL)

- Replayed NODE_HELLO

**Expected Result:** DROP

---

## 8. Resolver Determinism Tests

### 8.1 Deterministic Ranking (MANDATORY)

Given identical resolver input sets:

- Node ordering MUST be identical

**Expected Result:** ACCEPT

---

### 8.2 Scope Conflict Resolution (MANDATORY)

- service://eu.chat.opennet vs service://anon.chat.opennet

**Expected Result:** Correct scope selection

---

## 9. Trust Graph Test Vectors

### 9.1 Trust Increase (MANDATORY)

- Successful interactions increase trust

**Expected Result:** Trust score > previous

---

### 9.2 Trust Decay (MANDATORY)

- Inactive peer over time

**Expected Result:** Trust score decreases

---

### 9.3 Sybil Resistance Simulation (OPTIONAL)

- Multiple nodes sharing graph edges

**Expected Result:** No trust amplification

---

## 10. Stream Layer Tests

### 10.1 In-Order Delivery (MANDATORY)

- Sequential STREAM_DATA frames

**Expected Result:** Ordered stream

---

### 10.2 Replay Attack (MANDATORY)

- Duplicate stream sequence number

**Expected Result:** DROP

---

## 11. Error Handling Tests

- Malformed payload
- Unknown message type
- Invalid flags

**Expected Result:** DROP or ABORT as defined

---

## 12. Test Execution Rules

- Tests MUST be stateless unless specified
- Time-dependent tests MUST mock time
- Randomness MUST be seeded

---

## 13. Reporting Format

Implementations SHOULD output:

```
TEST_ID | RESULT | NOTES
```

---

## 14. Reference Implementation Alignment

All test vectors MUST pass on the OpenNet Rust Reference Node.

Failure indicates a bug in the specification or implementation.

---

## 15. Versioning and Evolution

- Test vectors are versioned independently
- New tests MAY be added
- Existing tests MUST NOT change semantics

---

## 16. Security Considerations

Compliance tests help detect:
- Protocol downgrade attacks
- Malformed input attacks
- State desynchronization

They do NOT guarantee full security.

---

## 17. Relationship to Other OpenNet RFCs

This RFC:
- Validates OpenNet Core RFC
- Validates OpenNet Wire RFC
- Validates Rust Reference Node RFC

---

## 18. Conclusion

The OpenNet Compliance Test Vectors RFC establishes a definitive, objective standard for correctness. Any implementation passing these tests can credibly claim OpenNet compatibility, ensuring a healthy, interoperable, and fork-resistant ecosystem.

