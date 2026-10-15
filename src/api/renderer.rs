//! # Renderer
//! 
//! The renderer used to make calls to the gpu
//! 

use std::{marker::PhantomData};
use crate::{api::metal_device::MetalDevice, functions::get_default_device, metal::{metal_buffer::MetalBuffer, metal_format::MetalFormat, metal_layer::MetalLayer, metal_resources::MetalResources}, modules::{dimensional::Dimensional, render_strategy::{CPUBuffer, RenderMethods, RenderStrategy}}};

pub struct Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat,
{
    device: MetalDevice,
    layer: MetalLayer<P>,
    buffer: MetalBuffer,
    _render_mark: PhantomData<T>,
}

impl<T, P> Drop for Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat,
{
    fn drop(&mut self) {}
}

// impl<P> RenderMethods for Renderer<TextureShader, P>
// where
//     P: Copy + MetalFormat,
// {}

// impl<P> RenderMethods for Renderer<ComputeShader, P>
// where
//     P: Copy + MetalFormat,
// {}

impl<P> RenderMethods for Renderer<CPUBuffer, P>
where
    P: Copy + MetalFormat,
{}

// impl<P> RenderMethods for Renderer<TextureBuffer, P>
// where
//     P: Copy + MetalFormat,
// {}

impl<T, P> Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat,
{
    /// Creates a new `Renderer`
    pub fn new<D, U>(
        length: D,
    ) -> Option<Self>
    where
        D: Dimensional + Dimensional<Output = U>,
        U: Into<usize>,
    {
        let device = if let Some(d) = get_default_device()
            { d } else { return None };

        let buffer = if let Some(b) =
            device.create_buffer(
                length.get_size().into(),
                MetalResources::StorageModeShared,
            ) { b } else { return None };
        
        let layer = MetalLayer::default()
            .with_device(&device)
            .with_frame_buffer_only(false);

        Some(Self {
            device,
            layer,
            buffer,
            _render_mark: PhantomData,
        })
    }
}
