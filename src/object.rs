use std::{ffi::c_void, ptr};

use crate::{bindings::{mpr_obj_get_type, mpr_obj_set_prop, mpr_prop, mpr_type}, device::{Device, MappableType}, signal::Signal};

pub trait AsMprObject {
  fn as_mpr_object(&self) -> *mut c_void;
}

impl AsMprObject for Signal {
  fn as_mpr_object(&self) -> *mut c_void {
    self.handle as *mut c_void
  }
}
impl AsMprObject for Device {
  fn as_mpr_object(&self) -> *mut c_void {
    self.handle as *mut c_void
  }
}

pub trait MapperObject {
  fn get_type(&self) -> mpr_type;
  fn set_property<T: MappableType>(&self, property: mpr_prop, value: T);
  fn set_property_str(&self, property: mpr_prop, value: &str);
}

impl<A> MapperObject for A where A: AsMprObject {
  fn get_type(&self) -> mpr_type {
    unsafe {
      mpr_obj_get_type(self.as_mpr_object())
    }
  }
  fn set_property<T: MappableType>(&self, property: mpr_prop, value: T) {
    unsafe {
      mpr_obj_set_prop(self.as_mpr_object(), property, ptr::null(), 1, T::get_mpr_type(), &value as *const T as *const c_void, 1);
    }
  }
  fn set_property_str(&self, property: mpr_prop, value: &str) {
    let value_ptr = std::ffi::CString::new(value).expect("CString::new failed");
    unsafe {
      mpr_obj_set_prop(self.as_mpr_object(), property, ptr::null(), 1, mpr_type::MPR_STR, value_ptr.as_ptr() as *const c_void, 1);
    }
  }
}