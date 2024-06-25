
use std::ffi::c_void;

use crate::{bindings::{mpr_sig, mpr_sig_get_value, mpr_sig_set_value, mpr_type}, device::MappableType};

pub struct Signal {
    pub(crate) handle: mpr_sig,
    pub(crate) owned: bool,
    pub(crate) data_type: mpr_type,
    pub(crate) vector_length: u32
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
        unsafe {
            let ptr = mpr_sig_get_value(self.handle, 0, &mut time);
            if ptr.is_null() {
                return None;
            }
            let value = *(ptr as *const T);
            Some((value, time))
        }
    }
}