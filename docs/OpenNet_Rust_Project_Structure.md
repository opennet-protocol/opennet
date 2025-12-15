# OpenNet Rust Implementation — Project Structure

## Complete File and Directory Layout

Based on the OpenNet RFC specifications, this document defines the complete Rust project structure for implementing the OpenNet protocol suite.

---

## 1. Repository Root Structure

```
opennet/
├── Cargo.toml                    # Workspace root
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── rustfmt.toml                  # Code formatting rules
├── clippy.toml                   # Linting rules
├── deny.toml                     # Dependency audit
│
├── crates/                       # All library crates
│   ├── opennet-core/
│   ├── opennet-wire/
│   ├── opennet-identity/
│   ├── opennet-revocation/
│   ├── opennet-time/
│   ├── opennet-trust/
│   ├── opennet-resolver/
│   ├── opennet-transport/
│   ├── opennet-node/
│   └── opennet-tests/
│
├── bins/                         # Binary applications
│   ├── opennet-daemon/           # Full node daemon
│   └── opennet-cli/              # CLI tools
│
├── specs/                        # RFC-aligned specifications
│   ├── rfc-core.md
│   ├── rfc-wire.md
│   ├── rfc-identity.md
│   ├── rfc-trust.md
│   └── ...
│
├── test-vectors/                 # Canonical test vectors
│   ├── cbor/
│   ├── trust/
│   ├── resolver/
│   └── fsm/
│
├── docs/                         # Documentation
│   ├── architecture.md
│   ├── getting-started.md
│   └── api/
│
└── scripts/                      # Build and utility scripts
    ├── generate-test-vectors.sh
    └── verify-determinism.sh
```

---

## 2. Workspace Cargo.toml

```toml
# opennet/Cargo.toml

[workspace]
resolver = "2"
members = [
    "crates/opennet-core",
    "crates/opennet-wire",
    "crates/opennet-identity",
    "crates/opennet-revocation",
    "crates/opennet-time",
    "crates/opennet-trust",
    "crates/opennet-resolver",
    "crates/opennet-transport",
    "crates/opennet-node",
    "crates/opennet-tests",
    "bins/opennet-daemon",
    "bins/opennet-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"
repository = "https://github.com/opennet-protocol/opennet"
authors = ["OpenNet Contributors"]

[workspace.dependencies]
# Internal crates
opennet-core = { path = "crates/opennet-core" }
opennet-wire = { path = "crates/opennet-wire" }
opennet-identity = { path = "crates/opennet-identity" }
opennet-revocation = { path = "crates/opennet-revocation" }
opennet-time = { path = "crates/opennet-time" }
opennet-trust = { path = "crates/opennet-trust" }
opennet-resolver = { path = "crates/opennet-resolver" }
opennet-transport = { path = "crates/opennet-transport" }
opennet-node = { path = "crates/opennet-node" }

# Cryptography
ed25519-dalek = { version = "2.1", features = ["rand_core"] }
sha2 = "0.10"
rand = "0.8"
rand_chacha = "0.3"

# Serialization (CBOR)
ciborium = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_bytes = "0.11"

# Networking
tokio = { version = "1.35", features = ["full"] }
quinn = "0.11"                    # QUIC implementation
rustls = "0.23"

# Fixed-point arithmetic (NO FLOATING POINT)
fixed = "1.24"
num-traits = "0.2"

# Collections (deterministic)
indexmap = "2.1"                  # Ordered map
btreemap = "0.1"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Testing
proptest = "1.4"
criterion = "0.5"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
panic = "abort"
```

---

## 3. Crate: opennet-core

**Purpose**: Core types, NodeId, ServiceId, Epoch — Shared primitives

```
crates/opennet-core/
├── Cargo.toml
└── src/
    ├── lib.rs                    # Public API exports
    ├── node_id.rs                # NodeId type and derivation
    ├── service_id.rs             # ServiceId type
    ├── epoch.rs                  # Epoch struct and validation
    ├── scope.rs                  # Scope enum and parsing
    ├── uri.rs                    # open:// and service:// URI parsing
    ├── error.rs                  # Core error types
    ├── constants.rs              # Protocol constants
    └── types/
        ├── mod.rs
        ├── hash.rs               # Hash wrapper types
        ├── signature.rs          # Signature wrapper
        ├── public_key.rs         # PublicKey wrapper
        └── timestamp.rs          # Timestamp (NOT system time)
```

### Core Cargo.toml

