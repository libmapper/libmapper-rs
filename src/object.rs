use std::{ffi::c_void, ptr};

use crate::{bindings::{mpr_obj, mpr_obj_get_prop_by_idx, mpr_obj_get_type, mpr_obj_set_prop, mpr_prop, mpr_type}, device::{Device, MappableType}, graph::Map, signal::Signal};

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

impl AsMprObject for mpr_obj {
  fn as_mpr_object(&self) -> *mut c_void {
      self as *const mpr_obj as *mut c_void
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
  /// If the property does not exist, or if the type is not matched, this function will return an error.
  fn get_property<T: MappableType + Default + Copy>(&self, property: mpr_prop) -> Result<T, PropertyError>;

  /// Get the value of a string property by it's key from this object.
  /// If the property does not exist, or if the type is not matched, this function will return an error.
  fn get_property_str(&self, property: mpr_prop) -> Result<String, PropertyError>;
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

  fn get_property<T: MappableType + Default + Copy>(&self, property: mpr_prop) -> Result<T, PropertyError> {
    unsafe {
      let mut actual_type: mpr_type = mpr_type::MPR_NULL;
      let mut value: *const c_void  = ptr::null();
      mpr_obj_get_prop_by_idx(self.as_mpr_object(), property as i32,  ptr::null_mut(), ptr::null_mut(), 
      &mut actual_type, &mut value, ptr::null_mut());
      if value.is_null() {
        return Err(PropertyError::PropertyNotFound)
      }
      if actual_type != T::get_mpr_type() {
        return Err(PropertyError::TypeMismatch)
      }
      let value = value as *const T;
      Ok(*value)
    }
  }

  fn get_property_str(&self, property: mpr_prop) -> Result<String, PropertyError> {
    unsafe {
      let mut actual_type: mpr_type = mpr_type::MPR_NULL;
      let mut value: *const c_void  = ptr::null();
      mpr_obj_get_prop_by_idx(self.as_mpr_object(), property as i32,  ptr::null_mut(), ptr::null_mut(), 
      &mut actual_type, &mut value, ptr::null_mut());
      if value.is_null() {
        return Err(PropertyError::PropertyNotFound)
      }
      if actual_type != mpr_type::MPR_STR {
        return Err(PropertyError::TypeMismatch)
      }
      let value = value as *const std::os::raw::c_char;
      let value = std::ffi::CStr::from_ptr(value).to_str().unwrap().to_string();
      Ok(value)
    }
  }
}

/// Errors that can occur when working with properties
#[derive(Debug, PartialEq)]
pub enum PropertyError {
  /// The property was not found on the object
  PropertyNotFound,
  /// The property was found, but the type did not match the expected type
  TypeMismatch
}