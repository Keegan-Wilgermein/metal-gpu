//! # Metal Command Queue
//! 
//! Wrapper around `MTLCommandQueue`
//! 

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLCommandQueue;

use crate::metal::metal_command_buffer::MetalCommandBuffer;

/// Wrapper around `MTLCommandQueue`
pub struct MetalCommandQueue {
    retain: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl From<MetalCommandQueue> for Retained<ProtocolObject<dyn MTLCommandQueue>> {
    fn from(value: MetalCommandQueue) -> Self {
        value.retain
    }
}

impl From<Retained<ProtocolObject<dyn MTLCommandQueue>>> for MetalCommandQueue {
    fn from(value: Retained<ProtocolObject<dyn MTLCommandQueue>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl MetalCommandQueue {
    /// Creates a new command buffer
    pub(crate) fn command_buffer(&self) -> Option<MetalCommandBuffer> {
        self.retain.commandBuffer().map(|v| {
            v.into()
        })
    }
}
