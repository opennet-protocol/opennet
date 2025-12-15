# opennet-identity

Identity lifecycle management for OpenNet.

## Purpose

Manages node identities, key pairs, and epoch transitions:

- `KeyPair` - Ed25519 keypair management
- `NodeIdentity` - Complete node identity
- `IdentityWatcher` - FSM event producer for identity events

## RFC Reference

- Identity Lifecycle RFC

## Module Structure

```
src/
├── lib.rs
├── keypair.rs          # Ed25519 keypair management
├── node_identity.rs    # Complete node identity
├── rotation/
│   ├── mod.rs
│   ├── request.rs      # Key rotation request
│   ├── validation.rs   # Rotation validation
│   └── chain.rs        # Epoch chain verification
├── announcement.rs     # Identity announcement
├── compromise.rs       # Compromise detection
├── storage.rs          # Secure key storage
├── watcher.rs          # IdentityWatcher (FSM event producer)
└── error.rs
```

## Identity Model

- **NodeId**: Immutable, SHA-256 hash of canonical_public_key
- **Key Pair**: Rotatable, Ed25519 mandatory

## Epoch Rules

- Epoch numbers MUST strictly increase monotonically
- Rollback is NOT accepted
- Invalid rotation causes identity rejection

## Security

- Uses `zeroize` for secure memory clearing
- Keys are never logged or serialized in plaintext

## License

MIT OR Apache-2.0
