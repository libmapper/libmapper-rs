//! # libmapper-rs
//! 
//! libmapper is a cross-platform library for connecting data sources to synthesizers, DAWs, and other hardware or software devices.
//! It provides a simple API for creating and managing signals, devices, and mappings between them.
//! 
//! This project contains safe, idiomatic rust bindings to the libmapper C api.
//! 
//! ## Concepts
//! ### [Devices](device)
//! Libmapper operates on a shared peer-to-peer graph. A [Device](device::Device) represents a connection to this graph and is a container for signals.
//! 
//! Most libmapper code will start by creating a device and polling until it becomes ready, like so:
//! ```
//! use libmapper_rs::device::Device;
//! fn main() {
//!     let mut device = Device::new("CoolDevice").unwrap();
//!     loop {
//!       device.poll_and_block(10);
//!       if device.is_ready() {
//!          break;
//!         }
//!     }
//!     println!("Device is ready!");
//!     // create signals, maps, etc.
//! }
//! ```

use bindings::mpr_get_version;

pub mod device;
pub mod graph;
pub mod signal;
pub mod object;

pub mod constants {
    pub use crate::bindings::mpr_dir;
    pub use crate::bindings::mpr_type;
    pub use crate::bindings::mpr_prop;
}
mod bindings;

/// Get the version of the loaded libmapper library.
pub fn get_mapper_version() -> &'static str {
    unsafe {
        let version = mpr_get_version();
        std::ffi::CStr::from_ptr::<'static>(version).to_str().unwrap()
    }
}