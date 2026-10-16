//! # Metal Texture
//! 
//! Wrapper around `MTLTexture`
//! 

use std::marker::PhantomData;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLTexture;

use crate::{metal::metal_format::MetalFormat};

/// Add docs later
pub struct MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    retain: Retained<ProtocolObject<dyn MTLTexture>>,
    _pixel_format_mark: PhantomData<P>,
}

impl<P> AsRef<Retained<ProtocolObject<dyn MTLTexture>>> for MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    fn as_ref(&self) -> &Retained<ProtocolObject<dyn MTLTexture>> {
        &self.retain
    }
}

impl<P> From<Retained<ProtocolObject<dyn MTLTexture>>> for MetalTexture<P>
where
    P: Copy + MetalFormat,
{
    fn from(value: Retained<ProtocolObject<dyn MTLTexture>>) -> Self {
        Self {
            retain: value,
            _pixel_format_mark: PhantomData,
        }
    }
}

impl<P> MetalTexture<P>
where
    P: Copy + MetalFormat,
{}
