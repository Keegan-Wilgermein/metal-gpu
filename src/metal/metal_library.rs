//! # Metal Library
//! 
//! A wrapper around `MTLLibrary`
//! 

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSString;
use objc2_metal::MTLLibrary;

use crate::metal::metal_function::MetalFunction;

/// Wrapper around `MTLLibrary`
pub struct MetalLibrary {
    retain: Retained<ProtocolObject<dyn MTLLibrary>>,
}

impl From<Retained<ProtocolObject<dyn MTLLibrary>>> for MetalLibrary {
    fn from(value: Retained<ProtocolObject<dyn MTLLibrary>>) -> Self {
        Self {
            retain: value,
        }
    }
}

impl From<MetalLibrary> for Retained<ProtocolObject<dyn MTLLibrary>> {
    fn from(value: MetalLibrary) -> Self {
        value.retain
    }
}

impl MetalLibrary {
    pub(crate) fn new_function(&self, name: &str) -> Option<MetalFunction> {
        self.retain.newFunctionWithName(
            &NSString::from_str(name),
        ).map(|v| {
            v.into()
        })
    }
}
