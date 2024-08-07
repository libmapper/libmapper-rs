use std::{env, sync::{atomic::{AtomicU64, Ordering}, Arc}, thread::{self, JoinHandle}};

use libmapper_rs::graph::Graph;


fn main() {
  let graph = Arc::new(Graph::create());
  let counter: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

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

  let mut num_workers = 10;
  let arg = env::args().skip(1).next();
  if arg.is_some() {
    num_workers = arg.unwrap().parse::<i32>().unwrap_or(10);
  }

  // spawn n threads creating then deleting devices at random
  for _ in 0..num_workers {
    let graph = graph.clone();
    let id_counter = counter.clone();
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
        thread::sleep(std::time::Duration::from_millis(id_counter.fetch_add(1, Ordering::SeqCst) as u64));
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