# OpenNet Whitepaper

## A Service-Centric, Node-Agnostic, Censorship-Resistant Internet Protocol

---

## Abstract

OpenNet is a fully decentralized Internet architecture designed to replace the traditional IP/DNS-centric model. Instead of binding domains to servers or IP addresses, OpenNet binds domains to **services**, represented by cryptographic **ServiceIDs** and delivered by dynamic sets of independent nodes. This whitepaper defines the OpenNet architecture, addressing model, URI schemes (`open://`, `service://`), resolution algorithms, routing behavior, security model, and the complete set of protocol message types. The goal is a global, permissionless, non-ownable Internet where the same domain can be served simultaneously by many computers without centralized control.

---

## 1. Introduction

The current Internet is built on a fragile chain:

```
Domain → DNS Authority → IP Address → Server
```

This design enables censorship, domain seizure, centralized surveillance, and single points of failure. OpenNet breaks this chain by redefining what a domain represents and how services are reached.

OpenNet does not address machines. It addresses **services**.

---

## 2. Design Principles

OpenNet is built on the following core principles:

- No central authority
- No domain ownership
- No IP dependency
- Services are collective, not proprietary
- Nodes are ephemeral and replaceable
- The network must self-heal
- Resolution and routing are client-side decisions

---

## 3. Core Architecture Model

```
Domain
  ↓
ServiceID
  ↓
Node Set
  ↓
Route
  ↓
Stream
```

- **Domain**: Human-readable identifier
- **ServiceID**: Cryptographic identifier of a service
- **Node Set**: Independent nodes providing the same service
- **Route**: Dynamic peer-to-peer path
- **Stream**: Encrypted end-to-end data channel

---

## 4. Addressing and URI Scheme

### 4.1 open:// URI (Connection)

```
open://serviceid.domain
```

Meaning: Connect to the most suitable node providing the given service under the specified domain.

Examples:
```
open://chat.opennet
open://eu.chat.opennet
open://9fa3e1.example.com
```

---

### 4.2 service:// URI (Discovery)

```
service://scope.serviceid.domain
```

Meaning: Discover and list all active nodes providing the specified service.

---

## 5. ServiceID

The ServiceID is the **true identity** of a service in OpenNet.

Properties:
- Cryptographically derived (hash-based)
- Globally unique
- Non-ownable
- Shareable by multiple nodes
- Independent of domain naming

---

## 6. Scope Layer (Region / Network)

Multiple service clusters may exist under the same domain:

```
chat.opennet
 ├─ svc:AAA (EU)
 └─ svc:BBB (ANON)
```

User interaction examples:
```
open://chat.opennet        → automatic selection
open://eu.chat.opennet     → EU scope
open://anon.chat.opennet   → anonymous network
```

---

## 7. Node Model

Each node in OpenNet:

- Is identified by a public key
- Operates independently
- May provide multiple services
- May forward traffic for other nodes
- Is not authoritative for any domain

---

## 8. Service Participation

A node joins a service by announcing:

```
SERVICE_JOIN serviceid domain
```

Characteristics:
- No central registry
- Gossip-based propagation
- Time-to-live (TTL) enforced

---

## 9. Resolution Process

When a client resolves:

```
open://serviceid.domain
```

The following steps MUST occur:

1. Parse the URI
2. Resolve the domain via distributed gossip
3. Match and validate the ServiceID
4. Apply scope constraints
5. Retrieve active node set
6. Rank nodes locally
7. Establish a secure stream

---

## 10. Node Selection Algorithm

Node selection is performed entirely on the client side.

Selection criteria include:
- Network latency
- Current load
- Trust / reputation score
- Route depth

No node is globally preferred or authoritative.

---

## 11. Routing and Forwarding Model

- If a node has a direct service route, it responds locally
- If not, it forwards the request to connected peers
- If a root or optimized route is discovered, it is reused
- Routing tables are adaptive and temporary

---

## 12. Security Model

OpenNet security is based on cryptography, not authority:

