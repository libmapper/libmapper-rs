//! Graph and Map types for working with libmapper's distributed graph.
//! 
//! The [Map] type is used to create a connection between two [Signal] instances.
//! 
//! The [Graph] type can be shared between devices to improve performance and memory usage.
use crate::{bindings::*, signal::Signal};

/// A graph is a lightweight connection to libmapper's distributed graph.
/// You can use a graph to create maps and query the state of the graph.
pub struct Graph {
    pub(crate) handle: mpr_graph,
    owned: bool
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}

impl Graph {
  pub fn create() -> Graph {
    Graph {
      owned: true,
      handle: unsafe { mpr_graph_new(0) }
    }
  }
}

impl Drop for Graph {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                mpr_graph_free(self.handle);
            }
        }
    }
}

impl Graph {
  /// Poll the graph without blocking
  pub fn poll(&self) {
    unsafe {
      mpr_graph_poll(self.handle, 0);
    }
  }

  pub fn poll_and_block(&self, time: i32) {
    unsafe {
      mpr_graph_poll(self.handle, time);
    }
  }
}

/// A directional between multiple signals. Changes to input signals will affect output signals.
/// 
/// # Examples
/// Create a map between two signals:
/// ```
/// use std::thread;
/// use std::time::Duration;
/// use libmapper_rs::graph::Map;
/// use libmapper_rs::signal::Signal;
/// fn create_map(sig_a: &Signal, sig_b: &Signal) -> Map {
///   let map = Map::create(sig_a, sig_b);
///   loop {
///     if map.is_ready() {
///      break;
///     }
///    thread::sleep(Duration::from_millis(10));
///   }
///   map
/// }
/// ```
pub struct Map {
  pub(crate) handle: mpr_map
}

impl Map {
  /// Create a new map between two signals.
  /// This does not actually create the map in the graph, [push](Map::push) must be called to let the rest of the graph know about the map.
  pub fn create(src: &Signal, dst: &Signal) -> Map {
    Map {
      handle: unsafe { mpr_map_new(1, &src.handle, 1, &dst.handle) }
    }
  }

  /// Publish this map to the distributed graph.
  /// After calling this function and once [is_ready](Map::is_ready) returns `true`, the map is active.
  pub fn push(&self) {
    unsafe {
      mpr_obj_push(self.handle);
    }
  }

  /// Returns `true` once the map has been published and is active.
  /// Otherwise, returns false.
  pub fn is_ready(&self) -> bool {
    unsafe {
      mpr_map_get_is_ready(self.handle) != 0
    }
  }

  /// Destroy the map, severing the connection between the signals.
  pub fn release(self) {
    unsafe {
      mpr_map_release(self.handle)
    }
  }
}