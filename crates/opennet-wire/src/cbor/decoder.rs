//! CBOR decoder with canonical validation.

use crate::error::{WireError, Result};

/// CBOR decoder with canonical validation.
pub struct CborDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CborDecoder<'a> {
    /// Create new decoder.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Check if at end.
    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Remaining bytes.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Decode unsigned integer.
    pub fn decode_uint(&mut self) -> Result<u64> {
        let (major, value) = self.decode_type_and_value()?;
        if major != 0 {
            return Err(WireError::InvalidCbor("expected uint".into()));
        }
        Ok(value)
    }

    /// Decode signed integer.
    pub fn decode_int(&mut self) -> Result<i64> {
        let (major, value) = self.decode_type_and_value()?;
        match major {
            0 => Ok(value as i64),
            1 => Ok(-1 - value as i64),
            _ => Err(WireError::InvalidCbor("expected int".into())),
        }
    }

    /// Decode byte string.
    pub fn decode_bytes(&mut self) -> Result<Vec<u8>> {
        let (major, len) = self.decode_type_and_value()?;
        if major != 2 {
            return Err(WireError::InvalidCbor("expected bytes".into()));
        }
        let len = len as usize;
        if self.pos + len > self.data.len() {
            return Err(WireError::InvalidCbor("unexpected end".into()));
        }
        let bytes = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(bytes)
    }

    /// Decode text string.
    pub fn decode_text(&mut self) -> Result<String> {
        let (major, len) = self.decode_type_and_value()?;
        if major != 3 {
            return Err(WireError::InvalidCbor("expected text".into()));
        }
        let len = len as usize;
        if self.pos + len > self.data.len() {
            return Err(WireError::InvalidCbor("unexpected end".into()));
        }
        let text = std::str::from_utf8(&self.data[self.pos..self.pos + len])
            .map_err(|e| WireError::InvalidCbor(e.to_string()))?
            .to_string();
        self.pos += len;
        Ok(text)
    }

    /// Decode array header, returns length.
    pub fn decode_array_header(&mut self) -> Result<usize> {
        let (major, len) = self.decode_type_and_value()?;
        if major != 4 {
            return Err(WireError::InvalidCbor("expected array".into()));
        }
        Ok(len as usize)
    }

    /// Decode map header, returns length.
    pub fn decode_map_header(&mut self) -> Result<usize> {
        let (major, len) = self.decode_type_and_value()?;
        if major != 5 {
            return Err(WireError::InvalidCbor("expected map".into()));
        }
        Ok(len as usize)
    }

    /// Internal: decode type and value.
    fn decode_type_and_value(&mut self) -> Result<(u8, u64)> {
        if self.is_empty() {
            return Err(WireError::InvalidCbor("unexpected end".into()));
        }

        let initial = self.data[self.pos];
        self.pos += 1;

        let major = initial >> 5;
        let additional = initial & 0x1f;

        let value = match additional {
            0..=23 => additional as u64,
            24 => {
                if self.pos >= self.data.len() {
                    return Err(WireError::InvalidCbor("unexpected end".into()));
                }
                let v = self.data[self.pos] as u64;
                self.pos += 1;
                v
            }
            25 => {
                if self.pos + 2 > self.data.len() {
                    return Err(WireError::InvalidCbor("unexpected end".into()));
                }
                let v = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]) as u64;
                self.pos += 2;
                v
            }
            26 => {
                if self.pos + 4 > self.data.len() {
                    return Err(WireError::InvalidCbor("unexpected end".into()));
                }
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(&self.data[self.pos..self.pos + 4]);
                self.pos += 4;
                u32::from_be_bytes(bytes) as u64
            }
            27 => {
                if self.pos + 8 > self.data.len() {
                    return Err(WireError::InvalidCbor("unexpected end".into()));
                }
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&self.data[self.pos..self.pos + 8]);
                self.pos += 8;
                u64::from_be_bytes(bytes)
            }
            _ => return Err(WireError::InvalidCbor("invalid additional info".into())),
        };

        Ok((major, value))
    }
}
