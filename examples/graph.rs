use std::time::Duration;

use libmapper_rs::{constants::{mpr_prop, mpr_type}, graph::Graph, object::MapperObject};

/// Prints out a list of all device names found on the network

fn main() {
  let graph = Graph::create();
  let mut count = 0;

  // Have to subscribe to all devices first in order to discover devices we don't own!
  graph.subscribe(None, &[mpr_type::MPR_DEV]);

  loop {

    graph.poll_and_block(Duration::from_millis(10));
    let list = graph.get_devices();

    if list.len() != 0 {
      for dev in list {
        println!("Found device: {:?}", dev.get_property_str(mpr_prop::MPR_PROP_NAME).unwrap());
      }
      break;
    }
    
    println!("Loop {}", count);
    count += 1;
  }
  
}