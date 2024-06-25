
use std::ffi::c_void;

use crate::{bindings::{mpr_sig, mpr_sig_get_inst_status, mpr_sig_get_value, mpr_sig_inst_status, mpr_sig_set_value, mpr_type}, device::MappableType};

pub struct Signal {
    pub(crate) handle: mpr_sig,
    pub(crate) owned: bool,
    pub(crate) data_type: mpr_type,
    pub(crate) vector_length: u32
}

impl Signal {
    pub fn get_status(&self) -> SignalStatus {
        SignalStatus(unsafe {
            mpr_sig_get_inst_status(self.handle, 0)
        })
    }
}

/// A struct that represents the status of a signal instance.
/// When this struct is created by Signal::get_status(), the flags `was_set_remote` and `was_set_local` will be reset.
pub struct SignalStatus(i32);
impl SignalStatus {
    /// Returns true if the signal was set remotely since the last time the status was queried.
    pub fn was_set_remote(&self) -> bool {
        self.0 & mpr_sig_inst_status::MPR_SIG_INST_SET_REMOTE as i32 != 0
    }
    /// Returns true if the signal was set locally since the last time the status was queried.
    pub fn was_set_local(&self) -> bool {
        self.0 & mpr_sig_inst_status::MPR_SIG_INST_SET_LOCAL as i32 != 0
    }
    /// Returns true if the signal has a value (i.e. Signal::get_value* will return Some).
    pub fn has_value(&self) -> bool {
        self.0 & mpr_sig_inst_status::MPR_SIG_INST_HAS_VALUE as i32 != 0
    }
    /// If the signal is active
    pub fn is_active(&self) -> bool {
        self.0 & mpr_sig_inst_status::MPR_SIG_INST_IS_ACTIVE as i32 != 0
    }
}

impl Signal {
    pub fn set_value_single<T: MappableType + Copy>(&mut self, value: &T) {
        if T::get_mpr_type() != self.data_type {
            panic!("Data type mismatch");
        }
        unsafe {
            mpr_sig_set_value(self.handle, 0, 1,  self.data_type, value as *const T as *const c_void);
        }
    }

    pub fn get_value_single<T: MappableType + Copy>(&self) -> Option<(T, u64)> {
        let mut time = 0;
        if T::get_mpr_type() != self.data_type {
            panic!("Data type mismatch");
        }
        unsafe {
            let ptr = mpr_sig_get_value(self.handle, 0, &mut time);
            if ptr.is_null() {
                return None;
            }
            let value = *(ptr as *const T);
            Some((value, time))
        }
    }

    pub fn get_value<T: MappableType + Copy>(&self) -> Option<(&[T], u64)> {
        let mut time = 0;
        if T::get_mpr_type() != self.data_type {
            panic!("Data type mismatch");
        }
        unsafe {
            let ptr = mpr_sig_get_value(self.handle, 0, &mut time);
            if ptr.is_null() {
                return None;
            }
            let slice = std::slice::from_raw_parts(ptr as *const T, self.vector_length as usize);
            Some((slice, time))
        }
    }

    pub fn set_value<T: MappableType + Copy>(&mut self, values: &[T]) {
        if T::get_mpr_type() != self.data_type {
            panic!("Data type mismatch");
        }
        if values.len() != self.vector_length as usize {
            panic!("Vector length mismatch");
        }
        unsafe {
            mpr_sig_set_value(self.handle, 0, self.vector_length as i32, self.data_type, values.as_ptr() as *const c_void);
        }
    }
}