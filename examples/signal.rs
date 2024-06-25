use std::time::{SystemTime, UNIX_EPOCH};

use libmapper_rs::{constants::mpr_dir, device::Device};

fn main() {
  let dev = Device::create("rustmapper");
  loop {
      dev.poll_and_block(10);
      if dev.is_ready() {
          break;
      }
  }

  println!("Device became ready!");
  let mut sig = dev.create_signal::<f64>("test_sin", mpr_dir::MPR_DIR_OUT);
  let debug_sig = dev.create_signal::<f64>("debug_msg", mpr_dir::MPR_DIR_IN);
  loop {
      dev.poll_and_block(100);
      let time = ((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64) as f64 / 1000f64).sin();
      sig.set_value_single(&time);
      let msgs = debug_sig.get_value_single::<f64>();
      if msgs.is_some() {
        println!("Debug message: {}", msgs.unwrap().0);
      }
  }
}