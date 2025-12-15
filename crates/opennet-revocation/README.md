# opennet-revocation

Revocation and recovery for OpenNet.

## Purpose

Handles identity revocation and optional recovery:

- `RevocationListener` - FSM event producer
- Quorum-based validation
- Recovery process

## RFC Reference

- Revocation & Recovery RFC

## Module Structure

```
src/
├── lib.rs
├── revocation/
│   ├── mod.rs
│   ├── object.rs       # Revocation CBOR object
│   ├── trigger.rs      # Revocation triggers
│   ├── propagation.rs  # Gossip propagation
│   └── validation.rs   # Revocation validation
├── quorum/
│   ├── mod.rs
│   ├── validator.rs    # Quorum validation
│   └── weight.rs       # Weight-based quorum
├── recovery/
│   ├── mod.rs
│   ├── request.rs      # Recovery request
│   ├── attestation.rs  # Peer attestation
│   └── process.rs      # Recovery process
├── listener.rs         # RevocationListener
└── error.rs
```

## Revocation Object (CBOR)

```
type = REVOCATION
node_id, revoked_epoch, reason_code
timestamp, signatures[]
```

## Recovery Rules

- Recovery does NOT restore full historical trust
- Trusted peers must co-sign recovery
- New epoch starts with reduced weight

## License

MIT OR Apache-2.0
