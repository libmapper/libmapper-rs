use libmapper_rs::device::Device;

fn main() {
    let dev = Device::create("rustmapper");
    loop {
        dev.poll_and_block(10);
        if dev.is_ready() {
            break;
        }
    }

    println!("Device became ready!");
    loop {
        dev.poll_and_block(100);
    }
}