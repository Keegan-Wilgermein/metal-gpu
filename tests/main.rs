//! # Tests

use metal_gpu::{api::{render_strategy::CPUBuffer, renderer::Renderer}, functions::{get_available_devices, get_default_device}};

#[test]
fn main() {
    let renderer = Renderer::<CPUBuffer, u16>::new();
}
