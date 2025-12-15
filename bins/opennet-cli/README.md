# opennet-cli

OpenNet CLI tools.

## Purpose

Command-line tools for OpenNet management and debugging:

- URI resolution
- Identity management
- Trust inspection
- Peer management
- Debug utilities

## Usage

```bash
# Resolve an open:// URI
opennet resolve open://chat.opennet

# Show identity info
opennet identity show

# Rotate keys
opennet identity rotate

# Inspect trust graph
opennet trust inspect <node_id>

# List peers
opennet peer list

# Debug mode
opennet debug dump-state
```

## Commands

| Command | Description |
|---------|-------------|
| `resolve` | Resolve open:// URIs |
| `identity` | Identity management |
| `trust` | Trust inspection |
| `peer` | Peer management |
| `debug` | Debug utilities |

## Module Structure

```
src/
├── main.rs
├── commands/
│   ├── mod.rs
│   ├── resolve.rs    # Resolve command
│   ├── identity.rs   # Identity commands
│   ├── trust.rs      # Trust inspection
│   ├── peer.rs       # Peer management
│   └── debug.rs      # Debug commands
└── output.rs         # Output formatting
```

## License

MIT OR Apache-2.0
