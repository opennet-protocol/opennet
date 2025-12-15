# opennet-daemon

OpenNet full node daemon.

## Purpose

Runs a full OpenNet node as a background service:

- Full FSM implementation
- Resolver daemon
- Transport server
- Metrics export

## Usage

```bash
# Run with default config
opennet-daemon

# Run with custom config
opennet-daemon --config /path/to/config.toml

# Run with verbose logging
opennet-daemon -v
```

## Configuration

See `docs/configuration.md` for configuration options.

## Signals

- `SIGTERM` / `SIGINT`: Graceful shutdown
- `SIGHUP`: Reload configuration

## Module Structure

```
src/
├── main.rs     # Entry point
├── cli.rs      # CLI argument parsing
├── config.rs   # Configuration loading
├── logging.rs  # Logging setup
├── signals.rs  # Signal handling
└── metrics.rs  # Metrics export
```

## License

MIT OR Apache-2.0
