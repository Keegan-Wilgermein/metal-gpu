//! # Render strategy
//! 
//! Implementations and gating
//! for  `T` in a `Renderer`
//! 

use std::ops::Mul;
use crate::{api::dimensions::Dimensions, modules::{into_ext::IntoExtend, primitive::UPrimitive}};

/// Methods that the renderer must implement
/// on every kind of pipeline
pub trait RenderMethods {
    /// Creates a new `Self`
    /// with a length
    fn new<C, U>(
        length: C,
    ) -> Option<Self>
    where
        Self: Sized,
        C: Into<Dimensions<U>>,
        U: IntoExtend<usize>
            + UPrimitive
            + Copy
            + Mul<Output = U>;
}

/// Renderer gate
pub trait RenderStrategy {}

/// Add later
pub struct TextureShader;

impl RenderStrategy for TextureShader {}

/// Add later
pub struct ComputeShader;

impl RenderStrategy for ComputeShader {}

/// Add later
pub struct CPUBuffer;

impl RenderStrategy for CPUBuffer {}

/// Add later
pub struct TextureBuffer;

impl RenderStrategy for TextureBuffer {}
