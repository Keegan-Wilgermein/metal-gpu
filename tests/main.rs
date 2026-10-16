//! # Tests

use std::{env::current_dir, time::Instant};

use metal_gpu::{api::renderer::Renderer, modules::{render_strategy::{ComputeShader, RenderMethods}, save::SaveImage}};

#[test]
fn main() {
    let width: u32 = 16_384;
    let height: u32 = 16_384;
    let renderer = Renderer::<ComputeShader, u16>::new((width, height));

    if let Some(mut rend) = renderer {
        let now = Instant::now();

        match rend.draw_frame() {
            Ok(_) => (),
            Err(_) => println!("Failed to draw"),
        }

        println!("Time to render: {:?}", now.elapsed());

        if let Ok(mut path) = current_dir() {
            let now = Instant::now();

            path.push("tests/files.nosync/triangle_compute_shader.png");

            match rend.save_png(path) {
                Ok(_) => (),
                Err(err) => println!("Failed to save with: {}", err),
            }

            println!("time to save: {:?}", now.elapsed());
        }
    }
}
