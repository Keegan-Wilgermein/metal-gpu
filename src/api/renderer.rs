//! # Renderer
//! 
//! The renderer used to make calls to the gpu
//! 

use std::marker::PhantomData;
use crate::{api::{metal_device::MetalDevice, render_strategy::{CPUBuffer, ComputeShader, RenderMethods, RenderStrategy, TextureBuffer, TextureShader}}, functions::get_default_device, modules::metal_format::MetalFormat};

pub struct Renderer<T, P>
where
    T: RenderStrategy,
    P: Clone + Copy + MetalFormat,
{
    device: MetalDevice,
    _render_mark: PhantomData<T>,
    _pixel_format_mark: PhantomData<P>,
}

impl<T, P> Drop for Renderer<T, P>
where
    T: RenderStrategy,
    P: Clone + Copy + MetalFormat,
{
    fn drop(&mut self) {}
}

impl<T, P> From<MetalDevice> for Renderer<T, P>
where
    T: RenderStrategy,
    P: Clone + Copy + MetalFormat,
{
    fn from(value: MetalDevice) -> Self {
        Self {
            device: value,
            _render_mark: PhantomData,
            _pixel_format_mark: PhantomData,
        }
    }
}

// impl<P> RenderMethods for Renderer<TextureShader, P>
// where
//     P: Clone + Copy + MetalFormat,
// {}

// impl<P> RenderMethods for Renderer<ComputeShader, P>
// where
//     P: Clone + Copy + MetalFormat,
// {}

impl<P> RenderMethods for Renderer<CPUBuffer, P>
where
    P: Clone + Copy + MetalFormat,
{}

// impl<P> RenderMethods for Renderer<TextureBuffer, P>
// where
//     P: Clone + Copy + MetalFormat,
// {}

impl<T, P> Renderer<T, P>
where
    T: RenderStrategy,
    P: Clone + Copy + MetalFormat,
{

    /// Creates a new `Renderer`
    pub fn new() -> Option<Self> {
        let device = if let Some(d) = get_default_device()
            { d } else { return None };

        Some(Self {
            device,
            _render_mark: PhantomData,
            _pixel_format_mark: PhantomData
        })
    }
}
