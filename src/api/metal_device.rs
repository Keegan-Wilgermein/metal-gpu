//! # MetalDevice
//! 
//! Metal api interface
//! 

use std::hash::Hash;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLDevice};

use crate::metal::{metal_buffer::MetalBuffer, metal_format::MetalFormat, metal_resources::MetalResources};

/// Metal api interface
pub struct MetalDevice {
    retain: Retained<ProtocolObject<dyn MTLDevice>>,
}

impl Hash for MetalDevice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.get_id());
    }
}

impl From<Retained<ProtocolObject<dyn MTLDevice>>> for MetalDevice {
    fn from(value: Retained<ProtocolObject<dyn MTLDevice>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl From<MetalDevice> for Retained<ProtocolObject<dyn MTLDevice>> {
    fn from(value: MetalDevice) -> Self {
        value.retain
    }
}

impl AsRef<ProtocolObject<dyn MTLDevice>> for MetalDevice {
    fn as_ref(&self) -> &ProtocolObject<dyn MTLDevice> {
        &self.retain
    }
}

impl MetalDevice {
    /// Gets the devices name
    pub fn get_name(&self) -> String {
        self.retain.name().to_string()
    }

    /// Gets the id of the device
    pub fn get_id(&self) -> u64 {
        self.retain.registryID()
    }
}

// Crate only implementations

impl MetalDevice {
    /// Creates a buffer
    pub(crate) fn create_buffer(
        &self,
        length: usize,
        options: MetalResources,
    ) -> Option<MetalBuffer> {
        self.retain.newBufferWithLength_options(
            length,
            options.into(),
        ).map(|retain| {
            MetalBuffer::from(retain)
        })
    }

    /// Gets the minimum linear texture alignment
    /// for a given pixel format
    pub(crate) fn min_linear_texture_alignment_for_format<P>(&self) -> usize
    where
        P: Copy + MetalFormat,
    {
        self.retain.minimumLinearTextureAlignmentForPixelFormat(P::PIXEL_FORMAT)
    }
}
