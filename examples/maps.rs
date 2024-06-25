use libmapper_rs::{device::Device, graph::Map};



pub fn main() {
    let dev = Device::create("rustmapper");
    loop {
        dev.poll_and_block(10);
        if dev.is_ready() {
            break;
        }
    }

    println!("Device became ready!");
    let mut sig_a = dev.create_signal::<i32>("output", libmapper_rs::constants::mpr_dir::MPR_DIR_OUT);
    let sig_b = dev.create_signal::<i32>("input", libmapper_rs::constants::mpr_dir::MPR_DIR_IN);
    let map = Map::create(&sig_a, &sig_b);
    map.push();
    loop {
        dev.poll_and_block(100);
        if map.is_ready() {
          break;
        }
    }
    println!("Map created!");
    for i in 0..100 {
      sig_a.set_value_single(&i);
      dev.poll_and_block(10);
      let val = sig_b.get_value_single::<i32>().expect("Signal didn't send!");
      println!("Sent: {}, Received: {}", i, val.0);
      assert_eq!(i, val.0)
    }
}