```toml
# crates/opennet-core/Cargo.toml

[package]
name = "opennet-core"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Core types for OpenNet protocol"

[dependencies]
ed25519-dalek.workspace = true
sha2.workspace = true
serde.workspace = true
thiserror.workspace = true
fixed.workspace = true

[dev-dependencies]
proptest.workspace = true
```

### Core lib.rs Structure

```rust
// crates/opennet-core/src/lib.rs

//! # OpenNet Core
//! 
//! Core types and primitives for the OpenNet protocol.
//! 
//! RFC: OpenNet Core Protocol

#![forbid(unsafe_code)]
#![deny(floating_point_arithmetic)]
#![warn(missing_docs)]

pub mod error;
pub mod types;

mod node_id;
mod service_id;
mod epoch;
mod scope;
mod uri;
mod constants;

pub use node_id::NodeId;
pub use service_id::ServiceId;
pub use epoch::{Epoch, EpochId};
pub use scope::Scope;
pub use uri::{OpenNetUri, ServiceUri};
pub use constants::*;
pub use error::{CoreError, Result};

/// Protocol version
pub const PROTOCOL_VERSION: u16 = 1;

/// Wire format version
pub const WIRE_VERSION: u16 = 1;
```

---

## 4. Crate: opennet-wire

**Purpose**: CBOR/TLV canonical encoding — Wire format

```
crates/opennet-wire/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── cbor/
    │   ├── mod.rs
    │   ├── canonical.rs          # Canonical CBOR encoding
    │   ├── decoder.rs            # CBOR decoding with validation
    │   ├── encoder.rs            # CBOR encoding
    │   └── keys.rs               # CBOR key registry
    ├── tlv/
    │   ├── mod.rs
    │   ├── frame.rs              # TLV frame structure
    │   ├── types.rs              # TLV type registry
    │   ├── reader.rs             # TLV reader
    │   └── writer.rs             # TLV writer
    ├── frame/
    │   ├── mod.rs
    │   ├── header.rs             # Frame header
    │   ├── payload.rs            # Payload handling
    │   └── signature.rs          # Signature placement
    ├── messages/
    │   ├── mod.rs
    │   ├── node_hello.rs
    │   ├── node_welcome.rs
    │   ├── service_join.rs
    │   ├── service_leave.rs
    │   ├── stream_open.rs
    │   ├── stream_data.rs
    │   ├── stream_close.rs
    │   └── revocation.rs
    ├── validation.rs             # Wire format validation
    └── error.rs
```

### Wire Cargo.toml

```toml
# crates/opennet-wire/Cargo.toml

[package]
name = "opennet-wire"
version.workspace = true
edition.workspace = true
description = "Wire format encoding for OpenNet (CBOR/TLV)"

[dependencies]
opennet-core.workspace = true
ciborium.workspace = true
serde.workspace = true
serde_bytes.workspace = true
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
hex = "0.4"
```

---

## 5. Crate: opennet-identity

**Purpose**: Key lifecycle, rotation, merge/split — Identity management

```
crates/opennet-identity/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── keypair.rs                # Ed25519 keypair management
    ├── node_identity.rs          # Complete node identity
    ├── rotation/
    │   ├── mod.rs
    │   ├── request.rs            # Key rotation request
    │   ├── validation.rs         # Rotation validation
    │   └── chain.rs              # Epoch chain verification
    ├── announcement.rs           # Identity announcement
    ├── compromise.rs             # Compromise detection
    ├── storage.rs                # Secure key storage
    ├── watcher.rs                # IdentityWatcher (FSM event producer)
    └── error.rs
```

### Identity Cargo.toml

```toml
# crates/opennet-identity/Cargo.toml

[package]
name = "opennet-identity"
version.workspace = true
edition.workspace = true
description = "Identity lifecycle management for OpenNet"

# RFC: Identity Lifecycle RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
ed25519-dalek.workspace = true
sha2.workspace = true
rand.workspace = true
rand_chacha.workspace = true
serde.workspace = true
thiserror.workspace = true
zeroize = "1.7"                   # Secure memory clearing

[dev-dependencies]
proptest.workspace = true
tempfile = "3.9"
```

---

## 6. Crate: opennet-revocation

**Purpose**: Revocation & recovery logic

