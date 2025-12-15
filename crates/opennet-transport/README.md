# opennet-transport

Transport layer for OpenNet (QUIC/TCP).

## Purpose

Establishes secure connections with Resolver-approved nodes:

- QUIC client/server
- TCP fallback
- Session management
- Admission control

## RFC Reference

- Transport RFC
- Resolver-Trust-Transport Integration RFC

## Module Structure

```
src/
├── lib.rs
├── quic/
│   ├── mod.rs
│   ├── client.rs       # QUIC client
│   ├── server.rs       # QUIC server
│   ├── connection.rs   # QUIC connection
│   ├── stream.rs       # QUIC stream
│   └── config.rs       # QUIC configuration
├── tcp/
│   ├── mod.rs
│   ├── client.rs       # TCP fallback client
│   ├── server.rs       # TCP fallback server
│   └── connection.rs   # TCP connection
├── session/
│   ├── mod.rs
│   ├── binding.rs      # (NodeId, Epoch) binding
│   ├── lifecycle.rs    # Session lifecycle
│   └── manager.rs      # Session manager
├── admission/
│   ├── mod.rs
│   ├── control.rs      # Admission control
│   └── threshold.rs    # Threshold enforcement
├── handshake/
│   ├── mod.rs
│   ├── initiator.rs    # Handshake initiator
│   ├── responder.rs    # Handshake responder
│   └── verification.rs # Handshake verification
├── backpressure.rs     # Flow control
└── error.rs
```

## Session Binding

Each session is bound to `(NodeId, Epoch)`:
- Auto-terminate on epoch change
- Prevent revoked key communication
- Replay protection

## License

MIT OR Apache-2.0
