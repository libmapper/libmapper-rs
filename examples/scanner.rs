//! # Example: Scan
//! Will print a tree of all devices and signals found on the network.

use std::time::Duration;

use libmapper_rs::{constants::{mpr_prop, mpr_type}, graph::Graph, object::MapperObject};

/// Prints out a list of all device names found on the network

fn main() {
  let graph = Graph::create();
  let mut count = 0;

  // Have to subscribe to all devices first in order to discover devices we don't own!
  graph.subscribe(None, &[mpr_type::MPR_DEV, mpr_type::MPR_SIG]);

  loop {

    graph.poll_and_block(Duration::from_millis(10));
    let list = graph.get_devices();

    if list.len() != 0 {
      for dev in list {
        println!("Device: {}", dev.get_property_str(mpr_prop::MPR_PROP_NAME).unwrap());
        let signals = dev.get_signals(libmapper_rs::constants::mpr_dir::MPR_DIR_ANY);
        for sig in signals {
          println!("\tSignal: {}", sig.get_property_str(mpr_prop::MPR_PROP_NAME).unwrap());
          println!("\t\tType: {:?}", sig.get_property::<mpr_type>(mpr_prop::MPR_PROP_TYPE).unwrap());
          println!("\t\tVector Length: {:?}", sig.get_vector_length());
          println!("\t\tMin: {:?}", sig.get_property::<f32>(mpr_prop::MPR_PROP_MIN));
          println!("\t\tMax: {:?}", sig.get_property::<f32>(mpr_prop::MPR_PROP_MAX));
          println!("\t\tUnit: {:?}", sig.get_property_str(mpr_prop::MPR_PROP_UNIT));
          println!("\t\tNum instances: {:?}", sig.get_property::<i32>(mpr_prop::MPR_PROP_NUM_INST).unwrap());
          println!("");
        }
      }
      break;
    }
    
    println!("Loop {}", count);
    count += 1;
  }
  
}