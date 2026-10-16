//! Metal Buffer
//! A buffer from the Metal API
//!

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLBuffer, MTLTextureDescriptor};

use crate::metal::{metal_format::MetalFormat, metal_texture::MetalTexture, metal_texture_desc::MetalTextureDecsription};

/// Add docs later
pub struct MetalBuffer {
    retain: Retained<ProtocolObject<dyn MTLBuffer>>,
}

impl From<Retained<ProtocolObject<dyn MTLBuffer>>> for MetalBuffer {
    fn from(value: Retained<ProtocolObject<dyn MTLBuffer>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl AsRef<ProtocolObject<dyn MTLBuffer>> for MetalBuffer {
    fn as_ref(&self) -> &ProtocolObject<dyn MTLBuffer> {
        &self.retain
    }
}

impl MetalBuffer {
    pub(crate) fn new_texture_with_offset<P>(
        &self,
        desc: MetalTextureDecsription<P>,
        offset: usize,
        bytes_per_row: usize,
    ) -> Option<MetalTexture<P>>
    where
        P: Copy + MetalFormat,
    {
        let desc: Retained<MTLTextureDescriptor> = desc.into();
        self.retain.newTextureWithDescriptor_offset_bytesPerRow(
            &desc,
            offset,
            bytes_per_row,
        ).map(|v| {
            v.into()
        })
    }
}
