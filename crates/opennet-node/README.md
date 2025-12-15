# opennet-node

Full-node implementation for OpenNet.

## Purpose

Implements the Full-Node State Machine (FSM):

- State management (BOOTSTRAP → ACTIVE → etc.)
- Event processing
- Layer integration (Resolver, Trust, Transport)

## RFC Reference

- Full-Node State Machine RFC
- Full-Node Behavior & Integration RFC
- Identity/Epoch/Revocation → FSM Binding RFC

## Module Structure

```
src/
├── lib.rs
├── fsm/
│   ├── mod.rs
│   ├── state.rs        # NodeState enum
│   ├── event.rs        # StateEvent enum
│   ├── transition.rs   # Transition function
│   ├── table.rs        # Transition table
│   └── invariants.rs   # FSM invariants
├── states/
│   ├── mod.rs
│   ├── bootstrap.rs    # BOOTSTRAP behavior
│   ├── syncing.rs      # SYNCING behavior
│   ├── active.rs       # ACTIVE behavior
│   ├── degraded.rs     # DEGRADED behavior
│   └── quarantined.rs  # QUARANTINED behavior
├── events/
│   ├── mod.rs
│   ├── queue.rs        # Deterministic event queue
│   ├── priority.rs     # Event priority
│   └── producer.rs     # Event producer registry
├── integration/
│   ├── mod.rs
│   ├── resolver.rs     # Resolver integration
│   ├── trust.rs        # Trust integration
│   ├── transport.rs    # Transport integration
│   └── pipeline.rs     # Request pipeline
├── peer/
│   ├── mod.rs
│   ├── discovery.rs    # Peer discovery
│   ├── manager.rs      # Peer manager
│   └── exchange.rs     # Trust exchange
├── sync/
│   ├── mod.rs
│   ├── trust_sync.rs   # Trust graph sync
│   ├── revocation_sync.rs
│   └── epoch_sync.rs
├── config.rs           # Node configuration
└── error.rs
```

## FSM State Definition

```rust
pub enum NodeState {
    Bootstrap,
    Syncing,
    Active,
    Degraded,
    Quarantined,
}
```

## Event Priority (highest first)

1. SecurityViolation
2. TrustCritical
3. TrustBelowWarn
4. TrustRecovered

## License

MIT OR Apache-2.0
