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
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA8Uint;
}

impl MetalFormat for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA8Sint;
}

impl MetalFormat for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA16Uint;
}

impl MetalFormat for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA16Sint;
}

impl MetalFormat for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA32Uint;
}

impl MetalFormat for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA32Sint;
}

impl MetalFormat for f32 {
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::RGBA32Float;
}
