//! # Renderer
//! 
//! The renderer used to make calls to the gpu
//! 

use std::{fs::File, io::BufWriter, marker::PhantomData, mem, ops::Mul, path::PathBuf};

use objc2_metal::{MTLTexture};
use png::{ColorType, Encoder, EncodingError};

use crate::{api::{dimensions::Dimensions, pixel::Pixel}, functions::get_default_device, metal::{metal_buffer::MetalBuffer, metal_command_queue::MetalCommandQueue, metal_compute_pipeline::MetalComputePipeline, metal_format::MetalFormat, metal_grid::MetalGrid, metal_resources::MetalResources, metal_texture::MetalTexture, metal_texture_desc::MetalTextureDecsription, metal_texture_usage::MetalTextureUsage}, modules::{buffer_conversion::IntoBuffer, into_ext::IntoExtend, primitive::UPrimitive, render_strategy::{CPUBuffer, ComputeShader, RenderMethods, RenderStrategy}, save::{PngChannel, SaveImage}}};

pub struct Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat,
{
    texture: MetalTexture<P>,
    buffer: MetalBuffer,
    command: Option<MetalCommandQueue>,
    pipeline: Option<MetalComputePipeline>,
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

impl<P> RenderMethods for Renderer<ComputeShader, P>
where
    P: Copy + MetalFormat,
{
    fn new<C, U>(
        length: C,
    ) -> Option<Self>
    where
        Self: Sized,
        C: Into<Dimensions<U>>,
        U: IntoExtend<usize>
            + UPrimitive
            + Copy
            + Mul<Output = U>
    {
        let length = length.into();
        let device = get_default_device()?;

        let buffer = device.create_buffer(
            length.get_size().into_idx() * mem::size_of::<Pixel<P>>(),
            MetalResources::StorageModeShared,
        )?;

        let desc = MetalTextureDecsription::new(
            length.get_x_length().into_idx(),
            length.get_y_length().into_idx(),
            MetalTextureUsage::ShaderWrite,
        );

        let texture = MetalTexture::new(
            &device,
            &buffer,
            desc,
            length.get_x_length().into_idx(),
        )?;

        let command = device.new_command_queue()?;
        let library = device.new_library(
            include_str!("../metal/shaders/library.metal"),
            None,
        ).ok()?;

        let function = library.new_function("triangle")?;
        let pipeline = device.new_compute_pipeline(function).ok()?;

        Some(Self {
            buffer,
            texture,
            command: Some(command),
            pipeline: Some(pipeline),
            _render_mark: PhantomData,
        })
    }
}

impl<P> RenderMethods for Renderer<CPUBuffer, P>
where
    P: Copy + MetalFormat,
{
    /// Creates a new `Renderer`
    /// with the `CPUBuffer` strategy
    /// 
    /// The `CPUBuffer` stategy renders
    /// everything on the CPU and never
    /// touches the GPU for any render tasks
    /// 
    /// This method will return `None`
    /// if creating the relevant buffers
    /// fails or the value of `length` is
    /// not a multiple of the size of `P`
    fn new<C, U>(
        length: C,
    ) -> Option<Self>
    where
        C: Into<Dimensions<U>>,
        U: IntoExtend<usize>
            + UPrimitive
            + Copy
            + Mul<Output = U>,
    {
        let length = length.into();
        let device = get_default_device()?;

        let buffer = device.create_buffer(
                length
                    .get_size()
                    .into_idx() * mem::size_of::<Pixel<P>>(),
                MetalResources::StorageModeShared,
            )?;

        let desc = MetalTextureDecsription::new(
            length.get_x_length().into_idx(),
            length.get_y_length().into_idx(),
            MetalTextureUsage::ShaderRead,
        );

        let texture = MetalTexture::new(
            &device,
            &buffer,
            desc,
            length.get_x_length().into_idx(),
        )?;

        Some(Self {
            texture,
            buffer,
            command: None,
            pipeline: None,
            _render_mark: PhantomData,
        })
    }
}

// impl<P> RenderMethods for Renderer<TextureBuffer, P>
// where
//     P: Copy + MetalFormat,
// {}

impl<P> Renderer<CPUBuffer, P>
where
    P: Copy + MetalFormat + 'static,
{
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

impl<P> Renderer<ComputeShader, P>
where
    P: Copy + MetalFormat,
{
    /// Draws a frame
    pub fn draw_frame(
        &mut self,
    ) -> Result<(), ()> {
        let command = self.command.as_ref().ok_or(())?;
        let pipeline = self.pipeline.as_ref().ok_or(())?;

        let cmd_buffer = command.command_buffer().ok_or(())?;
        let encoder = cmd_buffer.compute_command_encoder().ok_or(())?;

        encoder.set_pipeline(&pipeline);
        encoder.set_texture(&self.texture, 0);

        let grid = MetalGrid::from(
            (self.texture.get_width(), self.texture.get_height()),
        );

        let group = MetalGrid::from(
            (
                pipeline.thread_execution_width(),
                pipeline.max_threads_per_threadgroup() / pipeline.thread_execution_width(),
            ),
        );

        encoder.dispatch_threads(grid, group);
        encoder.end_encoding();

        cmd_buffer.commit();
        cmd_buffer.wait();
        
        if cmd_buffer.errors().is_some() {
            return Err(());
        }

        Ok(())
    }
}

impl<T, P> SaveImage for Renderer<T, P>
where
    T: RenderStrategy,
    P: Copy + MetalFormat + PngChannel + 'static,
{
    fn save_png(&self, path: impl Into<PathBuf>) -> Result<(), EncodingError> {
        let frame: &mut [Pixel<P>] = self.buffer.as_ref().into_pixel_buffer();

        let width = u32::try_from(self.texture.as_ref().width())
            .map_err(|_| EncodingError::LimitsExceeded)?;
        let height = u32::try_from(self.texture.as_ref().height())
            .map_err(|_| EncodingError::LimitsExceeded)?;

        let mut data = Vec::with_capacity(frame.len() * mem::size_of::<Pixel<P>>());
        for pixel in frame.iter() {
            pixel.get_red().write_sample(&mut data);
            pixel.get_green().write_sample(&mut data);
            pixel.get_blue().write_sample(&mut data);
            pixel.get_alpha().write_sample(&mut data);
        }

        let file = File::create(path.into())?;

        let mut encoder = Encoder::new(BufWriter::new(file), width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(P::BIT_DEPTH);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;
        writer.finish()?;

        Ok(())
    }
}
