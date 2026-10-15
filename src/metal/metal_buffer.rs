//! Metal Buffer
//! A buffer from the Metal API
//!

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLBuffer;

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
