# OpenNet Resolver Daemon RFC

## Operating System and Browser Level Name Resolution for OpenNet

Category: Standards Track
Status: Draft

---

## Abstract

This document defines the **OpenNet Resolver Daemon**, a system-level service responsible for resolving `open://` and `service://` URIs into concrete OpenNet node endpoints. The resolver daemon is the critical integration layer that makes OpenNet usable by operating systems, browsers, and applications without centralized DNS infrastructure. This RFC specifies resolver behavior, APIs, caching rules, security constraints, and OS/browser integration requirements.

---

## 1. Requirements Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 2. Purpose and Role of the Resolver Daemon

The OpenNet Resolver Daemon (ORD):

- Replaces traditional DNS resolution for OpenNet namespaces
- Resolves `open://` and `service://` URIs
- Performs deterministic node selection
- Enforces scope and trust rules
- Exposes resolution results to local applications

The resolver daemon is **mandatory** for user-facing OpenNet deployments.

---

## 3. Resolver Architecture Overview

```
Application / Browser
        ↓
URI Handler (open://, service://)
        ↓ IPC
OpenNet Resolver Daemon
        ↓
OpenNet Network (P2P)
```

The resolver daemon MUST run as a background system service.

---

## 4. Supported URI Schemes

### 4.1 open:// URI

```
open://<service-id>.<domain>/<path>
```

Example:

```
open://chat.opennet/room/123
```

---

### 4.2 service:// URI

```
service://<scope>.<service>.<domain>
```

Example:

```
service://eu.chat.opennet
```

---

## 5. Resolution Workflow

The resolver daemon MUST execute the following steps:

1. Parse and validate URI syntax
2. Normalize domain and service identifiers
3. Query OpenNet network for service records
4. Apply scope filtering
5. Rank candidate nodes deterministically
6. Return ordered node list or best node

---

## 6. Deterministic Node Selection

### 6.1 Input Set

Node ranking MUST consider:

- Trust score
- Network latency
- Scope match
- Availability history

---

### 6.2 Deterministic Ranking Rule

Given the same input set, the resolver MUST produce the same node ordering.

Randomness MUST NOT be used.

---

## 7. Scope Resolution Rules

Scopes define logical network partitions (e.g., `eu`, `anon`).

Rules:

- Exact scope match MUST be preferred
- Fallback scopes MAY be used if allowed
- Scope conflicts MUST NOT leak traffic across restricted scopes

---

## 8. Caching and TTL

### 8.1 Cache Entries

Resolver cache entries include:

- Service ID
- Node list
- Trust snapshot
- Expiration time

---

### 8.2 Cache Invalidation

Cache entries MUST be invalidated when:

- TTL expires
- Trust score drops below threshold
- Node becomes unreachable

---

## 9. Security Requirements

### 9.1 Local Security

- Resolver MUST run with least privilege
- IPC interface MUST be access-controlled

---

### 9.2 Network Security

- All resolver queries MUST be signed
- Responses MUST be verified

---

## 10. IPC API Specification

### 10.1 Resolve Request (Abstract)

```
RESOLVE {
  uri: "open://chat.opennet",
  flags: [BEST, ALL]
}
```

---

### 10.2 Resolve Response

```
RESOLVED {
  nodes: [NodeId...],
  metadata: {...}
}
```

---

## 11. OS Integration

### 11.1 Linux

- systemd service
- UNIX domain socket
- xdg-open integration

---

### 11.2 Windows

- Background service
- Named pipe IPC
- Protocol handler registration

---

### 11.3 macOS

- launchd service
- XPC IPC
- URL scheme registration

---

## 12. Browser Integration

### 12.1 Native Browsers

Browsers SHOULD:

- Delegate `open://` handling to OS resolver
- Use resolver results for connection bootstrapping

---

### 12.2 Extensions

If native integration is unavailable, browser extensions MAY act as resolver clients.

---

## 13. Failure Handling

- Resolution failure MUST return explicit error codes
- Partial results MAY be returned if allowed

---

## 14. Privacy Considerations

- Resolver MUST minimize metadata leakage
- Cached queries SHOULD be ephemeral

---

## 15. Relationship to Other OpenNet RFCs

This RFC:

- Depends on OpenNet Core RFC
- Depends on OpenNet Resolver Behavior RFC
- Is validated by Compliance Test Vectors RFC

---

## 16. Implementation Guidance

A minimal resolver daemon SHOULD be under 10k LOC.

---

## 17. Conclusion

The OpenNet Resolver Daemon is the bridge between decentralized networking theory and practical user experience. By integrating directly with operating systems and browsers, it enables OpenNet to function as a real, censorship-resistant, and user-accessible internet.

