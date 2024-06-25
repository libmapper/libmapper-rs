
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