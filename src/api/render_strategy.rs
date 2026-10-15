//! # Render strategy
//! 
//! Implementations and gating
//! for  `T` in a `Renderer`
//! 

/// Methods that the renderer must implement
/// on every kind of pipeline
pub trait RenderMethods {}

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
