//! # Metal Compute Pipeline
//!
//! A wrapper around `MTLComputePipelineState`
//!

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLComputePipelineState;

/// Wrapper around `MTLComputePipelineState`
pub struct MetalComputePipeline {
    retain: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
}

impl From<Retained<ProtocolObject<dyn MTLComputePipelineState>>> for MetalComputePipeline {
    fn from(value: Retained<ProtocolObject<dyn MTLComputePipelineState>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl From<MetalComputePipeline> for Retained<ProtocolObject<dyn MTLComputePipelineState>> {
    fn from(value: MetalComputePipeline) -> Self {
        value.retain
    }
}

impl AsRef<ProtocolObject<dyn MTLComputePipelineState>> for MetalComputePipeline {
    fn as_ref(&self) -> &ProtocolObject<dyn MTLComputePipelineState> {
        &self.retain
    }
}

impl MetalComputePipeline {
    /// Gets the number of threads the GPU
    /// runs together in lockstep
    pub(crate) fn thread_execution_width(&self) -> usize {
        self.retain.threadExecutionWidth()
    }

    /// Gets the maximum number of threads
    /// in a single threadgroup
    pub(crate) fn max_threads_per_threadgroup(&self) -> usize {
        self.retain.maxTotalThreadsPerThreadgroup()
    }
}
