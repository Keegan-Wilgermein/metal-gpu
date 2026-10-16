//! # Metal Texture
//! 
//! Wrapper around `MTLTexture`
//! 

use std::{marker::PhantomData, mem};

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLTexture};

use crate::{api::{metal_device::MetalDevice, pixel::Pixel}, metal::{metal_buffer::MetalBuffer, metal_format::MetalFormat, metal_texture_desc::MetalTextureDecsription}, modules::byte_alignment::ByteAlignment};

/// Add docs later
pub struct MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    retain: Retained<ProtocolObject<dyn MTLTexture>>,
    _pixel_format_mark: PhantomData<P>,
}

impl<P> AsRef<Retained<ProtocolObject<dyn MTLTexture>>> for MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    fn as_ref(&self) -> &Retained<ProtocolObject<dyn MTLTexture>> {
        &self.retain
    }
}

impl<P> From<Retained<ProtocolObject<dyn MTLTexture>>> for MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    fn from(value: Retained<ProtocolObject<dyn MTLTexture>>) -> Self {
        Self {
            retain: value,
            _pixel_format_mark: PhantomData,
        }
    }
}

impl<P> MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    pub(crate) fn new(
        device: &MetalDevice,
        buffer: &MetalBuffer,
        desc: MetalTextureDecsription<P>,
        width: usize,
    ) -> Option<MetalTexture<P>>
    where
        P: Copy + MetalFormat,
    {
        // Metal aborts the process instead of failing
        // if a 2D texture is larger than 16384 in either
        // direction, the limit on every Mac GPU
        if desc.get_width() > 16384 || desc.get_height() > 16384 {
            return None;
        }

        let bytes_per_row = device.min_linear_texture_alignment_for_format::<P>()
            .check_alignment(width * mem::size_of::<Pixel<P>>()).ok()?;

        Some(buffer.new_texture_with_offset(
            desc,
            0,
            bytes_per_row,
        )?)
    }

    /// gets the width of the texture
    pub(crate) fn get_width(&self) -> usize {
        self.retain.width()
    }

    /// gets the height of the texture
    pub(crate) fn get_height(&self) -> usize {
        self.retain.height()
    }
}
