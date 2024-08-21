use std::time::Duration;

use libmapper_rs::device::Device;

fn main() {
    println!("Using libmapper version {} ", libmapper_rs::get_mapper_version());
    let dev = Device::create("rustmapper");
    loop {
        dev.poll_and_block(Duration::from_millis(10));
        if dev.is_ready() {
            break;
        }
    }

    println!("Device became ready!");
}