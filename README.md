<p align="center">
  <img src="Opennet Assets/fulllogo.png" alt="OpenNet Logo" width="400">
</p>

# OpenNet: A Service-Centric, Decentralized Internet Protocol

## Comprehensive Technical Whitepaper

**Version 1.0 — December 2025**  
**Status: Draft — Pre-IETF Internet-Draft**

---

# Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Introduction: Why a New Internet Architecture?](#2-introduction-why-a-new-internet-architecture)
3. [Design Philosophy](#3-design-philosophy)
4. [Architectural Overview](#4-architectural-overview)
5. [Addressing and URI Schemes](#5-addressing-and-uri-schemes)
6. [Identity Lifecycle](#6-identity-lifecycle)
7. [Trust Graph](#7-trust-graph)
8. [Time and Epoch Semantics](#8-time-and-epoch-semantics)
9. [Revocation and Recovery](#9-revocation-and-recovery)
10. [Resolver Mechanism](#10-resolver-mechanism)
11. [Full-Node State Machine (FSM)](#11-full-node-state-machine-fsm)
12. [Wire Format: CBOR and TLV](#12-wire-format-cbor-and-tlv)
13. [Transport Layer](#13-transport-layer)
14. [Resolver-Trust-Transport Integration](#14-resolver-trust-transport-integration)
15. [Rust Reference Implementation](#15-rust-reference-implementation)
16. [Testing and Compliance](#16-testing-and-compliance)
17. [Cross-Implementation Interoperability](#17-cross-implementation-interoperability)
18. [Governance and Standardization](#18-governance-and-standardization)
19. [Security Analysis](#19-security-analysis)
20. [Conclusion and Future Vision](#20-conclusion-and-future-vision)
- [Appendix A: RFC Document List](#appendix-a-rfc-document-list)
- [Appendix B: Glossary](#appendix-b-glossary)

---

# 1. Executive Summary

OpenNet is a comprehensive protocol suite that fundamentally reimagines the IP/DNS-based internet architecture. Instead of the fragile "server → IP address → domain" chain of the traditional internet, OpenNet adopts a service-centric approach where domains are bound to cryptographic service identifiers rather than physical machines.

This whitepaper presents a comprehensive synthesis of the OpenNet specification set, consisting of 24 RFC (Request for Comments) documents. The goal is to consolidate all protocol components, design decisions, and implementation requirements into a single coherent document.

## Key Innovations

- **Service-Centric Addressing**: Domains bind to cryptographic ServiceIDs, not servers
- **No Central Authority**: No DNS registrars, certificate authorities, or governance bodies
- **Cryptographic Identity**: Ed25519-based node identities managed through epochs
- **Trust Graph**: Exponential decay mechanism resistant to Sybil and Eclipse attacks
- **Deterministic Behavior**: All implementations produce identical outputs for identical inputs
- **Censorship Resistance**: Technically impossible to shut down a single domain

## Target Audience

This document is prepared for protocol developers, security researchers, network engineers, standardization bodies, and anyone interested in decentralized systems.

---

# 2. Introduction: Why a New Internet Architecture?

## 2.1 Structural Problems of the Current Internet

Today's internet is built on an architecture designed in the 1970s, shaped by the needs of that era. This architecture was designed to connect trusted institutional networks and is not suitable for today's global, adversarial, and censorship-prone environment.

The fundamental weaknesses of the current internet architecture are:

1. **DNS Centralization**: Domain names depend on central registration authorities (registrars) and root servers. This structure is vulnerable to domain seizure, censorship, and single points of failure.

2. **IP Address Dependency**: Services are bound to physical servers and IP addresses. When a server is shut down, the service ends.

3. **Certificate Authority (CA) Trust**: TLS/SSL certificates depend on centralized CAs. These CAs can be hacked or compelled by governments to issue fraudulent certificates.

4. **Ease of Censorship**: Governments or ISPs can easily block websites through DNS manipulation, IP blocking, or BGP hijacking.

5. **Surveillance Infrastructure**: The centralized structure facilitates traffic monitoring and user profiling.

## 2.2 Inadequacy of Existing Solutions

Existing alternatives such as blockchain-based domain systems (Namecoin, ENS, Handshake), Tor network, and IPFS are important steps, but each has its own limitations:

- **Blockchain DNS**: Slow, difficult to scale, dependent on consensus mechanisms
- **Tor**: Anonymity-focused, performance degradation, .onion addresses hard to remember
- **IPFS**: Suitable for static content, inadequate for dynamic services

OpenNet presents a new approach that combines the advantages of existing solutions while addressing their shortcomings.

## 2.3 OpenNet's Core Promise

OpenNet is designed to answer the fundamental question: "How can we build an internet that no central authority can control, cannot be shut down, cannot be censored, and is always accessible?"

The answer lies in changing the fundamental addressing unit of the internet: addressing services instead of machines.

---

# 3. Design Philosophy

OpenNet's design is built on seven interconnected core principles. These principles are consistently applied across every layer and every RFC of the protocol.

## 3.1 No Central Authority

In OpenNet, no entity—company, government, or organization—has privileged control over the protocol. This principle encompasses:

- No domain registration authority
- No certificate authority
- No governance authority
- Protocol updates occur through organic adoption

## 3.2 Service-Centric Addressing

Traditional internet addresses machines (IP addresses). OpenNet addresses services. A domain is bound not to a specific server, but to a cryptographically defined service (ServiceID). This service can be simultaneously provided by hundreds of independent nodes.

## 3.3 Cryptographic Identity

Every node in OpenNet has a cryptographic identity based on an Ed25519 public key pair. The NodeId is derived as the SHA-256 hash of the public key, and this identity remains unchanged throughout the node's lifecycle. Keys can be rotated, but identity is preserved.

## 3.4 Determinism

One of OpenNet's most critical design decisions is complete determinism. Given the same inputs, different implementations (Rust, Go, C++) must produce bit-for-bit identical outputs. This principle:

- Prohibits floating-point arithmetic
- Controls random number generation
- Specifies sorting and selection algorithms
- Standardizes error codes

## 3.5 Trust Economics

In OpenNet, trust is treated as an economic resource. New nodes start with low trust, trust is earned over time and decays. Bad behavior is immediately punished. This model makes Sybil attacks economically deterrent.

## 3.6 Self-Healing

The network automatically detects and isolates faulty or malicious nodes. This happens without centralized moderation or management. Trust decay and quarantine mechanisms fulfill this function.

## 3.7 Client-Side Decision Making

Resolver and node selection occurs entirely on the client side. No server or network component can tell the client which node to connect to. This provides fundamental protection against Eclipse and MITM attacks.

---

# 4. Architectural Overview

## 4.1 Layered Architecture

OpenNet is built on a modular layered architecture. Each layer has a specific responsibility and communicates with other layers through defined interfaces.

| Layer | Responsibility |
|-------|---------------|
| **Application** | User applications and open:// URI processing |
| **Resolver** | Name resolution, node discovery and ranking |
| **Trust** | Trust graph management, score calculation |
| **Identity** | Identity lifecycle, epoch management |
| **Transport** | QUIC/TCP connections, flow control |
| **Wire** | CBOR/TLV binary encoding, signatures |

## 4.2 Data Flow

An open:// request passes through the following stages:

1. **URI Parsing**: `open://chat.opennet/room/123` → ServiceId, domain, path
2. **Resolver Call**: Candidate node list retrieved for domain
3. **Trust Evaluation**: Trust score calculated for each candidate node
4. **Node Ranking**: Ranking based on trust, latency, and diversity factors
5. **Transport Connection**: Encrypted connection established with highest-ranked node
6. **Data Stream**: End-to-end encrypted data transfer

## 4.3 Node Types

There are three main node types in the OpenNet network:

- **Full Node**: Implements all protocol layers. Responds to resolver queries, maintains trust graph, provides services.
- **Light Node**: Client functionality only. Connects to network through resolver daemon.
- **Relay Node**: Traffic routing. Used for NAT traversal and connection bridging.

---

# 5. Addressing and URI Schemes

## 5.1 ServiceID Concept

ServiceID is OpenNet's fundamental addressing unit and a service's true identity. Unlike traditional domain names, ServiceID:

- Is cryptographically derived (hash-based)
- Is globally unique
- Cannot be owned—any node can provide this service
- Is independent of domain naming

## 5.2 open:// URI Scheme

The open:// scheme is used to establish a connection to a service:

```
open://[scope.]serviceid.domain[/path]
```

**Examples:**
- `open://chat.opennet` — Automatic node selection
- `open://eu.chat.opennet` — European scope
- `open://anon.chat.opennet` — Anonymous network

## 5.3 service:// URI Scheme

The service:// scheme is used for service discovery:

```
service://scope.serviceid.domain
```

This scheme returns a list of all active nodes providing a specific service, rather than establishing a connection.

## 5.4 Scope Layer

Scope defines logical network partitions under the same domain. Use cases:

- **Geographic**: `eu`, `us`, `asia` — Regional routing for low latency
- **Privacy**: `anon`, `private` — Anonymous or private network
- **Test**: `dev`, `staging` — Development environments

---

# 6. Identity Lifecycle

Identity is OpenNet's most fundamental security primitive. The Identity Lifecycle RFC defines all stages of a node identity from birth to death.

## 6.1 Identity Model: Immutable Identity, Mutable Keys

OpenNet separates identity continuity from key material:

- **NodeId**: Immutable, permanent throughout lifetime. SHA-256 hash of canonical_public_key.
- **Key Pair**: Rotatable. Ed25519 mandatory, post-quantum optional.

## 6.2 Epoch Concept

An epoch represents a continuous validity interval of a key pair:

- **epoch_id**: Monotonically increasing number (u64)
- **start_time**: Epoch start time
- **max_duration**: Maximum validity duration
- **key_material_hash**: Hash of associated key material

Epoch numbers MUST strictly increase monotonically. Rollback is not accepted.

## 6.3 Key Rotation

Key rotation is performed by broadcasting a KeyRotation message signed with the old key:

1. New key pair is generated
2. KeyRotation message is created (NodeId, old epoch, new epoch, new public key)
3. Message is signed with old private key
4. Broadcast to network

In a valid rotation, trust score is preserved. Invalid rotation causes identity rejection.

## 6.4 Identity Merge and Split

To prevent trust laundering attacks:

- **Identity Merge**: Not automatically supported. Only possible through explicit protocol extension.
- **Identity Split**: Creates new NodeId. Trust is NOT transferred—starts from zero.

---

# 7. Trust Graph

The Trust Weight Dynamics RFC defines OpenNet's economic and cryptographic security model. This model creates a trust system resistant to Sybil, Eclipse, and reputation laundering attacks.

## 7.1 Mathematical Model

The trust graph is a directed weighted graph:

```
G = (V, E) where V = nodes, E = trust edges
```

Each edge e(u → v) has the following attributes:

- **weight**: w ∈ (0, 1] — Trust weight
- **epoch_created**: Epoch when edge was created
- **last_updated**: Last update time

## 7.2 Trust Calculation

A node's trust weight (TW) is in the range [0.0, 1.0]:

- 0.0 = No trust
- 1.0 = Maximum trust
- New nodes start with ε = 0.01

Trust propagation occurs through a multiplicative model:

```
TW(j) += TW(i) × w(i,j) × decay(age)
```

## 7.3 Exponential Decay

Trust decays exponentially over time:

```
decay(t) = e^(−λt)  where λ = 0.05 (default)
```

This function ensures:

- Long-lived nodes remain stable
- Inactive nodes gradually fade
- Sudden behavior changes are penalized

## 7.4 Thresholds

The Resolver and Transport layers enforce the following thresholds:

| Threshold | Default | Description |
|-----------|---------|-------------|
| ResolveThreshold | 0.2 | Minimum to include node in candidate list |
| ConnectThreshold | 0.3 | Minimum for connection attempt |
| RelayThreshold | 0.4 | Minimum for traffic relay |

## 7.5 Sybil Resistance Analysis

The trust system economically deters Sybil attacks:

- Cost to raise TW grows exponentially
- Trust cannot be rapidly accumulated
- Identity splitting resets accumulated trust
- Transitive trust is capped (τ upper bound)

---

# 8. Time and Epoch Semantics

The Time & Epoch Semantics RFC defines how time is handled in a fully decentralized environment. It provides consistent time semantics without dependence on centralized time authorities.

## 8.1 Time Sources

OpenNet nodes MAY use the following time sources:

- **System Clock**: Operating system clock
- **Monotonic Clock**: Non-decreasing counter
- **Network Median Time (NMT)**: Median of samples collected from peers

No node should rely solely on a single external time source.

## 8.2 Network Median Time (NMT)

Each node calculates NMT with the following formula:

```
NMT = median(peer_time_samples)
```

Samples are collected during authenticated handshakes. Outliers beyond drift tolerance are discarded.

## 8.3 Drift Tolerance

- Maximum acceptable drift: ±Δ = 5 minutes (default)
- Drift exceeding Δ causes trust decay

## 8.4 Replay Protection

Each message includes:

- **epoch_id**: Message's epoch
- **message_timestamp**: Message timestamp
- **nonce**: Unique number

Messages are rejected if: Timestamp is outside replay window OR nonce is reused within window.

```
Default replay window: 120 seconds
```

---

# 9. Revocation and Recovery

The Revocation & Recovery RFC defines identity revocation and optional recovery after key compromise or malicious behavior.

## 9.1 Threat Model

The revocation mechanism addresses the following threats:

- Private key compromise
- Forced key disclosure
- Long-term silent impersonation
- Trust laundering through identity reset

## 9.2 Revocation Triggers

A node MUST initiate revocation when:

- Private key is exposed
- Unauthorized signatures are detected
- Trust graph decay threshold is exceeded

Peers MAY initiate revocation votes when Byzantine behavior is observed or conflicting epochs are broadcast.

## 9.3 Revocation Object

A revocation is a signed CBOR object:

```
type = REVOCATION
node_id, revoked_epoch, reason_code
timestamp, signatures[]
```

## 9.4 Quorum Validation

Revocation is considered valid when:

- Node self-signs, OR
- A quorum of trusted peers co-sign

Quorum size is determined by trust weight, not raw count.

## 9.5 Recovery Process

Recovery is optional and proceeds as follows:

1. Node generates new keypair
2. Recovery declaration references revoked epoch
3. Trusted peers co-sign recovery
4. Trust graph links new epoch with reduced weight

**Important**: Recovery does NOT restore full historical trust.

---

# 10. Resolver Mechanism

The Resolver Daemon RFC defines a system-level service that resolves open:// and service:// URIs into concrete node endpoints.

## 10.1 Resolver Daemon Architecture

The OpenNet Resolver Daemon (ORD) replaces traditional DNS resolution:

```
Application → URI Handler → IPC → Resolver Daemon → OpenNet P2P Network
```

## 10.2 Resolution Workflow

The resolver daemon executes the following steps:

1. **URI Parsing and Validation**: Syntax checking
2. **Domain and ServiceID Normalization**: Consistent format
3. **OpenNet Network Query**: P2P query for service records
4. **Scope Filtering**: Geographic/logical constraints
5. **Deterministic Ranking**: Trust, latency, diversity
6. **Result Return**: Ordered node list or best node

## 10.3 Deterministic Node Selection

Node ranking score is calculated with the following formula:

```
RankScore = TW × ScopeAffinity × DiversityFactor
```

Where:
- **TW**: Trust Weight
- **ScopeAffinity ∈ [0,1]**: Scope match degree
- **DiversityFactor**: Factor penalizing same-cluster selection

Ranking MUST be stable and deterministic. For tie-breaking:

```
Hash(NodeId || EpochId || ContextSalt)
```

## 10.4 Operating System Integration

| Platform | Integration |
|----------|-------------|
| **Linux** | systemd service, UNIX domain socket, xdg-open integration |
| **Windows** | Background service, named pipe IPC, protocol handler |
| **macOS** | launchd service, XPC IPC, URL scheme registration |

---

# 11. Full-Node State Machine (FSM)

The Full-Node State Machine RFC defines the authoritative state machine and behavioral rules for an OpenNet full node. This is the "executable specification" of the protocol.

## 11.1 Node States

An OpenNet full node MUST be in exactly one of the following states:

| State | Description |
|-------|-------------|
| **BOOTSTRAP** | Initial startup. No trusted peers, no local trust graph. |
| **SYNCING** | Synchronization. Trust graph, epoch, and revocation data being retrieved. |
| **ACTIVE** | Fully operational. Resolves requests, routes traffic. |
| **DEGRADED** | Reduced capacity. Temporary failures detected. |
| **QUARANTINED** | Isolated. Revoked or unsafe state. |

## 11.2 State Transition Rules

State transitions MUST be deterministic and table-driven:

| From | To | Condition |
|------|-----|-----------|
| BOOTSTRAP | SYNCING | ≥1 valid peer discovered |
| SYNCING | ACTIVE | Synchronization completed |
| ACTIVE | DEGRADED | Trust < T_warn |
| DEGRADED | ACTIVE | Trust ≥ T_ok |
| ACTIVE | QUARANTINED | Trust < T_min |
| DEGRADED | QUARANTINED | SecurityViolation event |

**Critical**: Any transition not defined in the table MUST panic and fail tests.

## 11.3 FSM Event Producers

Only the following subsystems are authorized to produce FSM events:

- **IdentityWatcher**: Key validity, rotation, compromise
- **EpochMonitor**: Time drift, epoch validity
- **RevocationListener**: Revocation propagation
- **TrustEvaluator**: Trust score dynamics

## 11.4 FSM Events

```
PeerDiscovered
SyncCompleted
TrustBelowWarn
TrustRecovered
TrustCritical
SecurityViolation
```

## 11.5 Event Priority

When multiple events occur in the same processing window, they MUST be processed in priority order (highest first):

1. SecurityViolation
2. TrustCritical
3. TrustBelowWarn
4. TrustRecovered

---

# 12. Wire Format: CBOR and TLV

The CBOR & TLV Schemas RFC defines OpenNet's wire-level binary encoding rules. These rules ensure bit-level compatibility across different implementations.

## 12.1 Hybrid Encoding Model

OpenNet uses a hybrid model:

- **CBOR**: For structured payloads
- **TLV**: For low-level framing and extensions

```
[ TLV(Frame) ] → contains → [ CBOR(Payload) ]
```

## 12.2 Canonical CBOR Rules

All OpenNet CBOR payloads MUST enforce:

1. Use definite-length encoding
2. Use ONLY integer map keys
3. Sort map keys in ascending order
4. Reject duplicate keys
5. Use minimal integer encoding
6. Disallow floating-point values

**Critical**: Non-canonical CBOR MUST be rejected.

## 12.3 TLV Frame Structure

```
TLV := [ Type (uint16) | Length (uint32) | Value (bytes) ]
```

Network byte order (big-endian) is used. Length MUST exactly match Value size.

## 12.4 TLV Type Registry

| Type | Name |
|------|------|
| 0x0001 | FRAME_HEADER |
| 0x0002 | CBOR_PAYLOAD |
| 0x0003 | SIGNATURE |
| 0x0004 | NODE_ID |
| 0x00FF | EXTENSION |

## 12.5 Frame Composition

A valid OpenNet frame MUST contain TLVs in this order:

1. FRAME_HEADER
2. CBOR_PAYLOAD
3. SIGNATURE

Any deviation MUST be rejected.

## 12.6 Signature Coverage

The SIGNATURE TLV MUST cover:

```
FRAME_HEADER || CBOR_PAYLOAD
```

---

# 13. Transport Layer

The Transport layer establishes secure connections with nodes approved by the Resolver.

## 13.1 Supported Protocols

- **QUIC**: Preferred protocol. UDP-based, low latency, built-in encryption.
- **TCP**: Fallback. Used when QUIC is not supported.

## 13.2 Transport Responsibilities

The Transport layer MUST enforce:

- Accept only Resolver-approved nodes
- Reject connections below ConnectThreshold
- Bind sessions to (NodeId, Epoch) pair
- Terminate sessions when trust or epoch is invalidated
- Apply backpressure and flow control

## 13.3 Session Binding

Each transport session is bound to the following pair:

```
(NodeId, Epoch)
```

This binding ensures:

- Automatic session termination on epoch change
- Prevention of communication with revoked keys
- Protection against replay attacks

---

# 14. Resolver-Trust-Transport Integration

The Resolver-Trust-Transport Integration RFC defines the end-to-end processing of an open:// request.

## 14.1 Execution Pipeline

Request processing occurs in the following order:

```
open:// URI → Resolver → Trust Evaluation → FSM Gate → Transport → Secure Session
```

Each layer is authoritative only for its own domain.

## 14.2 Preconditions

The following conditions MUST be satisfied before processing a request:

- `NodeState == ACTIVE`
- Identity is valid and not revoked
- Epoch drift < D_warn
- Trust score ≥ T_ok

If any precondition fails, the request MUST be aborted.

## 14.3 Failure Semantics → FSM Events

Failures are converted to FSM events:

| Failure Condition | FSM Event |
|-------------------|-----------|
| All candidates unreachable | TrustBelowWarn |
| Transport handshake failure | TrustBelowWarn |
| Cryptographic failure | SecurityViolation |
| Repeated failures (threshold exceeded) | TrustCritical |

FSM events are emitted after request termination.

## 14.4 Success Semantics

On success:

- No FSM event is emitted
- Trust MAY be slightly reinforced (optional, bounded)

## 14.5 Determinism Requirements

- Resolver ordering MUST be stable
- Retry counts MUST be fixed
- No adaptive heuristics allowed

---

# 15. Rust Reference Implementation

The Rust Reference Node RFC and Freeze Plan define OpenNet's authoritative executable specification.

## 15.1 Design Principles

- **Minimalism**: Implement only what the RFC set requires
- **Determinism**: Same inputs MUST produce same outputs
- **Auditability**: Readable, small, dependency-minimized
- **Safety**: Rust's memory safety guarantees, no unsafe unless strictly required

## 15.2 Crate Structure

```
opennet-node/crates/
├─ opennet-core       # Core types: NodeId, ServiceId, Epoch
├─ opennet-wire       # CBOR / TLV canonical encoding
├─ opennet-identity   # Key lifecycle, rotation
├─ opennet-revocation # Revocation & recovery logic
├─ opennet-time       # Epochs, NMT
├─ opennet-trust      # Trust graph & weight dynamics
├─ opennet-resolver   # open:// URI resolution
├─ opennet-transport  # QUIC / TCP bindings
├─ opennet-node       # Full-node state machine
└─ opennet-tests      # Compliance & integration tests
```

## 15.3 RFC ↔ Code Mapping

Each crate maps directly to a specific RFC:

| Crate | Governing RFC |
|-------|---------------|
| opennet-identity | Identity Lifecycle RFC |
| opennet-revocation | Revocation & Recovery RFC |
| opennet-time | Time & Epoch Semantics RFC |
| opennet-trust | Trust Weight Dynamics RFC |
| opennet-resolver | Resolver RFC |
| opennet-transport | Transport RFC |
| opennet-node | Full-Node Behavior RFC |

## 15.4 Forbidden Constructs

The following constructs are PROHIBITED:

- `f32`, `f64` — Floating-point arithmetic
- `HashMap`, `HashSet` — Non-deterministic collections
- Direct system time access
- Unseeded randomness

## 15.5 Freeze Versioning

- `v1.0.0-alpha` — Architecture frozen
- `v1.0.0-beta` — Interoperability validated
- `v1.0.0` — Internet-Draft reference implementation

**Important**: After freeze, RFC text MUST conform to Rust behavior, not vice versa.

---

# 16. Testing and Compliance

The Compliance Test Vectors RFC and FSM Exhaustive Test Suite RFC define protocol correctness and compliance validation.

## 16.1 Test Philosophy

The test suite aims to answer the question: "Is it possible for an OpenNet node to behave incorrectly?"

The goal is to reduce this answer to NO within defined protocol bounds.

## 16.2 Test Categories

- **Wire Format Tests**: Frame parsing, CBOR canonicity
- **Cryptographic Tests**: Signature verification, invalid signature rejection
- **Resolver Tests**: Deterministic ranking, scope resolution
- **Trust Tests**: Decay correctness, threshold enforcement
- **FSM Tests**: Valid/invalid transitions, event priority
- **Integration Tests**: End-to-end pipeline, multi-node

## 16.3 Test Vector Format

Each test vector is a canonical CBOR object:

```
{
  0: test_id,
  1: description,
  2: input_frames,
  3: expected_result,
  4: flags
}
```

## 16.4 Result Codes

| Code | Meaning |
|------|---------|
| ACCEPT | Input valid, state advanced |
| DROP | Input ignored |
| REJECT | Input invalid |
| ABORT | Fatal protocol violation |

## 16.5 Compliance Criteria

An implementation is compliant if:

- All tests defined in the RFC pass
- No undefined behavior is observed

---

# 17. Cross-Implementation Interoperability

The Cross-Implementation Interoperability RFC defines interoperability requirements for different language implementations.

## 17.1 Goals

- Different language implementations interoperate seamlessly
- Identical inputs produce identical outputs (determinism)
- Binary messages are bit-for-bit compatible
- Resolver, Trust, and Transport decisions match across implementations

## 17.2 Conformance Requirements

All implementations (Rust, Go, C++) MUST ensure:

- **Canonical CBOR Encoding**: Deterministic map key ordering
- **Fixed-Point Arithmetic**: Floating-point prohibited for trust calculations
- **Stable Sorting**: Resolver candidate ranking is deterministic
- **Standard Error Codes**: ERR_INVALID_EPOCH, ERR_REVOKED_IDENTITY, etc.

## 17.3 Golden Test Procedure

1. Execute identical test vectors
2. Serialize outputs
3. Byte-compare results
4. Fail on any divergence

## 17.4 Certification Criteria

An implementation is considered interoperable if:

- All compliance vectors pass
- All golden tests match reference outputs

---

# 18. Governance and Standardization

The Governance & Internet-Draft RFC defines OpenNet's governance, evolution, and standardization process.

## 18.1 Governance Principles

1. **No central authority**: No entity controls the protocol
2. **Open participation**: Anyone can contribute
3. **Deterministic evolution**: Changes are predictable
4. **Backward compatibility by default**: Existing behavior is preserved
5. **Implementation-driven validation**: Working code determines

## 18.2 Internet-Draft Structure

OpenNet will be submitted as a coordinated draft set:

| Draft Name | Scope |
|------------|-------|
| draft-opennet-core | Architecture, node definition, message taxonomy |
| draft-opennet-wire | CBOR/TLV schemas, canonical encoding |
| draft-opennet-identity | Identity lifecycle, epoch semantics |
| draft-opennet-revocation | Revocation & recovery |
| draft-opennet-trust | Trust graph, weight dynamics |
| draft-opennet-resolver | open:// URI semantics, resolver behavior |
| draft-opennet-transport | QUIC/TCP binding, admission rules |
| draft-opennet-integration | Resolver-Trust-Transport binding, full-node behavior |
| draft-opennet-compliance | Compliance criteria, test vectors |
| draft-opennet-interoperability | Cross-language determinism |

## 18.3 Standard-Ready Criteria

OpenNet is considered standard-ready when:

- All core drafts reach stable revision
- Two independent interoperable implementations exist
- Compliance test suite passes

## 18.4 No-Authority Upgrade Model

1. Proposal drafted as Internet-Draft
2. Reference implementation updated
3. Compliance tests updated
4. Ecosystem adoption

No approval body exists. A feature is considered "standard" when multiple independent implementations support it and compliance tests pass.

---

# 19. Security Analysis

## 19.1 Threat Model

OpenNet is designed against the following attack vectors:

- **Sybil Attack**: Creating many fake identities
- **Eclipse Attack**: Surrounding a node with malicious peers
- **Reputation Laundering**: Escaping bad reputation by changing identity
- **Replay Attack**: Replaying old messages
- **Clock Manipulation**: Exploiting time drift

## 19.2 Protection Mechanisms

| Threat | Protection |
|--------|------------|
| Sybil | Exponential trust cost, slow accumulation, reset on identity split |
| Eclipse | Peer diversity, trust clustering, diversity factor |
| Rep. Laundering | No trust transfer on identity split, monotonic epochs |
| Replay | Nonce + timestamp + replay window |
| Clock Skew | NMT median, drift trust decay, epoch rejection |

## 19.3 Cryptographic Agility

OpenNet supports cryptographic algorithm transitions:

- **Baseline**: Ed25519 (mandatory)
- **Future**: Post-quantum schemes (CRYSTALS-Dilithium, SPHINCS+)
- **Transition**: Hybrid classical + post-quantum support

## 19.4 Security Properties

The integration ensures:

- **Sybil resistance** via trust thresholds
- **Eclipse resistance** via diversity
- **Revocation enforcement** end-to-end
- **Replay protection** via epoch-bound nonces

---

# 20. Conclusion and Future Vision

## 20.1 Summary

OpenNet is a comprehensive protocol suite that addresses the fundamental architectural problems of the current internet. With service-centric addressing, decentralized trust management, and deterministic behavior guarantees, it offers a censorship-resistant, unstoppable internet infrastructure.

This specification set, consisting of 24 RFC documents, defines every aspect of the protocol in detail: from identity lifecycle to trust mathematics, from wire format to test vectors.

## 20.2 Key Achievements

- **Censorship Resistance**: Technically impossible to shut down a single domain
- **Decentralized Trust**: Cryptographic trust without CA or authority
- **Deterministic Behavior**: Different implementations produce same results
- **Evolvability**: Protocol can grow organically

## 20.3 Future Roadmap

1. **Short-Term**: Rust reference implementation v1.0.0, IETF Internet-Draft submission
2. **Medium-Term**: Go and C++ implementations, browser integration
3. **Long-Term**: Post-quantum cryptography transition, mobile SDKs

## 20.4 Closing Statement

OpenNet is not an "overlay" or extension of the current internet. It is an architectural change based on services instead of servers, identities instead of IP addresses, and collective participation instead of centralized ownership.

This document defines the foundation of a truly free, decentralized internet.

---

# Appendix A: RFC Document List

The OpenNet protocol suite consists of the following RFC documents:

## Core Architecture

- OpenNet Whitepaper — Service-Centric Internet Architecture
- OpenNet Companion RFCs — Resolver, Wire, Security, Governance

## Identity and Trust

- Identity Lifecycle RFC — Node Identity and Key Management
- Trust Weight Dynamics RFC — Trust Scoring and Decay
- Revocation & Recovery RFC — Revocation and Recovery

## Time and Epoch

- Time & Epoch Semantics RFC — Deterministic Time and Replay Protection

## Resolution and Integration

- Resolver Daemon RFC — OS/Browser Level Name Resolution
- Resolver-Trust-Transport Integration RFC — End-to-End Pipeline

## Wire Format

- CBOR & TLV Schemas RFC — Canonical Binary Encoding
- Low-Level Specifications RFC — Binary Schemas and Trust Mathematics

## State Machine

- Full-Node State Machine RFC — Authoritative FSM Definition
- Full-Node Behavior & Integration RFC — Deterministic Behavior
- Identity/Epoch/Revocation → FSM Binding RFC — Event Binding
- Resolver → FSM → Transport Integration RFC — Request Pipeline

## Implementation

- Rust Reference Node RFC — Executable Specification
- Rust Reference Implementation Freeze Plan
- Implementation & Deployment RFC

## Testing and Compliance

- Compliance Test Vectors RFC — Protocol Correctness Tests
- FSM Exhaustive Test Suite RFC — Comprehensive FSM Tests
- Cross-Implementation Interoperability RFC — Multi-Language Conformance

## Governance

- Governance & Internet-Draft RFC — Standards Process
- Internet-Draft Finalization RFC — IETF Submission Plan

---

# Appendix B: Glossary

| Term | Definition |
|------|------------|
| **CBOR** | Concise Binary Object Representation — Binary data format |
| **Epoch** | A bounded validity period of a key pair |
| **FSM** | Finite State Machine |
| **NMT** | Network Median Time |
| **NodeId** | Node's unique, immutable identifier |
| **Quorum** | Minimum number of trusted peers required to validate a decision |
| **Resolver** | Component that resolves open:// URIs to node endpoints |
| **ServiceID** | Cryptographic identifier of a service |
| **Sybil Attack** | Attack manipulating network by creating many fake identities |
| **TLV** | Type-Length-Value — Binary framing format |
| **Trust Weight** | Score in [0,1] range representing trust in a node |
| **Wire Format** | Binary encoding used for network transmission |
| **Scope** | Logical network partition (geographic, privacy, test) |
| **Decay** | Exponential reduction of trust over time |
| **Canonical** | Deterministic, standardized encoding |

---

*— End of Document —*
