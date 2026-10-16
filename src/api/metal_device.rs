//! # MetalDevice
//! 
//! Metal api interface
//! 

use std::hash::Hash;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSString};
use objc2_metal::{MTLDevice};

use crate::metal::{metal_buffer::MetalBuffer, metal_command_queue::MetalCommandQueue, metal_compile_options::MetalCompileOptions, metal_compute_pipeline::MetalComputePipeline, metal_format::MetalFormat, metal_function::MetalFunction, metal_library::MetalLibrary, metal_resources::MetalResources};

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

    /// Creates a new command queue
    pub(crate) fn new_command_queue(&self) -> Option<MetalCommandQueue> {
        self.retain.newCommandQueue().map(|retain| {
            retain.into()
        })
    }

    /// Creates a new library
    pub(crate) fn new_library(
        &self,
        source: &str,
        _options: Option<MetalCompileOptions>,
    ) -> Result<MetalLibrary, Retained<NSError>> {
        self.retain.newLibraryWithSource_options_error(
            &NSString::from_str(source),
            None,
        ).map(|v| {
            v.into()
        })
    }

    /// Creates a new compute pipeline
    pub(crate) fn new_compute_pipeline(
        &self,
        function: MetalFunction,
    ) -> Result<MetalComputePipeline, Retained<NSError>> {
        self.retain.newComputePipelineStateWithFunction_error(
            function.as_ref()
        ).map(|v| {
            v.into()
        })
    }
}
