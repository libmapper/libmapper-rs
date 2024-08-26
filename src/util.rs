use std::ffi::c_void;

use crate::bindings::{mpr_list_get_next, mpr_list_get_size};

/// Read a list of items from a libmapper list.
/// 
/// `constructor` is passed a pointer to the backing libmapper object and should return a new wrapped instance of the item.
pub fn read_list<T, J: Fn(*mut c_void) -> T>(list: *mut *mut c_void, constructor: J) -> Vec<T> {
  let mut ptr = list;
  let len = unsafe { mpr_list_get_size(ptr) };

  if len == 0 {
    return Vec::new();
  }

  let mut values = Vec::with_capacity(len as usize);

  for _ in 0..len {
    let val = constructor(unsafe {*ptr});
    values.push(val);

    ptr = unsafe {  mpr_list_get_next(ptr) };
  }

  values
}

pub fn mpr_type_from_i32(i: i32) -> crate::constants::mpr_type {
  unsafe {
    // TODO: VERY BAD
    std::mem::transmute(i)
  }
}