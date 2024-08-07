use std::{ffi::c_void, ptr};

use crate::{bindings::{mpr_obj_get_prop_as_obj, mpr_obj_get_type, mpr_obj_set_prop, mpr_prop, mpr_type}, device::{Device, MappableType}, signal::Signal, graph::Map};

pub trait AsMprObject {
  fn as_mpr_object(&self) -> *mut c_void;
}

impl AsMprObject for Signal {
  fn as_mpr_object(&self) -> *mut c_void {
    self.handle as *mut c_void
  }
}
impl AsMprObject for Device<'_> {
  fn as_mpr_object(&self) -> *mut c_void {
    self.handle as *mut c_void
  }
}

impl AsMprObject for Map {
  fn as_mpr_object(&self) -> *mut c_void {
    self.handle as *mut c_void
  }
}

pub trait MapperObject {
  /// Get the `mpr_type` representing this object
  fn get_type(&self) -> mpr_type;
  /// Set a property on this object to a numerical value
  fn set_property<T: MappableType>(&self, property: mpr_prop, value: T);
  /// Set a property on this object to a string value
  fn set_property_str(&self, property: mpr_prop, value: &str);

  /// Get the value of a property by it's key from this object.
  /// If the property does not exist, or if the type is not matched, this function will return `None`.
  fn get_property<T: MappableType + Default + Copy>(&self, property: mpr_prop) -> Option<T>;
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

  fn get_property<T: MappableType + Default + Copy>(&self, property: mpr_prop) -> Option<T> {
    unsafe {
      let result = mpr_obj_get_prop_as_obj(self.as_mpr_object(), property, ptr::null());
      if result.is_null() {
        None
      } else {
        Some(*(result as *const T))
      }
    }
  }
}