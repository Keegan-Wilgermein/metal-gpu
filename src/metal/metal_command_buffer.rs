//! # Metal Command Buffer
//! 
//! Wrapper around `MTLCommandBuffer`
//! 

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;
use objc2_metal::MTLCommandBuffer;

use crate::metal::metal_compute_command_encoder::MetalComputeCommandEncoder;

/// Wrapper around `MTLCommandBuffer`
pub struct MetalCommandBuffer {
    retain: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
}

impl From<MetalCommandBuffer> for Retained<ProtocolObject<dyn MTLCommandBuffer>> {
    fn from(value: MetalCommandBuffer) -> Self {
        value.retain
    }
}

impl From<Retained<ProtocolObject<dyn MTLCommandBuffer>>> for MetalCommandBuffer {
    fn from(value: Retained<ProtocolObject<dyn MTLCommandBuffer>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl MetalCommandBuffer {
    /// Creates a new command encoder
    pub(crate) fn compute_command_encoder(&self) -> Option<MetalComputeCommandEncoder> {
        self.retain.computeCommandEncoder().map(|v| {
            v.into()
        })
    }

    /// Commits the work and sends
    /// it to the GPU
    pub(crate) fn commit(&self) {
        self.retain.commit();
    }

    /// Waits until the GPU
    /// has finished its work
    pub(crate) fn wait(&self) {
        self.retain.waitUntilCompleted();
    }

    /// Checks if any errors occured
    pub(crate) fn errors(&self) -> Option<Retained<NSError>> {
        self.retain.error()
    }
}
