/// Make a lightshow in webmapper!
/// 
/// This example demonstrates how to create a device and set a custom property on it.

use std::time::Duration;

use libmapper_rs::{device::Device, object::MapperObject};

fn main() {
  let dev = Device::create("discoball");
  loop {
      dev.poll_and_block(Duration::from_millis(10));
      if dev.is_ready() {
          break;
      }
  }

  println!("Device became ready!");
  let mut hue = 0;
  loop {
      dev.poll_and_block(Duration::from_millis(5));
      dev.set_custom_property("color.hue", hue as f64 / 360.0, true);
      hue = (hue + 1) % 360;
  }
}