//! # Tests

use std::env::current_dir;

use metal_gpu::{api::renderer::Renderer, modules::{render_strategy::CPUBuffer, save::SaveImage}};

#[test]
fn main() {
    let renderer = Renderer::<CPUBuffer, u16>::new(64_u32);

    if let Some(mut rend) = renderer {
        rend.draw_frame(|buffer| {
            // Top, bottom left and bottom right corners
            let (v0, v1, v2) = ((32.0, 4.0), (4.0, 60.0), (60.0, 60.0));

            // Twice the signed area of the triangle `a`, `b`, `p`
            let edge = |a: (f32, f32), b: (f32, f32), p: (f32, f32)| {
                (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)
            };
            let area = edge(v0, v1, v2);

            buffer.iter_mut().enumerate().for_each(|(i, pixel)| {
                // Samples the centre of the pixel
                let p = ((i % 64) as f32 + 0.5, (i / 64) as f32 + 0.5);

                // How much of each corner's colour this pixel gets
                let w0 = edge(v1, v2, p) / area;
                let w1 = edge(v2, v0, p) / area;
                let w2 = edge(v0, v1, p) / area;

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    pixel.set_red((w0 * u16::MAX as f32) as u16);
                    pixel.set_green((w1 * u16::MAX as f32) as u16);
                    pixel.set_blue((w2 * u16::MAX as f32) as u16);
                    pixel.set_alpha(u16::MAX);
                }
            });
        });

        if let Ok(mut path) = current_dir() {
            path.push("tests/files/triangle.png");

            match rend.save_png(path) {
                Ok(_) => (),
                Err(err) => println!("Failed to save with: {}", err),
            }
        }
    }
}
