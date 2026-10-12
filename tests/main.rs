//! # Tests

use metal_gpu::{adapter_desc::{ADI, AdapterDescription}};

#[test]
fn main() {
    let adapter = AdapterDescription::default()
        .switch_memory_layout()
        .build();

    println!("{:?}", adapter);
}
