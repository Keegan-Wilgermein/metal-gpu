//! # Metal Resources
//! 
//! Wrapper around `MTLResourceOptions`
//! 

use objc2_metal::MTLResourceOptions;

/// Thin wrapper around `MTLResourceOptions`
/// 
/// Individual `MTLResourceOptions` values.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum MetalResources {
    CPUCacheModeDefaultCache = MTLResourceOptions::CPUCacheModeDefaultCache.0,
    CPUCacheModeWriteCombined = MTLResourceOptions::CPUCacheModeWriteCombined.0,
    StorageModeManaged = MTLResourceOptions::StorageModeManaged.0,
    StorageModePrivate = MTLResourceOptions::StorageModePrivate.0,
    StorageModeMemoryless = MTLResourceOptions::StorageModeMemoryless.0,
    HazardTrackingModeUntracked = MTLResourceOptions::HazardTrackingModeUntracked.0,
    HazardTrackingModeTracked = MTLResourceOptions::HazardTrackingModeTracked.0,
}

impl MetalResources {
    // Have the same value as `Self::CPUCacheModeDefaultCache` so they are constants
    // allowing them to be called normally
    #[allow(non_upper_case_globals)]
    pub const StorageModeShared: Self = Self::CPUCacheModeDefaultCache;
    #[allow(non_upper_case_globals)]
    pub const HazardTrackingModeDefault: Self = Self::CPUCacheModeDefaultCache;
}

impl From<MetalResources> for MTLResourceOptions {
    fn from(value: MetalResources) -> Self {
        Self(value as usize)
    }
}
