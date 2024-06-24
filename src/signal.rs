
use std::ffi::c_void;

use crate::{bindings::{mpr_sig, mpr_sig_get_value, mpr_sig_set_value, mpr_type}, device::Device};

pub struct Signal<T: Sized + Copy, const COUNT: i32> {
    pub(crate) handle: mpr_sig,
    pub(crate) data_type: mpr_type,
    pub(crate) phantom: std::marker::PhantomData<T>
}
impl<T: Copy, const COUNT: i32> Signal<T, COUNT> {

    pub fn get_value(&self, index: u64) -> (&[T], u64) {
        let mut time: u64 = 0;
        unsafe {
            let value = mpr_sig_get_value(self.handle, index, &mut time);
            let value = std::slice::from_raw_parts(value as *const T, COUNT as usize);
            (value, time)
        }
    }
    pub fn set_value(&self, value: &[T]) {
        unsafe {
            mpr_sig_set_value(self.handle, 0, COUNT, self.data_type, value.as_ptr() as *const c_void);
        }
    }
}

impl<T: Copy> Signal<T, 1> {
  pub fn get_value_single(&self, index: u64) -> Option<(T, u64)> {
    let mut time: u64 = 0;
    unsafe {
        let value = mpr_sig_get_value(self.handle, index, &mut time);
        if value.is_null() {
            return None;
        }
        Some((*(value as *const T), time))
    }
  }
  pub fn set_value_single(&self, value: &T) {
      unsafe {
          mpr_sig_set_value(self.handle, 0, 1, self.data_type, value as *const T as *const c_void);
      }
  }
} 