```
crates/opennet-revocation/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── revocation/
    │   ├── mod.rs
    │   ├── object.rs             # Revocation CBOR object
    │   ├── trigger.rs            # Revocation triggers
    │   ├── propagation.rs        # Gossip propagation
    │   └── validation.rs         # Revocation validation
    ├── quorum/
    │   ├── mod.rs
    │   ├── validator.rs          # Quorum validation
    │   └── weight.rs             # Weight-based quorum
    ├── recovery/
    │   ├── mod.rs
    │   ├── request.rs            # Recovery request
    │   ├── attestation.rs        # Peer attestation
    │   └── process.rs            # Recovery process
    ├── listener.rs               # RevocationListener (FSM event producer)
    └── error.rs
```

### Revocation Cargo.toml

```toml
# crates/opennet-revocation/Cargo.toml

[package]
name = "opennet-revocation"
version.workspace = true
edition.workspace = true
description = "Revocation and recovery for OpenNet"

# RFC: Revocation & Recovery RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
opennet-identity.workspace = true
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
```

---

## 7. Crate: opennet-time

**Purpose**: Epochs, monotonic network time (NMT)

```
crates/opennet-time/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── nmt/
    │   ├── mod.rs
    │   ├── calculator.rs         # NMT calculation
    │   ├── sampler.rs            # Peer time sampling
    │   └── outlier.rs            # Outlier detection
    ├── epoch/
    │   ├── mod.rs
    │   ├── validity.rs           # Epoch validity rules
    │   ├── expiry.rs             # Epoch expiration
    │   └── transition.rs         # Epoch transitions
    ├── drift/
    │   ├── mod.rs
    │   ├── detector.rs           # Drift detection
    │   └── tolerance.rs          # Drift tolerance config
    ├── replay/
    │   ├── mod.rs
    │   ├── window.rs             # Replay window
    │   └── nonce.rs              # Nonce tracking
    ├── monitor.rs                # EpochMonitor (FSM event producer)
    ├── clock.rs                  # Monotonic clock abstraction
    └── error.rs
```

### Time Cargo.toml

```toml
# crates/opennet-time/Cargo.toml

[package]
name = "opennet-time"
version.workspace = true
edition.workspace = true
description = "Time and epoch semantics for OpenNet"

# RFC: Time & Epoch Semantics RFC

[dependencies]
opennet-core.workspace = true
fixed.workspace = true
serde.workspace = true
thiserror.workspace = true
indexmap.workspace = true

[dev-dependencies]
proptest.workspace = true
```

---

## 8. Crate: opennet-trust

**Purpose**: Trust graph & weight dynamics

```
crates/opennet-trust/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── graph/
    │   ├── mod.rs
    │   ├── node.rs               # Trust graph node
    │   ├── edge.rs               # Trust edge (weighted)
    │   ├── storage.rs            # Graph storage
    │   └── snapshot.rs           # Graph snapshots
    ├── weight/
    │   ├── mod.rs
    │   ├── calculation.rs        # Trust weight calculation
    │   ├── propagation.rs        # Trust propagation model
    │   └── fixed_point.rs        # Fixed-point arithmetic
    ├── decay/
    │   ├── mod.rs
    │   ├── exponential.rs        # Exponential decay function
    │   └── parameters.rs         # Decay parameters
    ├── thresholds/
    │   ├── mod.rs
    │   ├── resolve.rs            # ResolveThreshold
    │   ├── connect.rs            # ConnectThreshold
    │   └── relay.rs              # RelayThreshold
    ├── events/
    │   ├── mod.rs
    │   ├── positive.rs           # Positive trust events
    │   └── negative.rs           # Negative trust events (penalties)
    ├── sybil/
    │   ├── mod.rs
    │   └── resistance.rs         # Sybil resistance analysis
    ├── evaluator.rs              # TrustEvaluator (FSM event producer)
    └── error.rs
```

### Trust Cargo.toml

```toml
# crates/opennet-trust/Cargo.toml

[package]
name = "opennet-trust"
version.workspace = true
edition.workspace = true
description = "Trust graph and weight dynamics for OpenNet"

# RFC: Trust Weight Dynamics RFC

[dependencies]
opennet-core.workspace = true
fixed.workspace = true
num-traits.workspace = true
indexmap.workspace = true
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
criterion.workspace = true

[[bench]]
name = "trust_calculation"
harness = false
```

---

## 9. Crate: opennet-resolver

**Purpose**: open:// URI resolution

