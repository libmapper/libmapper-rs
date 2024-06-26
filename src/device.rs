use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;
use crate::bindings::{mpr_dev, mpr_dev_free, mpr_dev_get_is_ready, mpr_dev_new, mpr_dev_poll, mpr_dir, mpr_sig_new, mpr_type};
use crate::graph::Graph;
use crate::signal::Signal;

/// A device is libmapper's connection to the distributed graph.
/// Each device is a collection of signal instances and their metadata.
///
/// # Examples
/// ```
/// use libmapper_rs::device::Device;
/// // you can create a device with Device::create
/// let dev = Device::create("rust");
/// // you have to poll a device occasionally to make things happen
/// loop {
///     dev.poll_and_block(10); // poll in 10ms intervals
///     if dev.is_ready() {
///        break;
///     }
/// }
/// // create signals, etc...
/// ```
pub struct Device {
    handle: mpr_dev,
    owned: bool
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                mpr_dev_free(self.handle);
            }
        }
    }
}

impl Device {
    pub fn create(name: &str) -> Device {
        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Device {
                owned: true,
                handle: mpr_dev_new(name_ptr.as_ptr(), ptr::null_mut())
            }
        }
    }
    pub fn create_from_graph(name: &str, graph: &Graph) -> Device {
        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Device {
                owned: true,
                handle: mpr_dev_new(name_ptr.as_ptr(), graph.handle)
            }
        }
    }
}

impl Device {
    /// Poll the device without blocking
    ///
    /// # Notes
    /// You may want to use [poll_all](Device::poll-all) in a multithreaded enviroment,
    /// when using non-blocking polling libmapper will use a heuristic to determine how many messages
    /// to parse at once for performance. If you don't care how long this function will take to run,
    /// call Device::poll_all.
    pub fn poll(&self) {
        unsafe {
            mpr_dev_poll(self.handle, 0);
        }
    }
    /// Processes all messages in the device's queue, no matter how long it takes.
    /// In a multithreaded environment this is probably what you want to use instead of [poll](Device::poll)
    pub fn poll_all(&self) {
        unsafe {
            mpr_dev_poll(self.handle, -1);
        }
    }
    /// Blocks the current thread for `time` milliseconds.
    /// Use this function instead of sleeping in a loop.
    pub fn poll_and_block(&self, time: u32) {
        unsafe {
            mpr_dev_poll(self.handle, time as c_int);
        }
    }
}

impl Device {
    /// Tests if the device is ready to use.
    /// Do not try to call any other methods until this returns `true`.
    pub fn is_ready(&self) -> bool {
        unsafe {
            mpr_dev_get_is_ready(self.handle) > 0
        }
    }
}

pub trait MappableType {
    fn get_mpr_type() -> mpr_type;
}

impl MappableType for f64 {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_DBL
    }
}

impl MappableType for f32 {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_FLT
    }
}

impl MappableType for i32 {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_INT32
    }
}

impl Device {
    pub fn create_signal<T: MappableType + Copy>(&self, name: &str, direction: mpr_dir) -> Signal {
        self.create_vector_signal::<T>(name, direction, 1)
    }
    pub fn create_vector_signal<T: MappableType + Copy>(&self, name: &str, direction: mpr_dir, vector_length: u32) -> Signal {
        let data_type: mpr_type = T::get_mpr_type();

        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Signal {
                handle: mpr_sig_new(self.handle, direction, name_ptr.as_ptr(), vector_length as i32, data_type, ptr::null(), ptr::null(), ptr::null(), ptr::null_mut(), None, 0),
                data_type,
                owned: true,
                vector_length
            }
        }
    }
}