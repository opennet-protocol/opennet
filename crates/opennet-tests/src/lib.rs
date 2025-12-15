//! # OpenNet Tests
//!
//! Compliance and integration tests for the OpenNet protocol.
//!
//! ## Test Philosophy
//!
//! This test suite aims to answer:
//!
//! > **"Is it possible for an OpenNet node to behave incorrectly?"**
//!
//! The goal is to reduce that answer to **NO** within defined protocol bounds.
//!
//! ## Test Categories
//!
//! - **Wire Format Tests**: Frame parsing, CBOR canonicity
//! - **Cryptographic Tests**: Signature verification
//! - **Resolver Tests**: Deterministic ranking
//! - **Trust Tests**: Decay correctness, threshold enforcement
//! - **FSM Tests**: Valid/invalid transitions, event priority
//! - **Integration Tests**: End-to-end pipeline
//!
//! ## Test Vector Format
//!
//! Each test vector is a canonical CBOR object:
//!
//! ```text
//! {
//!   0: test_id,
//!   1: description,
//!   2: input_frames,
//!   3: expected_result,
//!   4: flags
//! }
//! ```
//!
//! ## Result Codes
//!
//! | Code | Meaning |
//! |------|---------|
//! | ACCEPT | Input valid, state advanced |
//! | DROP | Input ignored |
//! | REJECT | Input invalid |
//! | ABORT | Fatal protocol violation |
//!
//! ## RFC Reference
//!
//! This crate implements:
//! - Compliance Test Vectors RFC
//! - FSM Exhaustive Test Suite RFC
//! - Cross-Implementation Interoperability RFC

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod vectors;
pub mod compliance;
pub mod integration;
pub mod determinism;
pub mod golden;
pub mod stress;
pub mod helpers;
