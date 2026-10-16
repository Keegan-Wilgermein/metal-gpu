//! # Metal Function
//! 
//! A wrapper around `MTLFunction`
//! 

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLFunction;

/// Wrapper around `MTLFunction`
pub struct MetalFunction {
    retain: Retained<ProtocolObject<dyn MTLFunction>>,
}

impl From<Retained<ProtocolObject<dyn MTLFunction>>> for MetalFunction {
    fn from(value: Retained<ProtocolObject<dyn MTLFunction>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl From<MetalFunction> for Retained<ProtocolObject<dyn MTLFunction>> {
    fn from(value: MetalFunction) -> Self {
        value.retain
    }
}

impl AsRef<ProtocolObject<dyn MTLFunction>> for MetalFunction {
    fn as_ref(&self) -> &ProtocolObject<dyn MTLFunction> {
        &self.retain
    }
}
