# 🌐 OpenNet Resolver → FSM → Transport Integration RFC

> **Status:** Normative – Execution Flow Binding

This document specifies the **end-to-end execution path** for processing an `open://` request inside an OpenNet full node, binding **Resolver**, **FSM**, and **Transport** into a deterministic pipeline.

RFC 2119 keywords apply.

---

## 1. Purpose

This RFC answers the question:

> **How does an ACTIVE OpenNet node process an `open://service.domain/path` request?**

It defines:
- Resolver invocation rules
- FSM gating conditions
- Transport selection and failure semantics
- Event emission on failure

This document is mandatory for request-level correctness.

---

## 2. Preconditions (Normative)

An OpenNet node MUST satisfy all of the following before processing requests:

- NodeState == `ACTIVE`
- Identity == valid and non-revoked
- Epoch drift < D_warn
- Trust score ≥ T_ok

If any precondition fails, request processing MUST be aborted.

---

## 3. High-Level Execution Flow

```text
open:// URI
   ↓
Resolver
   ↓
Trust Evaluation
   ↓
FSM Gate
   ↓
Transport Selection
   ↓
Connection Attempt
   ↓
Response / Failure
```

---

## 4. Resolver Phase

### 4.1 URI Parsing

- URI scheme MUST be `open://`
- ServiceId, domain, and scope MUST be extracted
- Invalid syntax MUST abort immediately

### 4.2 Service Resolution

Resolver MUST:
- Perform deterministic node ranking
- Filter nodes by scope (e.g. `service://eu.chat.opennet`)
- Return an ordered candidate list

Resolver MUST NOT:
- Open sockets
- Perform retries

---

## 5. FSM Gate

Before transport is attempted:

- NodeState MUST be rechecked
- If state != ACTIVE → abort request
- No FSM transition occurs on success

FSM interaction is **read-only** in the success path.

---

## 6. Transport Selection

### 6.1 Transport Choice

- QUIC preferred if supported
- TCP fallback allowed
- Choice MUST be deterministic

### 6.2 Attempt Semantics

- Candidates tried in resolver order
- First success terminates search

---

## 7. Failure Semantics → FSM Events

Failures MAY emit FSM events:

| Failure Condition | FSM Event |
|---|---|
| All candidates unreachable | `TrustBelowWarn` |
| Transport handshake failure | `TrustBelowWarn` |
| Cryptographic failure | `SecurityViolation` |
| Repeated failures (threshold) | `TrustCritical` |

FSM events are emitted **after request termination**.

---

## 8. Success Semantics

On success:

- No FSM event emitted
- Trust MAY be slightly reinforced (optional, bounded)

---

## 9. Determinism Requirements

- Resolver ordering MUST be stable
- Retry counts MUST be fixed
- No adaptive heuristics allowed

---

## 10. Rust Binding Requirements

In `opennet-node`:

```text
handle_open_request(uri):
  if state != ACTIVE → return Error
  candidates = resolver.resolve(uri)
  for node in candidates:
    if transport.connect(node):
      return Success
  emit FSM event
  return Failure
```

Resolver MUST NOT call transport directly.

---

## 11. Error Codes (Normative)

- ERR_NODE_INACTIVE
- ERR_RESOLUTION_FAILED
- ERR_TRANSPORT_FAILED
- ERR_SECURITY_VIOLATION

Error codes MUST be stable.

---

## 12. Security Considerations

This binding prevents:
- Resolver-triggered DoS
- FSM bypass attacks
- Transport-level trust poisoning

---

## 13. Compliance Criteria

A node is compliant if:

- It enforces FSM gating
- It emits correct FSM events on failure
- It preserves deterministic ordering

---

## 14. Conclusion

This RFC completes the **request execution pipeline** of OpenNet.

With this document, `open://` becomes a fully defined, protocol-level operation.

---

**End of Document**