```
crates/opennet-resolver/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── daemon/
    │   ├── mod.rs
    │   ├── service.rs            # Resolver daemon service
    │   ├── ipc/
    │   │   ├── mod.rs
    │   │   ├── unix.rs           # Unix socket IPC
    │   │   ├── windows.rs        # Named pipe IPC
    │   │   └── protocol.rs       # IPC protocol
    │   └── cache.rs              # Resolution cache
    ├── resolution/
    │   ├── mod.rs
    │   ├── parser.rs             # URI parsing
    │   ├── query.rs              # Network query
    │   └── result.rs             # Resolution result
    ├── ranking/
    │   ├── mod.rs
    │   ├── score.rs              # RankScore calculation
    │   ├── affinity.rs           # ScopeAffinity
    │   ├── diversity.rs          # DiversityFactor
    │   └── tiebreaker.rs         # Deterministic tie-breaking
    ├── scope/
    │   ├── mod.rs
    │   ├── filter.rs             # Scope filtering
    │   └── fallback.rs           # Fallback behavior
    ├── candidates/
    │   ├── mod.rs
    │   ├── set.rs                # Candidate set
    │   └── node_descriptor.rs    # Node descriptor
    └── error.rs
```

### Resolver Cargo.toml

```toml
# crates/opennet-resolver/Cargo.toml

[package]
name = "opennet-resolver"
version.workspace = true
edition.workspace = true
description = "Resolver daemon for OpenNet"

# RFC: Resolver Daemon RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
opennet-trust.workspace = true
tokio.workspace = true
serde.workspace = true
thiserror.workspace = true
indexmap.workspace = true

[target.'cfg(unix)'.dependencies]
tokio = { workspace = true, features = ["net"] }

[target.'cfg(windows)'.dependencies]
tokio = { workspace = true, features = ["net"] }

[dev-dependencies]
proptest.workspace = true
tokio-test = "0.4"
```

---

## 10. Crate: opennet-transport

**Purpose**: QUIC/TCP bindings

```
crates/opennet-transport/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── quic/
    │   ├── mod.rs
    │   ├── client.rs             # QUIC client
    │   ├── server.rs             # QUIC server
    │   ├── connection.rs         # QUIC connection
    │   ├── stream.rs             # QUIC stream
    │   └── config.rs             # QUIC configuration
    ├── tcp/
    │   ├── mod.rs
    │   ├── client.rs             # TCP fallback client
    │   ├── server.rs             # TCP fallback server
    │   └── connection.rs         # TCP connection
    ├── session/
    │   ├── mod.rs
    │   ├── binding.rs            # (NodeId, Epoch) binding
    │   ├── lifecycle.rs          # Session lifecycle
    │   └── manager.rs            # Session manager
    ├── admission/
    │   ├── mod.rs
    │   ├── control.rs            # Admission control
    │   └── threshold.rs          # Threshold enforcement
    ├── handshake/
    │   ├── mod.rs
    │   ├── initiator.rs          # Handshake initiator
    │   ├── responder.rs          # Handshake responder
    │   └── verification.rs       # Handshake verification
    ├── backpressure.rs           # Flow control
    └── error.rs
```

### Transport Cargo.toml

```toml
# crates/opennet-transport/Cargo.toml

[package]
name = "opennet-transport"
version.workspace = true
edition.workspace = true
description = "Transport layer for OpenNet (QUIC/TCP)"

# RFC: Transport RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
opennet-identity.workspace = true
opennet-trust.workspace = true
quinn.workspace = true
rustls.workspace = true
tokio.workspace = true
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
tokio-test = "0.4"
```

---

## 11. Crate: opennet-node

**Purpose**: Full-node state machine

```
crates/opennet-node/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── fsm/
    │   ├── mod.rs
    │   ├── state.rs              # NodeState enum
    │   ├── event.rs              # StateEvent enum
    │   ├── transition.rs         # Transition function
    │   ├── table.rs              # Transition table
    │   └── invariants.rs         # FSM invariants
    ├── states/
    │   ├── mod.rs
    │   ├── bootstrap.rs          # BOOTSTRAP state behavior
    │   ├── syncing.rs            # SYNCING state behavior
    │   ├── active.rs             # ACTIVE state behavior
    │   ├── degraded.rs           # DEGRADED state behavior
    │   └── quarantined.rs        # QUARANTINED state behavior
    ├── events/
    │   ├── mod.rs
    │   ├── queue.rs              # Deterministic event queue
    │   ├── priority.rs           # Event priority
    │   └── producer.rs           # Event producer registry
    ├── integration/
    │   ├── mod.rs
    │   ├── resolver.rs           # Resolver integration
    │   ├── trust.rs              # Trust integration
    │   ├── transport.rs          # Transport integration
    │   └── pipeline.rs           # Request pipeline
    ├── peer/
    │   ├── mod.rs
    │   ├── discovery.rs          # Peer discovery
    │   ├── manager.rs            # Peer manager
    │   └── exchange.rs           # Trust exchange
    ├── sync/
    │   ├── mod.rs
    │   ├── trust_sync.rs         # Trust graph sync
    │   ├── revocation_sync.rs    # Revocation sync
    │   └── epoch_sync.rs         # Epoch chain sync
    ├── config.rs                 # Node configuration
    └── error.rs
```

