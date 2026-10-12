//! # Buffer.rs
//! 
//! A gpu buffer
//! 

use crate::{image::save::SaveImage, memory_layout::{IntoMemLayout, MemLayout}, pixel::Pixel, point::Point};

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
pub type AoSBuffer = Vec<Point<Pixel>>;

impl IntoMemLayout for AoSBuffer {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::AoS
    }
}

impl Buffer for AoSBuffer {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
}

impl SaveImage for AoSBuffer {
    fn save_png(&self) {
        todo!()
    }
}

/// SoA meaning contiguous memory alignment,
/// ie. `[[x, x, ...], [y, y, ...]]`
pub type SoABuffer = Vec<Vec<Pixel>>;

impl IntoMemLayout for SoABuffer {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::SoA
    }
}

impl Buffer for SoABuffer {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
}

impl SaveImage for SoABuffer {
    fn save_png(&self) {
        todo!()
    }
}
