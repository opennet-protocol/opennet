# opennet-trust

Trust graph and weight dynamics for OpenNet.

## Purpose

Manages the trust graph and weight calculations:

- `TrustEvaluator` - FSM event producer
- Exponential decay function
- Threshold enforcement

## RFC Reference

- Trust Weight Dynamics RFC

## Module Structure

```
src/
├── lib.rs
├── graph/
│   ├── mod.rs
│   ├── node.rs         # Trust graph node
│   ├── edge.rs         # Trust edge (weighted)
│   ├── storage.rs      # Graph storage
│   └── snapshot.rs     # Graph snapshots
├── weight/
│   ├── mod.rs
│   ├── calculation.rs  # Trust weight calculation
│   ├── propagation.rs  # Trust propagation model
│   └── fixed_point.rs  # Fixed-point arithmetic
├── decay/
│   ├── mod.rs
│   ├── exponential.rs  # Exponential decay function
│   └── parameters.rs   # Decay parameters
├── thresholds/
│   ├── mod.rs
│   ├── resolve.rs      # ResolveThreshold
│   ├── connect.rs      # ConnectThreshold
│   └── relay.rs        # RelayThreshold
├── events/
│   ├── mod.rs
│   ├── positive.rs     # Positive trust events
│   └── negative.rs     # Negative trust events
├── sybil/
│   ├── mod.rs
│   └── resistance.rs   # Sybil resistance analysis
├── evaluator.rs        # TrustEvaluator
└── error.rs
```

## Trust Calculation

```text
TW(j) += TW(i) × w(i,j) × decay(age)
```

## Thresholds

| Threshold | Default |
|-----------|---------|
| Resolve | 0.2 |
| Connect | 0.3 |
| Relay | 0.4 |

## Design Rules

- **NO floating-point in production code**
- Use `fixed` crate for trust math
- All calculations must be deterministic

## License

MIT OR Apache-2.0
