use std::time::Duration;

use libmapper_rs::{constants::mpr_prop, device::Device, object::MapperObject};


fn main() {
  let dev = Device::create("CoolDev");
  loop {
    dev.poll_and_block(Duration::from_millis(10));
    if dev.is_ready() {
      println!("Device is ready!");
      break;
    }
  }
  let p = dev.get_property::<i64>(mpr_prop::MPR_PROP_ID).unwrap();
  println!("Device ID: {}", p);
}