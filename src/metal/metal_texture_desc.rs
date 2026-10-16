//! # Metal Texture Decription
//! 
//! Describes a texture decription
//! 

use std::marker::PhantomData;

use objc2::rc::Retained;
use objc2_metal::MTLTextureDescriptor;

use crate::metal::metal_format::MetalFormat;

/// A description of a metal texture
pub struct MetalTextureDecsription<P>
where
    P: Copy + MetalFormat,
{
    width: usize,
    height: usize,
    _pixel_format_mark: PhantomData<P>,
}

impl<P> From<MetalTextureDecsription<P>> for Retained<MTLTextureDescriptor>
where
    P: Copy + MetalFormat,
{
    fn from(value: MetalTextureDecsription<P>) -> Self {
        let out = MTLTextureDescriptor::new();
        out.setPixelFormat(P::PIXEL_FORMAT);
        unsafe { out.setWidth(value.width) };
        unsafe { out.setHeight(value.height) };

        out
    }
}

impl<P> MetalTextureDecsription<P>
where
    P: Copy + MetalFormat,
{
    /// Creates a new `MetalTextureDescription`
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            width,
            height,
            _pixel_format_mark: PhantomData,
        }
    }
}
