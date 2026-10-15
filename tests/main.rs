//! # Tests

use metal_gpu::{api::{dimensions::dimensions_2d::Dimensions2D, renderer::Renderer}, modules::render_strategy::CPUBuffer};

#[test]
fn main() {
    let dimensions = Dimensions2D::from((50_u8, 50));
    let renderer = Renderer::<CPUBuffer, u16>::new(dimensions);
}
