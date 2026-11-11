//! # Tests

use std::{time::Instant};

use metal_gpu::{api::{renderer::Renderer}, modules::{render_strategy::CPUBuffer}};

#[test]
fn main() {
    let width: u32 = 16_384;
    let height: u32 = 16_384;
    let renderer = Renderer::<CPUBuffer, u16>::new(
        (width, height),
    );

    if let Some(mut rend) = renderer {
        let now = Instant::now();

        for i in 1..=2 {
            rend.draw_frame(|buffer| {
                println!("Check 1: {:#?}", buffer[0]);
                buffer[0].set_all(5 * i, 855 * i, 300 * i, 855);
                println!("Check 2: {:#?}", buffer[0]);
            });
        }

        println!("Time to render: {:?}", now.elapsed());
    } else {
        println!("Fuck");
    }
}
