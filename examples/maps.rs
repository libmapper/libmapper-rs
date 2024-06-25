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
    let mut sigA = dev.create_signal::<i32>("output", libmapper_rs::constants::mpr_dir::MPR_DIR_OUT);
    let sigB = dev.create_signal::<i32>("input", libmapper_rs::constants::mpr_dir::MPR_DIR_IN);
    let map = Map::create(&sigA, &sigB);
    map.push();
    loop {
        dev.poll_and_block(100);
        if map.is_ready() {
          break;
        }
    }
    println!("Map created!");
    for i in 0..100 {
      sigA.set_value_single(&i);
      dev.poll_and_block(10);
      let val = sigB.get_value_single::<i32>().expect("Signal didn't send!");
      assert_eq!(i, val.0)
    }
}