# opennet-resolver

Resolver daemon for OpenNet.

## Purpose

Resolves `open://` and `service://` URIs to node endpoints:

- URI parsing and validation
- Deterministic node ranking
- Scope filtering
- OS integration (Linux/Windows/macOS)

## RFC Reference

- Resolver Daemon RFC
- Resolver-Trust-Transport Integration RFC

## Module Structure

```
src/
├── lib.rs
├── daemon/
│   ├── mod.rs
│   ├── service.rs      # Resolver daemon service
│   ├── ipc/
│   │   ├── mod.rs
│   │   ├── unix.rs     # Unix socket IPC
│   │   ├── windows.rs  # Named pipe IPC
│   │   └── protocol.rs # IPC protocol
│   └── cache.rs        # Resolution cache
├── resolution/
│   ├── mod.rs
│   ├── parser.rs       # URI parsing
│   ├── query.rs        # Network query
│   └── result.rs       # Resolution result
├── ranking/
│   ├── mod.rs
│   ├── score.rs        # RankScore calculation
│   ├── affinity.rs     # ScopeAffinity
│   ├── diversity.rs    # DiversityFactor
│   └── tiebreaker.rs   # Deterministic tie-breaking
├── scope/
│   ├── mod.rs
│   ├── filter.rs       # Scope filtering
│   └── fallback.rs     # Fallback behavior
├── candidates/
│   ├── mod.rs
│   ├── set.rs          # Candidate set
│   └── node_descriptor.rs
└── error.rs
```

## Ranking Formula

```
RankScore = TW × ScopeAffinity × DiversityFactor
```

## Determinism

- Resolver ordering MUST be stable
- Same inputs → same outputs
- No adaptive heuristics

## License

MIT OR Apache-2.0
