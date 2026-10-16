//! # Metal Layer
//! 
//! Wrapper around `CAMetalLayer`
//! 

use std::marker::PhantomData;

use objc2::rc::Retained;
use objc2_quartz_core::CAMetalLayer;

use crate::{api::metal_device::MetalDevice, metal::metal_format::MetalFormat};

/// Add docs later
pub struct MetalLayer<P>
where
    P: Copy + MetalFormat,
{
    retain: Retained<CAMetalLayer>,
    _pixel_format_mark: PhantomData<P>,
}

impl<P> Default for MetalLayer<P>
    where
    P: Copy + MetalFormat,
{
    fn default() -> Self {
        let layer = CAMetalLayer::new();
        layer.setPixelFormat(P::PIXEL_FORMAT);

        Self {
            retain: layer,
            _pixel_format_mark: PhantomData,
        }
    }
}

impl<P> AsRef<CAMetalLayer> for MetalLayer<P>
where
    P: Copy + MetalFormat,
{
    fn as_ref(&self) -> &CAMetalLayer {
        &self.retain
    }
}

impl<P> MetalLayer<P>
where
    P: Copy + MetalFormat,
{
    /// Sets the device the `MetalLayer` uses
    pub fn set_device(&self, device: &MetalDevice) {
        self.retain.setDevice(Some(device.as_ref()));
    }

    /// Sets the device the `MetalLayer` uses
    /// and returns itself
    pub fn with_device(self, device: &MetalDevice) -> Self {
        self.set_device(device);
        self
    }

    /// Sets the value of frame_buffer_only
    pub fn set_frame_buffer_only(&self, buffer_only: bool) {
        self.retain.setFramebufferOnly(buffer_only);
    }

    /// Sets the value of frame_buffer_only
    /// and returns itself
    pub fn with_frame_buffer_only(self, buffer_only: bool) -> Self {
        self.set_frame_buffer_only(buffer_only);
        self
    }
}

