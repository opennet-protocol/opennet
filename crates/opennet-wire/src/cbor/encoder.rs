//! CBOR encoder with canonical output.

use std::collections::BTreeMap;
use crate::error::Result;

/// CBOR encoder that produces canonical output.
pub struct CborEncoder {
    buffer: Vec<u8>,
}

impl CborEncoder {
    /// Create new encoder.
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Get encoded bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    /// Encode unsigned integer.
    pub fn encode_uint(&mut self, value: u64) -> &mut Self {
        self.encode_type_and_value(0, value);
        self
    }

    /// Encode signed integer.
    pub fn encode_int(&mut self, value: i64) -> &mut Self {
        if value >= 0 {
            self.encode_type_and_value(0, value as u64);
        } else {
            self.encode_type_and_value(1, (-1 - value) as u64);
        }
        self
    }

    /// Encode byte string.
    pub fn encode_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.encode_type_and_value(2, bytes.len() as u64);
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Encode text string.
    pub fn encode_text(&mut self, text: &str) -> &mut Self {
        self.encode_type_and_value(3, text.len() as u64);
        self.buffer.extend_from_slice(text.as_bytes());
        self
    }

    /// Encode array header.
    pub fn encode_array_header(&mut self, len: usize) -> &mut Self {
        self.encode_type_and_value(4, len as u64);
        self
    }

    /// Encode map header (keys must be added in order).
    pub fn encode_map_header(&mut self, len: usize) -> &mut Self {
        self.encode_type_and_value(5, len as u64);
        self
    }

    /// Encode a map with integer keys (automatically sorted).
    pub fn encode_int_map<V, F>(&mut self, map: &BTreeMap<i64, V>, encode_value: F) -> &mut Self
    where
        F: Fn(&mut Self, &V),
    {
        self.encode_map_header(map.len());
        for (key, value) in map {
            self.encode_int(*key);
            encode_value(self, value);
        }
        self
    }

    /// Encode boolean.
    pub fn encode_bool(&mut self, value: bool) -> &mut Self {
        self.buffer.push(if value { 0xf5 } else { 0xf4 });
        self
    }

    /// Encode null.
    pub fn encode_null(&mut self) -> &mut Self {
        self.buffer.push(0xf6);
        self
    }

    /// Internal: encode type and value.
    fn encode_type_and_value(&mut self, major: u8, value: u64) {
        let mt = major << 5;
        match value {
            0..=23 => self.buffer.push(mt | value as u8),
            24..=255 => {
                self.buffer.push(mt | 24);
                self.buffer.push(value as u8);
            }
            256..=65535 => {
                self.buffer.push(mt | 25);
                self.buffer.extend_from_slice(&(value as u16).to_be_bytes());
            }
            65536..=4294967295 => {
                self.buffer.push(mt | 26);
                self.buffer.extend_from_slice(&(value as u32).to_be_bytes());
            }
            _ => {
                self.buffer.push(mt | 27);
                self.buffer.extend_from_slice(&value.to_be_bytes());
            }
        }
    }
}

impl Default for CborEncoder {
    fn default() -> Self {
        Self::new()
    }
}
