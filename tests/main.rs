//! # Tests

use metal_gpu::functions::{get_available_devices, get_default_device};

#[test]
fn main() {
    let maybe_device = get_default_device();

    if let Some(device) = maybe_device {
        println!("Default device: {}", device.get_name());
    }

    let devices = get_available_devices();
    devices.iter().for_each(|device| {
        println!(
            "{} : {}",
            device.get_name(),
            device.get_id(),
        );
    });
}
