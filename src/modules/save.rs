//! # Save
//!
//! Trait for saving an image
//!

use std::path::PathBuf;
use png::{BitDepth, EncodingError};

pub trait SaveImage {
    /// Saves the latest frame to a file
    fn save_png(&self, path: impl Into<PathBuf>) -> Result<(), EncodingError>;
}

/// A channel type that can be
/// written as a PNG sample
pub trait PngChannel: Copy {
    /// The bit depth the channel is saved at
    const BIT_DEPTH: BitDepth;

    /// Appends the channel to `out`
    /// as big-endian sample bytes
    fn write_sample(self, out: &mut Vec<u8>);
}

impl PngChannel for u8 {
    const BIT_DEPTH: BitDepth = BitDepth::Eight;

    fn write_sample(self, out: &mut Vec<u8>) {
        out.push(self);
    }
}

impl PngChannel for i8 {
    const BIT_DEPTH: BitDepth = BitDepth::Eight;

    fn write_sample(self, out: &mut Vec<u8>) {
        // Shifts `MIN..=MAX` to `0..=255`
        out.push((self as u8) ^ 0x80);
    }
}

impl PngChannel for u16 {
    const BIT_DEPTH: BitDepth = BitDepth::Sixteen;

    fn write_sample(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_be_bytes());
    }
}

impl PngChannel for i16 {
    const BIT_DEPTH: BitDepth = BitDepth::Sixteen;

    fn write_sample(self, out: &mut Vec<u8>) {
        // Shifts `MIN..=MAX` to `0..=65535`
        out.extend_from_slice(&((self as u16) ^ 0x8000).to_be_bytes());
    }
}

impl PngChannel for u32 {
    const BIT_DEPTH: BitDepth = BitDepth::Sixteen;

    fn write_sample(self, out: &mut Vec<u8>) {
        // Keeps the top 16 bits
        out.extend_from_slice(&((self >> 16) as u16).to_be_bytes());
    }
}

impl PngChannel for i32 {
    const BIT_DEPTH: BitDepth = BitDepth::Sixteen;

    fn write_sample(self, out: &mut Vec<u8>) {
        // Shifts `MIN..=MAX` to `0..=u32::MAX`
        // then keeps the top 16 bits
        out.extend_from_slice(&((((self as u32) ^ 0x8000_0000) >> 16) as u16).to_be_bytes());
    }
}

impl PngChannel for f32 {
    const BIT_DEPTH: BitDepth = BitDepth::Sixteen;

    fn write_sample(self, out: &mut Vec<u8>) {
        // Float colour is `0.0..=1.0`, anything
        // outside is clamped and NaN becomes 0
        let value = (self.clamp(0.0, 1.0) * u16::MAX as f32).round() as u16;
        out.extend_from_slice(&value.to_be_bytes());
    }
}
