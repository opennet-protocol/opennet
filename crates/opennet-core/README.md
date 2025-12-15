# opennet-core

Core types and primitives for the OpenNet protocol.

## Purpose

This crate provides the foundational types used throughout OpenNet:

- `NodeId` - Unique, immutable node identifier (SHA-256 hash of public key)
- `ServiceId` - Cryptographic service identifier
- `Epoch` - Bounded validity period for key pairs
- `Scope` - Logical network partitions
- `OpenNetUri` - Parser for `open://` URI scheme

## RFC Reference

- OpenNet Core Protocol RFC
- OpenNet Companion RFCs

## Module Structure

```
src/
├── lib.rs          # Public API exports
├── node_id.rs      # NodeId type and derivation
├── service_id.rs   # ServiceId type
├── epoch.rs        # Epoch struct and validation
├── scope.rs        # Scope enum and parsing
├── uri.rs          # open:// and service:// URI parsing
├── error.rs        # Core error types
├── constants.rs    # Protocol constants
└── types/
    ├── mod.rs
    ├── hash.rs       # Hash wrapper types
    ├── signature.rs  # Signature wrapper
    ├── public_key.rs # PublicKey wrapper
    └── timestamp.rs  # Timestamp (NOT system time)
```

## Design Rules

### PROHIBITED
```rust
// ❌ NEVER use these
f32, f64                    // Floating-point arithmetic
std::collections::HashMap   // Non-deterministic ordering
std::time::SystemTime       // Direct system time
```

### REQUIRED
```rust
// ✅ ALWAYS use these
fixed::types::I32F32        // Fixed-point arithmetic
std::collections::BTreeMap  // Deterministic ordered map
```

## Usage

```toml
[dependencies]
opennet-core = { path = "../opennet-core" }
```

## License

MIT OR Apache-2.0
