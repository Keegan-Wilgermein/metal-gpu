//! # Render strategy
//! 
//! Implementations and gating
//! for  `T` in a `Renderer`
//! 

/// Renderer gate
pub trait RenderStrategy {}

/// Add later
pub struct TextureShader;

impl RenderStrategy for TextureShader {}

/// Drawn from a predefined shader file
pub struct ComputeShader;

impl RenderStrategy for ComputeShader {}

/// A buffer used by the CPU to draw pixel by pixel
pub struct CPUBuffer;

impl RenderStrategy for CPUBuffer {}

/// Add later
pub struct TextureBuffer;

impl RenderStrategy for TextureBuffer {}
