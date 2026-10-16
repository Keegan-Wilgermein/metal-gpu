//! # Format
//! 
//! Marker to define a primitive type
//! 

use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};
use objc2_metal::MTLPixelFormat;

/// A type that can be used as a `MTLPixelFormat`
pub trait MetalFormat
where
    Self: Sized
        + Add + AddAssign
        + Sub + SubAssign
        + Mul + MulAssign
        + Div + DivAssign,
{
    /// The minimum value of `Self`
    const MIN: Self;
    /// The maximum value of `Self`
    const MAX: Self;
    /// The pixel format of the type
    const PIXEL_FORMAT: MTLPixelFormat;
    /// The pixel format of the type as `usize`
    const PIXEL_FORMAT_USIZE: usize = Self::PIXEL_FORMAT.0;
}

impl MetalFormat for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;
}
