use libmapper_rs::{constants::mpr_prop, device::Device, object::MapperObject};


fn main() {
  let dev = Device::create("CoolDev");
  loop {
    dev.poll_and_block(10);
    if dev.is_ready() {
      println!("Device is ready!");
      break;
    }
  }
  let p = dev.get_property_str(mpr_prop::MPR_PROP_NAME).unwrap();
  println!("Device Name: {}", p);
}