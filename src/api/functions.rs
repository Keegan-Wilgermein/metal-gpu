//! # Functions
//! 
//! Standalone functions for accessing data
//! 

use objc2_metal::{MTLCopyAllDevices, MTLCreateSystemDefaultDevice};

use crate::api::metal_device::MetalDevice;

/// Gets the current system default graphics device
pub fn get_default_device() -> Option<MetalDevice> {
    MTLCreateSystemDefaultDevice()
        .map(|retain| {
            MetalDevice::from(retain)
        }
    )
}

/// Gets the available graphics devices
pub fn get_available_devices() -> Vec<MetalDevice> {
    MTLCopyAllDevices().into_iter()
    .map(|retain| {
        MetalDevice::from(retain)
    }).collect()
}
