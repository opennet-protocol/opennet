# opennet-tests

Compliance and integration tests for OpenNet.

## Purpose

Validates protocol correctness and compliance:

- Wire format compliance
- Cryptographic compliance
- FSM exhaustive testing
- Determinism verification
- Golden test harness

## RFC Reference

- Compliance Test Vectors RFC
- FSM Exhaustive Test Suite RFC
- Cross-Implementation Interoperability RFC

## Module Structure

```
src/
├── lib.rs
├── vectors/
│   ├── mod.rs
│   ├── loader.rs       # Test vector loader
│   ├── format.rs       # Vector format
│   └── result.rs       # Expected results
├── compliance/
│   ├── mod.rs
│   ├── wire.rs         # Wire format compliance
│   ├── crypto.rs       # Cryptographic compliance
│   ├── resolver.rs     # Resolver compliance
│   ├── trust.rs        # Trust compliance
│   └── fsm.rs          # FSM compliance
├── integration/
│   ├── mod.rs
│   ├── single_node.rs  # Single node tests
│   ├── multi_node.rs   # Multi-node tests
│   └── pipeline.rs     # Pipeline tests
├── determinism/
│   ├── mod.rs
│   ├── replay.rs       # Deterministic replay
│   └── comparison.rs   # Output comparison
├── golden/
│   ├── mod.rs
│   ├── harness.rs      # Golden test harness
│   └── fixtures.rs     # Test fixtures
├── stress/
│   ├── mod.rs
│   ├── burst.rs        # Event burst tests
│   └── adversarial.rs  # Adversarial tests
└── helpers/
    ├── mod.rs
    ├── mock_time.rs    # Mock time source
    ├── mock_network.rs # Mock network
    └── test_node.rs    # Test node builder
```

## Compliance Criteria

An implementation is compliant if:
- All tests defined in RFCs pass
- No undefined behavior is observed

## License

MIT OR Apache-2.0
