use crate::{bindings::*, signal::Signal};

/// A graph is a lightweight connection to libmapper's distributed graph.
/// You can use a graph to create maps and query the state of the graph.
pub struct Graph {
    pub(crate) handle: mpr_graph,
    owned: bool
}

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

pub struct Map {
  pub(crate) handle: mpr_map
}

impl Map {
  pub fn create(src: &Signal, dst: &Signal) -> Map {
    Map {
      handle: unsafe { mpr_map_new(1, &src.handle, 1, &dst.handle) }
    }
  }

  pub fn push(&self) {
    unsafe {
      mpr_obj_push(self.handle);
    }
  }

  pub fn is_ready(&self) -> bool {
    unsafe {
      mpr_map_get_is_ready(self.handle) != 0
    }
  }
}