//! # Metal Compute Command Encoder
//!
//! Wrapper around `MTLComputeCommandEncoder`
//!

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLCommandEncoder, MTLComputeCommandEncoder};

use crate::metal::{metal_compute_pipeline::MetalComputePipeline, metal_format::MetalFormat, metal_grid::MetalGrid, metal_texture::MetalTexture};

/// The number of texture slots every Metal
/// device supports for a compute kernel
const MAX_TEXTURE_INDEX: usize = 31;

/// Wrapper around `MTLComputeCommandEncoder`
pub struct MetalComputeCommandEncoder {
    retain: Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>,
}

impl From<Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>> for MetalComputeCommandEncoder {
    fn from(value: Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl From<MetalComputeCommandEncoder> for Retained<ProtocolObject<dyn MTLComputeCommandEncoder>> {
    fn from(value: MetalComputeCommandEncoder) -> Self {
        value.retain
    }
}

impl MetalComputeCommandEncoder {
    /// Sets the pipeline that
    /// following dispatches run
    pub(crate) fn set_pipeline(&self, pipeline: &MetalComputePipeline) {
        self.retain.setComputePipelineState(pipeline.as_ref());
    }

    /// Binds `texture` to the kernel's
    /// `[[texture(index)]]` argument
    ///
    /// # Panics
    /// If `index` is `MAX_TEXTURE_INDEX` or more
    pub(crate) fn set_texture<P>(&self, texture: &MetalTexture<P>, index: usize)
    where
        P: Copy + MetalFormat,
    {
        assert!(
            index < MAX_TEXTURE_INDEX,
            "texture index {index} is out of range, the limit is {MAX_TEXTURE_INDEX}",
        );

        // SAFETY: `index` is checked above, and command buffers made with
        // `commandBuffer` retain the textures bound to them until they finish
        unsafe { self.retain.setTexture_atIndex(Some(texture.as_ref()), index) };
    }

    /// Runs the pipeline once for every thread in `grid`,
    /// in groups of `group` threads
    ///
    /// `grid` doesn't need to be a multiple
    /// of `group`, so kernels should bounds
    /// check their thread position
    pub(crate) fn dispatch_threads(&self, grid: MetalGrid, group: MetalGrid) {
        self.retain.dispatchThreads_threadsPerThreadgroup(grid.into(), group.into());
    }

    /// Finishes encoding commands
    ///
    /// Must be called before the command buffer
    /// is committed, and the encoder can't be
    /// used afterwards, so this takes `self`
    pub(crate) fn end_encoding(self) {
        self.retain.endEncoding();
    }
}
