//! TLV reader.

use super::frame::TlvFrame;
use crate::error::Result;

/// TLV frame reader.
pub struct TlvReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> TlvReader<'a> {
    /// Create new reader.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Check if at end.
    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Read next frame.
    pub fn read_frame(&mut self) -> Result<Option<TlvFrame>> {
        if self.is_empty() {
            return Ok(None);
        }

        let (frame, consumed) = TlvFrame::from_bytes(&self.data[self.pos..])?;
        self.pos += consumed;
        Ok(Some(frame))
    }

    /// Read all remaining frames.
    pub fn read_all(&mut self) -> Result<Vec<TlvFrame>> {
        let mut frames = Vec::new();
        while let Some(frame) = self.read_frame()? {
            frames.push(frame);
        }
        Ok(frames)
    }
}
