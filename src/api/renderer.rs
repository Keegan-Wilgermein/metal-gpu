//! # Renderer
//! 
//! The renderer used to make calls to the gpu
//! 

use std::{marker::PhantomData, mem, ops::Mul};

use crate::{api::{dimensions::Dimensions, metal_device::MetalDevice, pixel::Pixel}, functions::get_default_device, metal::{metal_buffer::MetalBuffer, metal_format::MetalFormat, metal_resources::MetalResources, metal_texture::MetalTexture, metal_texture_desc::MetalTextureDecsription}, modules::{buffer_conversion::IntoBuffer, into_ext::IntoExtend, primitive::UPrimitive, render_strategy::{CPUBuffer, RenderMethods, RenderStrategy}}};

pub struct Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat,
{
    device: MetalDevice,
    texture: MetalTexture<P>,
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
    P: Copy + MetalFormat + 'static,
{
    /// Creates a new `Renderer`
    pub fn new<C>(
        length: C,
    ) -> Option<Self>
    where
        C: Into<Dimensions<C>>
        + IntoExtend<usize>
        + UPrimitive
        + Copy
        + Mul<Output = C>,
    {
        let length = length.into();
        let device = if let Some(d) = get_default_device()
            { d } else { return None };

        let buffer = if let Some(b) =
            device.create_buffer(
                length
                    .get_size()
                    .into_idx() * mem::size_of::<Pixel<P>>(),
                MetalResources::StorageModeShared,
            ) { b } else { return None };

        let desc = MetalTextureDecsription::new(
            length.get_x_length().into_idx(),
            length.get_y_length().into_idx(),
        );

        let texture = if let Some(t) = device.new_texture(desc)
            { t } else { return None };

        Some(Self {
            device,
            texture,
            buffer,
            _render_mark: PhantomData,
        })
    }

    /// Draws a frame
    pub fn draw_frame<F>(
        &mut self,
        callback: F,
    )
    where
        F: FnOnce(&mut [Pixel<P>])
    {
        let rgba_buffer: &mut [Pixel<P>] = self.buffer.as_ref()
            .into_pixel_buffer();

        (callback)(rgba_buffer);
    }
}
