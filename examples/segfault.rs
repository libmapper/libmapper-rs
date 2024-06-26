use std::{sync::Arc, thread::{self, JoinHandle}};

use libmapper_rs::graph::Graph;


fn main() {
  let graph = Arc::new(Graph::create());

  let mut threads = Vec::<JoinHandle<()>>::new();

  {
    // spawn a thread to poll the graph
    let graph = Arc::clone(&graph);
    let thread = std::thread::spawn(move || {
      loop {
        graph.poll();
      }
    });
    threads.push(thread);
  }

  // spawn 10 threads creating then deleting devices at random
  for _ in 0..10 {
    let graph = graph.clone();
    let thread = std::thread::spawn(move || {
      let name = format!("rust_{:?}", thread::current().id());
      loop {
        let _dev = libmapper_rs::device::Device::create_from_graph(&name, &graph);
        loop {
          _dev.poll();
          if _dev.is_ready() {
            break;
          }
        }
        let signal = _dev.create_signal::<f32>("test_sig", libmapper_rs::constants::mpr_dir::MPR_DIR_OUT);
        thread::sleep(std::time::Duration::from_millis(rand::random::<u64>() % 10));
        drop(signal);
        drop(_dev);
      }
    });
    threads.push(thread);
  }

  for thread in threads {
    thread.join().unwrap();
  }
}