### Node Cargo.toml

```toml
# crates/opennet-node/Cargo.toml

[package]
name = "opennet-node"
version.workspace = true
edition.workspace = true
description = "Full-node implementation for OpenNet"

# RFC: Full-Node State Machine RFC
# RFC: Full-Node Behavior & Integration RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
opennet-identity.workspace = true
opennet-revocation.workspace = true
opennet-time.workspace = true
opennet-trust.workspace = true
opennet-resolver.workspace = true
opennet-transport.workspace = true
tokio.workspace = true
tracing.workspace = true
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
tokio-test = "0.4"
tracing-subscriber.workspace = true
```

---

## 12. Crate: opennet-tests

**Purpose**: Compliance & integration tests

```
crates/opennet-tests/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── vectors/
    │   ├── mod.rs
    │   ├── loader.rs             # Test vector loader
    │   ├── format.rs             # Vector format
    │   └── result.rs             # Expected results
    ├── compliance/
    │   ├── mod.rs
    │   ├── wire.rs               # Wire format compliance
    │   ├── crypto.rs             # Cryptographic compliance
    │   ├── resolver.rs           # Resolver compliance
    │   ├── trust.rs              # Trust compliance
    │   └── fsm.rs                # FSM compliance
    ├── integration/
    │   ├── mod.rs
    │   ├── single_node.rs        # Single node tests
    │   ├── multi_node.rs         # Multi-node tests
    │   └── pipeline.rs           # Pipeline tests
    ├── determinism/
    │   ├── mod.rs
    │   ├── replay.rs             # Deterministic replay
    │   └── comparison.rs         # Output comparison
    ├── golden/
    │   ├── mod.rs
    │   ├── harness.rs            # Golden test harness
    │   └── fixtures.rs           # Test fixtures
    ├── stress/
    │   ├── mod.rs
    │   ├── burst.rs              # Event burst tests
    │   └── adversarial.rs        # Adversarial tests
    └── helpers/
        ├── mod.rs
        ├── mock_time.rs          # Mock time source
        ├── mock_network.rs       # Mock network
        └── test_node.rs          # Test node builder
```

### Tests Cargo.toml

```toml
# crates/opennet-tests/Cargo.toml

[package]
name = "opennet-tests"
version.workspace = true
edition.workspace = true
description = "Compliance and integration tests for OpenNet"

# RFC: Compliance Test Vectors RFC
# RFC: FSM Exhaustive Test Suite RFC

[dependencies]
opennet-core.workspace = true
opennet-wire.workspace = true
opennet-identity.workspace = true
opennet-revocation.workspace = true
opennet-time.workspace = true
opennet-trust.workspace = true
opennet-resolver.workspace = true
opennet-transport.workspace = true
opennet-node.workspace = true
ciborium.workspace = true
serde.workspace = true
serde_json = "1.0"
thiserror.workspace = true

[dev-dependencies]
proptest.workspace = true
tokio-test = "0.4"
criterion.workspace = true
```

---

## 13. Binary: opennet-daemon

**Purpose**: Full node daemon executable

```
bins/opennet-daemon/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── cli.rs                    # CLI argument parsing
    ├── config.rs                 # Configuration loading
    ├── logging.rs                # Logging setup
    ├── signals.rs                # Signal handling
    └── metrics.rs                # Metrics export
```

### Daemon Cargo.toml

```toml
# bins/opennet-daemon/Cargo.toml

[package]
name = "opennet-daemon"
version.workspace = true
edition.workspace = true
description = "OpenNet full node daemon"

[[bin]]
name = "opennet-daemon"
path = "src/main.rs"

[dependencies]
opennet-node.workspace = true
opennet-core.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
clap = { version = "4.4", features = ["derive"] }
toml = "0.8"
serde.workspace = true
anyhow.workspace = true
```

---

## 14. Binary: opennet-cli

**Purpose**: CLI tools for management and debugging

