//! # Metal Layer
//! 
//! Wrapper around `CAMetalLayer`
//! 

use std::marker::PhantomData;

use crate::metal::metal_format::MetalFormat;

/// Add docs later
pub struct MetalLayer<P>
where
    P: Copy + MetalFormat,
{
    _pixel_format_mark: PhantomData<P>,
}
