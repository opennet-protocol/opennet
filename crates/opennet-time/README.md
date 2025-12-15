# opennet-time

Time and epoch semantics for OpenNet.

## Purpose

Provides consistent time semantics without centralized time authorities:

- `EpochMonitor` - FSM event producer
- `MonotonicClock` - Abstract clock
- Network Median Time (NMT)

## RFC Reference

- Time & Epoch Semantics RFC

## Module Structure

```
src/
├── lib.rs
├── nmt/
│   ├── mod.rs
│   ├── calculator.rs   # NMT calculation
│   ├── sampler.rs      # Peer time sampling
│   └── outlier.rs      # Outlier detection
├── epoch/
│   ├── mod.rs
│   ├── validity.rs     # Epoch validity rules
│   ├── expiry.rs       # Epoch expiration
│   └── transition.rs   # Epoch transitions
├── drift/
│   ├── mod.rs
│   ├── detector.rs     # Drift detection
│   └── tolerance.rs    # Drift tolerance config
├── replay/
│   ├── mod.rs
│   ├── window.rs       # Replay window
│   └── nonce.rs        # Nonce tracking
├── monitor.rs          # EpochMonitor
├── clock.rs            # Monotonic clock abstraction
└── error.rs
```

## Constants

- Default replay window: 120 seconds
- Default drift tolerance: ±5 minutes

## License

MIT OR Apache-2.0
