//! # Tests

use metal_gpu::{adapter_desc::{ADI, AdapterDescription}, image::save::SaveImage};

#[test]
fn main() {
    let adapter = AdapterDescription::default()
        .switch_memory_layout()
        .build();

    adapter.save_png();
    println!("{:?}", adapter);
}
