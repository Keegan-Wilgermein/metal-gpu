//! # Metal Compile Options
//! 
//! Compilation options for a `.metal` file
//! 

use objc2::rc::Retained;
use objc2_metal::MTLCompileOptions;

/// Compile options for a `.metal` file
pub struct MetalCompileOptions {}

impl Default for MetalCompileOptions {
    fn default() -> Self {
        Self {}
    }
}

impl From<MTLCompileOptions> for MetalCompileOptions {
    fn from(_value: MTLCompileOptions) -> Self {
        Self::default()
    }
}

impl From<MetalCompileOptions> for Retained<MTLCompileOptions> {
    fn from(_value: MetalCompileOptions) -> Self {
        MTLCompileOptions::new()
    }
}
