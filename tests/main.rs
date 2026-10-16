//! # Tests

use metal_gpu::{api::{renderer::Renderer}, modules::render_strategy::CPUBuffer};

#[test]
fn main() {
    let renderer = Renderer::<CPUBuffer, u16>::new(50_u32);

    if let Some(mut rend) = renderer {
        rend.draw_frame(|buffer| {
            println!("{}", buffer.len())
        });
    }
}
