# OpenNet RFC Specifications

This directory contains all RFC documents for the OpenNet protocol suite.

## Core Architecture
- `rfc-core-whitepaper.md` - OpenNet Service-Centric Decentralized Internet
- `rfc-companion.md` - Companion RFCs: Resolver, Wire, Security, Governance

## Wire Format
- `rfc-wire-cbor-tlv.md` - CBOR/TLV Schemas & Canonical Binary Encoding
- `rfc-wire-low-level.md` - Low-Level Specifications: Binary Schemas, Trust Math

## Identity & Security
- `rfc-identity-lifecycle.md` - Node Identity, Key Management, Trust Continuity
- `rfc-revocation-recovery.md` - Key Compromise Revocation and Identity Recovery

## Time & Epoch
- `rfc-time-epoch.md` - Deterministic Time, Epochs, and Replay Protection

## Trust
- `rfc-trust-dynamics.md` - Trust Scoring, Decay, and Resolver Thresholds

## Resolver
- `rfc-resolver-daemon.md` - OS/Browser Level Name Resolution
- `rfc-resolver-trust-transport.md` - End-to-End Node Selection and Connection
- `rfc-resolver-fsm-transport.md` - Resolver → FSM → Transport Integration

## Full-Node State Machine
- `rfc-fsm-state-machine.md` - Authoritative FSM Definition
- `rfc-fsm-behavior.md` - Deterministic Node State & Failure Handling
- `rfc-fsm-event-binding.md` - Identity/Epoch/Revocation → FSM Event Binding
- `rfc-fsm-test-suite.md` - FSM Exhaustive Test Suite

## Testing & Compliance
- `rfc-compliance-vectors.md` - Protocol Correctness & Conformance
- `rfc-interoperability.md` - Cross-Implementation Interoperability

## Rust Implementation
- `rfc-rust-reference-node.md` - Executable Specification
- `rfc-rust-freeze-plan.md` - Implementation Freeze Plan
- `rfc-implementation-deployment.md` - Reference Node, Resolver Daemon, Test Vectors

## Governance
- `rfc-governance.md` - Standards Process and Evolution Model
- `rfc-ietf-submission.md` - IETF Internet-Draft Submission Plan

---

## RFC to Crate Mapping

| RFC | Primary Crate |
|-----|---------------|
| Core Whitepaper | `opennet-core` |
| Wire CBOR/TLV | `opennet-wire` |
| Identity Lifecycle | `opennet-identity` |
| Revocation & Recovery | `opennet-revocation` |
| Time & Epoch | `opennet-time` |
| Trust Dynamics | `opennet-trust` |
| Resolver Daemon | `opennet-resolver` |
| FSM State Machine | `opennet-node` |
| Compliance Vectors | `opennet-tests` |