```
bins/opennet-cli/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── commands/
    │   ├── mod.rs
    │   ├── resolve.rs            # Resolve command
    │   ├── identity.rs           # Identity commands
    │   ├── trust.rs              # Trust inspection
    │   ├── peer.rs               # Peer management
    │   └── debug.rs              # Debug commands
    └── output.rs                 # Output formatting
```

### CLI Cargo.toml

```toml
# bins/opennet-cli/Cargo.toml

[package]
name = "opennet-cli"
version.workspace = true
edition.workspace = true
description = "OpenNet CLI tools"

[[bin]]
name = "opennet"
path = "src/main.rs"

[dependencies]
opennet-core.workspace = true
opennet-resolver.workspace = true
opennet-trust.workspace = true
tokio.workspace = true
clap = { version = "4.4", features = ["derive"] }
serde_json = "1.0"
anyhow.workspace = true
```

---

## 15. Test Vectors Directory

```
test-vectors/
├── README.md
├── cbor/
│   ├── valid/
│   │   ├── node_hello.cbor
│   │   ├── service_join.cbor
│   │   └── ...
│   └── invalid/
│       ├── non_canonical.cbor
│       ├── duplicate_keys.cbor
│       └── ...
├── trust/
│   ├── decay/
│   │   ├── exponential_001.json
│   │   └── ...
│   ├── propagation/
│   │   └── ...
│   └── thresholds/
│       └── ...
├── resolver/
│   ├── ranking/
│   │   └── ...
│   └── scope/
│       └── ...
├── fsm/
│   ├── valid_transitions/
│   │   └── ...
│   ├── invalid_transitions/
│   │   └── ...
│   └── event_priority/
│       └── ...
└── integration/
    └── ...
```

---

## 16. Crate Dependency Graph

```
                    ┌─────────────────┐
                    │  opennet-node   │
                    └────────┬────────┘
                             │
       ┌─────────────────────┼─────────────────────┐
       │                     │                     │
       ▼                     ▼                     ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────────┐
│opennet-resolver│   │opennet-transport│   │opennet-revocation│
└──────┬───────┘    └──────┬───────┘    └────────┬─────────┘
       │                   │                      │
       │     ┌─────────────┼──────────────────────┤
       │     │             │                      │
       ▼     ▼             ▼                      ▼
    ┌──────────────┐  ┌──────────────┐    ┌──────────────┐
    │opennet-trust │  │opennet-identity│   │ opennet-time │
    └──────┬───────┘  └──────┬───────┘    └──────┬───────┘
           │                 │                   │
           └─────────────────┼───────────────────┘
                             │
                             ▼
                    ┌──────────────┐
                    │opennet-wire  │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │opennet-core  │
                    └──────────────┘
```

---

## 17. Implementation Priority Order

Based on RFC dependencies, implement in this order:

1. **Phase 1: Core Foundation**
   - `opennet-core` — Base types
   - `opennet-wire` — CBOR/TLV encoding

2. **Phase 2: Identity & Time**
   - `opennet-identity` — Key management
   - `opennet-time` — Epoch handling

3. **Phase 3: Trust**
   - `opennet-trust` — Trust graph
   - `opennet-revocation` — Revocation

4. **Phase 4: Resolution & Transport**
   - `opennet-resolver` — URI resolution
   - `opennet-transport` — QUIC/TCP

5. **Phase 5: Node Integration**
   - `opennet-node` — FSM and integration
   - `opennet-tests` — Compliance tests

6. **Phase 6: Binaries**
   - `opennet-daemon` — Full node
   - `opennet-cli` — CLI tools

---

## 18. Key Implementation Rules

### PROHIBITED (Per RFC)

```rust
// ❌ NEVER use these
f32, f64                    // Floating-point arithmetic
std::collections::HashMap   // Non-deterministic ordering
std::collections::HashSet   // Non-deterministic ordering
std::time::SystemTime       // Direct system time
rand::thread_rng()          // Unseeded randomness
```

### REQUIRED (Per RFC)

```rust
// ✅ ALWAYS use these
fixed::types::I32F32        // Fixed-point for trust math
indexmap::IndexMap          // Deterministic ordered map
std::collections::BTreeMap  // Deterministic ordered map
rand_chacha::ChaCha20Rng    // Seeded deterministic RNG
```

### Code Documentation

```rust
/// RFC: Trust Weight Dynamics §7.2
/// 
/// Computes exponential decay for trust weights.
pub fn decay(epoch_age: EpochDelta, lambda: FixedPoint) -> FixedPoint {
    // Implementation
}
```

---

*— End of Project Structure Document —*
