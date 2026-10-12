//! # Buffer.rs
//! 
//! A gpu buffer
//! 

use crate::{point::Point};

/// Defines methods that all buffers should implement
pub trait Buffer {
    /// Creates a new empty buffer
    fn new() -> Self;

    /// Creates a new buffer with a
    /// starting capacity
    fn with_capacity(capacity: usize) -> Self;
}

/// AoS meaning interleaved memory alignment,
/// ie. `[[x, y], [x, y], ...]`
pub type AoSBuffer = Vec<Point<f32>>;

impl Buffer for AoSBuffer {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
}

/// SoA meaning contiguous memory alignment,
/// ie. `[[x, x, ...], [y, y, ...]]`
pub type SoABuffer = Vec<Vec<f32>>;

impl Buffer for SoABuffer {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
}
