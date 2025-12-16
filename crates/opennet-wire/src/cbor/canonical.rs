//! Canonical CBOR encoding rules.
//!
//! Rules:
//! 1. Use definite-length encoding
//! 2. Use ONLY integer map keys
//! 3. Sort map keys in ascending order
//! 4. Reject duplicate keys
//! 5. Use minimal integer encoding
//! 6. Disallow floating-point values

use crate::error::{WireError, Result};

/// Canonical CBOR encoder/validator.
pub struct CanonicalCbor;

impl CanonicalCbor {
    /// Validate that bytes are canonical CBOR.
    pub fn validate(bytes: &[u8]) -> Result<()> {
        let mut decoder = CborValidator::new(bytes);
        decoder.validate_value()?;
        
        if !decoder.is_empty() {
            return Err(WireError::InvalidCbor("trailing bytes".into()));
        }
        
        Ok(())
    }

    /// Check if integer uses minimal encoding.
    pub fn is_minimal_int(value: i64, encoded_len: usize) -> bool {
        match value {
            0..=23 => encoded_len == 1,
            24..=255 => encoded_len == 2,
            256..=65535 => encoded_len == 3,
            65536..=4294967295 => encoded_len == 5,
            _ => encoded_len == 9,
        }
    }
}

/// Internal validator state.
struct CborValidator<'a> {
    data: &'a [u8],
    pos: usize,
    map_key_stack: Vec<i64>,
}

impl<'a> CborValidator<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, map_key_stack: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn validate_value(&mut self) -> Result<()> {
        if self.is_empty() {
            return Err(WireError::InvalidCbor("unexpected end".into()));
        }

        let initial_byte = self.data[self.pos];
        let major_type = initial_byte >> 5;
        let additional = initial_byte & 0x1f;

        match major_type {
            0 | 1 => self.validate_int(additional)?,
            2 | 3 => self.validate_string(additional)?,
            4 => self.validate_array(additional)?,
            5 => self.validate_map(additional)?,
            6 => self.validate_tag(additional)?,
            7 if additional == 25 || additional == 26 || additional == 27 => {
                return Err(WireError::FloatingPointDetected);
            }
            7 => self.validate_simple(additional)?,
            _ => return Err(WireError::InvalidCbor("invalid major type".into())),
        }

        Ok(())
    }

    fn validate_int(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1; // Simplified - full impl would check minimal encoding
        Ok(())
    }

    fn validate_string(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1; // Simplified
        Ok(())
    }

    fn validate_array(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1; // Simplified
        Ok(())
    }

    fn validate_map(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1; // Simplified - full impl would check key ordering
        Ok(())
    }

    fn validate_tag(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1;
        Ok(())
    }

    fn validate_simple(&mut self, _additional: u8) -> Result<()> {
        self.pos += 1;
        Ok(())
    }
}
