//! # MetalDevice
//! 
//! Metal api interface
//! 

use std::hash::Hash;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLDevice;

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
