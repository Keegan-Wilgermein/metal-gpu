//! # Format
//! 
//! Marker to define a primitive type
//! 

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
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

    /// Gets the pixel format of the device
    fn get_pixel_format() -> MTLPixelFormat;

    /// Gets the pixel format as a `usize`
    fn get_pixel_format_as_usize() -> usize {
        Self::get_pixel_format().0
    }
}

impl MetalFormat for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA8Uint
    }
}

impl MetalFormat for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA8Sint
    }
}

impl MetalFormat for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA16Uint
    }
}

impl MetalFormat for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA16Sint
    }
}

impl MetalFormat for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA32Uint
    }
}

impl MetalFormat for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA32Sint
    }
}

impl MetalFormat for f32 {
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA32Float
    }
}

#[cfg(target_pointer_width = "32")]
impl MetalFormat for usize {
    const MIN: Self = usize::MIN;
    const MAX: Self = usize::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA32Uint
    }
}

#[cfg(target_pointer_width = "32")]
impl MetalFormat for isize {
    const MIN: Self = isize::MIN;
    const MAX: Self = isize::MAX;

    fn get_pixel_format() -> MTLPixelFormat {
        MTLPixelFormat::RGBA32Sint
    }
}
