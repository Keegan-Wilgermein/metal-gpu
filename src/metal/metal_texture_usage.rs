//! # Metal Texture Usage
//!
//! Wrapper around `MTLTextureUsage`
//!

use bitflags::bitflags;
use objc2_metal::MTLTextureUsage;

bitflags! {
    /// Ways a texture can be used
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MetalTextureUsage: usize {
        #[allow(non_upper_case_globals)]
        const ShaderRead = MTLTextureUsage::ShaderRead.0;
        #[allow(non_upper_case_globals)]
        const ShaderWrite = MTLTextureUsage::ShaderWrite.0;
        #[allow(non_upper_case_globals)]
        const RenderTarget = MTLTextureUsage::RenderTarget.0;
        #[allow(non_upper_case_globals)]
        const PixelFormatView = MTLTextureUsage::PixelFormatView.0;
        #[allow(non_upper_case_globals)]
        const ShaderAtomic = MTLTextureUsage::ShaderAtomic.0;
    }
}

impl MetalTextureUsage {
    #[allow(non_upper_case_globals)]
    pub const Unknown: Self = Self::empty();
}

impl Default for MetalTextureUsage {
    fn default() -> Self {
        Self::ShaderRead
    }
}

impl From<MetalTextureUsage> for MTLTextureUsage {
    fn from(value: MetalTextureUsage) -> Self {
        MTLTextureUsage(value.bits())
    }
}
