//! # Metal Texture Decription
//! 
//! Describes a texture decription
//! 

use std::marker::PhantomData;

use objc2::rc::Retained;
use objc2_metal::MTLTextureDescriptor;

use crate::metal::{metal_format::MetalFormat, metal_texture_usage::MetalTextureUsage};

/// A description of a metal texture
pub struct MetalTextureDecsription<P>
where
    P: Copy + MetalFormat,
{
    width: usize,
    height: usize,
    usage: MetalTextureUsage,
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
        out.setUsage(value.usage.into());

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
        usage: MetalTextureUsage,
    ) -> Self {
        Self {
            width,
            height,
            usage,
            _pixel_format_mark: PhantomData,
        }
    }

    /// Gets the width of the texture
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the texture
    pub fn get_height(&self) -> usize {
        self.height
    }
}
