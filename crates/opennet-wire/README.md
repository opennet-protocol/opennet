# opennet-wire

Wire format encoding for OpenNet (CBOR/TLV).

## Purpose

This crate provides wire-level binary encoding:

- **CBOR**: For structured payloads (canonical encoding)
- **TLV**: For low-level framing and extensions

## RFC Reference

- CBOR & TLV Schemas RFC
- Low-Level Specifications RFC

## Module Structure

```
src/
├── lib.rs
├── cbor/
│   ├── mod.rs
│   ├── canonical.rs    # Canonical CBOR encoding
│   ├── decoder.rs      # CBOR decoding with validation
│   ├── encoder.rs      # CBOR encoding
│   └── keys.rs         # CBOR key registry
├── tlv/
│   ├── mod.rs
│   ├── frame.rs        # TLV frame structure
│   ├── types.rs        # TLV type registry
│   ├── reader.rs       # TLV reader
│   └── writer.rs       # TLV writer
├── frame/
│   ├── mod.rs
│   ├── header.rs       # Frame header
│   ├── payload.rs      # Payload handling
│   └── signature.rs    # Signature placement
├── messages/
│   ├── mod.rs
│   ├── node_hello.rs
│   ├── node_welcome.rs
│   ├── service_join.rs
│   └── ...
├── validation.rs       # Wire format validation
└── error.rs
```

## Canonical CBOR Rules

1. Use definite-length encoding
2. Use ONLY integer map keys
3. Sort map keys in ascending order
4. Reject duplicate keys
5. Use minimal integer encoding
6. **Disallow floating-point values**

## TLV Type Registry

| Type | Name |
|------|------|
| 0x0001 | FRAME_HEADER |
| 0x0002 | CBOR_PAYLOAD |
| 0x0003 | SIGNATURE |
| 0x0004 | NODE_ID |
| 0x00FF | EXTENSION |

## License

MIT OR Apache-2.0
