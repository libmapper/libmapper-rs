#![feature(const_trait_impl,effects)]
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