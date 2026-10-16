//! # Metal Grid
//!
//! Wrapper around `MTLSize`
//!

use objc2_metal::MTLSize;

/// A size in threads, used for
/// dispatch grids and threadgroups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetalGrid {
    width: usize,
    height: usize,
    depth: usize,
}

impl From<MetalGrid> for MTLSize {
    fn from(value: MetalGrid) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth: value.depth,
        }
    }
}

impl From<MTLSize> for MetalGrid {
    fn from(value: MTLSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth: value.depth,
        }
    }
}

impl From<(usize, usize)> for MetalGrid {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1, 1)
    }
}

impl From<(usize, usize, usize)> for MetalGrid {
    fn from(value: (usize, usize, usize)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl MetalGrid {
    /// Creates a new `MetalGrid`
    pub fn new(
        width: usize,
        height: usize,
        depth: usize,
    ) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}