- Public-key node identities
- Mandatory message signatures
- End-to-end encryption
- Trust graphs and local reputation

There is no global blacklist or censorship layer.

---

## 13. Censorship Resistance

A domain in OpenNet cannot be shut down because:

- There is no single DNS record
- There is no central resolver
- Services are provided by many independent nodes
- Knowledge propagates via gossip

Shutting down nodes does not shut down services.

---

## 14. Protocol Message Types

### 14.1 Node Lifecycle Messages
- NODE_HELLO
- NODE_WELCOME
- NODE_GOODBYE
- NODE_STATUS
- NODE_PING
- NODE_PONG

### 14.2 Peer Discovery Messages
- PEER_DISCOVER
- PEER_OFFER
- PEER_ACCEPT
- PEER_REJECT

### 14.3 Service Management Messages
- SERVICE_ANNOUNCE
- SERVICE_JOIN
- SERVICE_LEAVE
- SERVICE_HEARTBEAT

### 14.4 Domain Resolution Messages
- DNS_REGISTER
- DNS_LOOKUP
- DNS_RESPONSE
- DNS_FORWARD

### 14.5 Routing Messages
- ROUTE_REQUEST
- ROUTE_RESPONSE
- ROUTE_FORWARD
- ROUTE_FAIL

### 14.6 Forwarding Messages
- NODE_REQUEST_FORWARD
- NODE_FORWARD_ACK
- NODE_FORWARD_DENY

### 14.7 Stream Messages
- STREAM_OPEN
- STREAM_ACCEPT
- STREAM_DATA
- STREAM_CLOSE
- STREAM_ABORT

### 14.8 Security Messages
- AUTH_CHALLENGE
- AUTH_RESPONSE
- SIGNATURE_VERIFY
- TRUST_ANNOUNCE

### 14.9 Error Messages
- ERROR_ROUTE_NOT_FOUND
- ERROR_SERVICE_UNAVAILABLE
- ERROR_NODE_UNREACHABLE

---

## 15. Normative Language (RFC 2119)

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

---

## 16. Resolver and Node Selection Pseudocode

```pseudo
function resolve(uri):
    parts = parseURI(uri)
    services = lookupDomain(parts.domain)
    service = matchServiceID(services, parts.serviceid)

    nodes = getActiveNodes(service)
    nodes = applyScope(nodes, parts.scope)

    ranked = rank(nodes)
    selected = ranked[0]

    return connect(selected)
```

---

## 17. Message Flow Examples

### Service Discovery
```
Client → Network : service://chat.opennet
Network → Client : node list
```

### Connection Establishment
```
Client → Node : STREAM_OPEN
Node → Client : STREAM_ACCEPT
```

### Forwarding
```
Node A → Node B : NODE_REQUEST_FORWARD
Node B → Node A : NODE_FORWARD_ACK
```

---

## 18. Sybil and Spam Resistance

OpenNet mitigates Sybil and spam attacks without central control:

- Node identities have a cryptographic cost
- Trust is earned gradually and decays over time
- New identities have no inherited reputation
- Rate limits are applied locally
- Network topology naturally deprioritizes isolated clusters

---

## 19. Browser and OS Integration

### Browser Integration

- `open://` and `service://` are registered URI schemes
- Browsers resolve OpenNet URIs internally or via a local daemon
- Users may manually select services when ambiguity exists

### Operating System Integration

- OpenNet runs as a background system service
- Applications communicate via local APIs or IPC
- No kernel modification is required

---

## 20. Reference Implementation Guidelines

### Go
- High concurrency via goroutines
- Fast prototyping

### Rust
- Memory safety
- Async networking (Tokio)
- Production-grade nodes

### C++
- High-performance routing
- Embedded and edge deployments

All implementations MUST conform to this specification.

---

## 21. Final Statement

OpenNet is not an overlay, proxy, or extension of the existing Internet. It is a replacement architecture based on services instead of servers, identities instead of IP addresses, and collective participation instead of centralized ownership.

This document defines the foundation of a truly free, decentralized Internet.

