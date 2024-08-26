use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;
use std::time::Duration;
use crate::bindings::{mpr_dev, mpr_dev_free, mpr_dev_get_is_ready, mpr_dev_get_sigs, mpr_dev_new, mpr_dev_poll, mpr_dir, mpr_obj, mpr_prop, mpr_sig_new, mpr_type};
use crate::graph::Graph;
use crate::object::MapperObject;
use crate::signal::Signal;

/// A device is libmapper's connection to the distributed graph.
/// Each device is a collection of signal instances and their metadata.
///
/// # Examples
/// ```
/// use libmapper_rs::device::Device;
/// use std::time::Duration;
/// // you can create a device with Device::create
/// let dev = Device::create("rust");
/// // you have to poll a device occasionally to make things happen
/// loop {
///     dev.poll_and_block(Duration::from_millis(10)); // poll in 10ms intervals
///     if dev.is_ready() {
///        break;
///     }
/// }
/// // create signals, etc...
/// ```
pub struct Device<'a> {
    pub(crate) handle: mpr_dev,
    pub(crate) owned: bool,
    pub(crate) graph: Option<&'a Graph>
}

unsafe impl Send for Device<'_> {}
unsafe impl Sync for Device<'_> {}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                mpr_dev_free(self.handle);
            }
        }
    }
}

impl Device<'_> {
    /// Create a new device with the given name.
    /// The device will use it's own connection to the graph.
    ///
    /// Before calling any other methods on the device, you should poll it until it is ready.
    /// 
    /// # Notes
    /// If you plan on creating multiple devices, consider using (Device::create_from_graph)[Device::create_from_graph] instead to pool resources.
    pub fn create(name: &str) -> Device {
        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Device {
                owned: true,
                handle: mpr_dev_new(name_ptr.as_ptr(), ptr::null_mut()),
                graph: None
            }
        }
    }
    /// Create a new device with a shared graph.
    /// Sharing a graph between devices allows them to pool some resources and networking, potentially improving performance.
    pub fn create_from_graph<'a>(name: &str, graph: &'a Graph) -> Device<'a> {
        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Device {
                owned: true,
                handle: mpr_dev_new(name_ptr.as_ptr(), graph.handle),
                graph: Some(graph)
            }
        }
    }
}

impl Device<'_> {
    /// Poll the device without blocking
    ///
    /// # Notes
    /// You may want to use [poll_all](Device::poll_all) in a multithreaded enviroment,
    /// when using non-blocking polling libmapper will use a heuristic to determine how many messages
    /// to parse at once for performance. If you don't care how long this function will take to run,
    /// call Device::poll_all.
    pub fn poll(&self) {
        unsafe {
            mpr_dev_poll(self.handle, 0);
        }
    }
    /// Processes all messages in the device's queue, no matter how long it takes.
    /// If using dedicated threads to poll devices this is probably what you want to use instead of [poll](Device::poll)
    pub fn poll_all(&self) {
        unsafe {
            mpr_dev_poll(self.handle, -1);
        }
    }
    /// Blocks the current thread for a specified amount of time.
    /// Use this function instead of sleeping in a loop.
    pub fn poll_and_block(&self, time: Duration) {
        unsafe {
            mpr_dev_poll(self.handle, time.as_millis() as c_int);
        }
    }
}

impl Device<'_> {
    /// Tests if the device is ready to use.
    /// Do not try to call any other methods until this returns `true`.
    pub fn is_ready(&self) -> bool {
        unsafe {
            mpr_dev_get_is_ready(self.handle) > 0
        }
    }
}

/// Marker trait for types that are bit-compatible with the libmapper C library.
/// If this trait is implemented on a type, that type can be passed to libmapper functions safely.
/// Use the `get_mpr_type` function to pass a type parameter to libmapper.
pub trait MappableType {
    /// Get the `mpr_type` representing this rust type.
    fn get_mpr_type() -> mpr_type;
}

impl MappableType for f64 {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_DBL
    }
}

impl MappableType for mpr_type {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_TYPE
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

impl MappableType for i64 {
    fn get_mpr_type() -> mpr_type {
        mpr_type::MPR_INT64
    }
}

impl<'a> Device<'a> {
    /// Get the shared graph used by this device.
    /// If the device was created with [Device::create](Device::create) this will return None.
    pub fn get_graph(&self) -> Option<&'a Graph> {
        self.graph
    }
}

impl Device<'_> {
    /// Check if the device was created with a shared graph.
    pub fn has_shared_graph(&self) -> bool {
        self.graph.is_some()
    }
    /// Create a signal with the given name and direction.
    /// 
    /// # Notes
    /// - The signal will have a vector length of 1 (i.e. single value).
    /// - The passed generic parameter controls what type of data the signal will hold.
    /// 
    /// # Examples
    /// ```
    /// use libmapper_rs::device::Device;
    /// use libmapper_rs::constants::mpr_dir;
    /// fn setup_signals(dev: &Device) {
    ///     // create an outgoing signal that outputs a single f64 value
    ///     let sig = dev.create_signal::<f64>("test_signal", mpr_dir::MPR_DIR_OUT);
    /// }
    /// ```
    pub fn create_signal<T: MappableType + Copy>(&self, name: &str, direction: mpr_dir) -> Signal {
        self.create_vector_signal::<T>(name, direction, 1)
    }
    /// Create a signal with the given name, direction, and vector length.
    /// 
    /// # Notes
    /// - The passed generic parameter controls what type of data the signal will hold.
    /// 
    pub fn create_vector_signal<T: MappableType + Copy>(&self, name: &str, direction: mpr_dir, vector_length: u32) -> Signal {
        let data_type: mpr_type = T::get_mpr_type();

        let name_ptr = CString::new(name).expect("CString::new failed");
        unsafe {
            Signal {
                handle: mpr_sig_new(self.handle, direction, name_ptr.as_ptr(), vector_length as i32, 
                    data_type, ptr::null(), ptr::null(), ptr::null(), ptr::null_mut(), None, 0),
                data_type,
                owned: true,
                vector_length
            }
        }
    }
    /// Get a list of all signals owned by this device.
    pub fn get_signals(&self, direction: mpr_dir) -> Vec<Signal> {
        let list = unsafe {mpr_dev_get_sigs(self.handle, direction)};
        crate::util::read_list(list, |ptr| {
            let data_type = (ptr as mpr_obj).get_property::<mpr_type>(mpr_prop::MPR_PROP_TYPE).unwrap();
            let vector_length = (ptr as mpr_obj).get_property::<i32>(mpr_prop::MPR_PROP_LEN).unwrap() as u32;
            Signal {
                handle: ptr,
                data_type,
                owned: false,
                vector_length 
            }
        })
    }
}