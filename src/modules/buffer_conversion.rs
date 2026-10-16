//! # Buffer Conversion
//! 
//! Traits for converting a `NonNull<c_void>`
//! into a usable buffer of `Pixel`s and back
//! 

use std::{mem, slice};

use objc2::{runtime::ProtocolObject};
use objc2_metal::MTLBuffer;

use crate::{api::pixel::Pixel, metal::{metal_format::MetalFormat}};

/// Turns `Self` into a slice of `Self::Output`
pub trait IntoBuffer<P> {
    type Output;

    /// Converts something into a pixel buffer
    fn into_pixel_buffer(&self) -> &mut [Self::Output];
}

/// Converts back from the output of
/// the `IntoBuffer` trait
pub trait FromBuffer {
    type Output;

    /// Converts something from a pixel buffer
    /// back to its original type
    fn from_pixel_buffer(&self) -> Self::Output;
}

impl<P> IntoBuffer<P> for ProtocolObject<dyn MTLBuffer>
where
    P: Copy + MetalFormat,
{
    type Output = Pixel<P>;

    fn into_pixel_buffer(&self) -> &mut [Self::Output] {
        let len = self.length() / mem::size_of::<Pixel<P>>();
        let ptr = self.contents().cast::<Pixel<P>>();

        let slice = unsafe {
            slice::from_raw_parts_mut(
                ptr.as_ptr(),
                len,
            )
        };

        slice
    }
}
