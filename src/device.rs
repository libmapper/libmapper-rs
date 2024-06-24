use std::ffi::{CString};
use std::os::raw::c_int;
use crate::bindings::{mpr_dev, mpr_dev_get_is_ready, mpr_dev_new, mpr_dev_poll};

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
/// }
/// ```
pub struct Device {
    handle: mpr_dev,
    owned: bool
}

impl Device {
    pub fn create(name: &str) -> Device {
        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Device {
                owned: true,
                handle: mpr_dev_new(name_ptr.as_ptr(), None)
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
    pub fn is_ready(&self) -> bool {
        unsafe {
            mpr_dev_get_is_ready(self.handle) > 0
        }
    }
}