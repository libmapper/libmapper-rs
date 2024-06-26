
use std::ffi::c_void;

use crate::{bindings::{mpr_sig, mpr_sig_free, mpr_sig_get_inst_status, mpr_sig_get_value, mpr_sig_inst_status, mpr_sig_set_value, mpr_type}, device::MappableType};

pub struct Signal {
    pub(crate) handle: mpr_sig,
    pub(crate) owned: bool,
    pub(crate) data_type: mpr_type,
    pub(crate) vector_length: u32
}

unsafe impl Send for Signal {}
unsafe impl Sync for Signal {}

impl Drop for Signal {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                mpr_sig_free(self.handle);
            }
        }
    }
}

/// An error that can occur when getting or setting the value of a signal.
#[derive(Debug)]
pub enum SignalError {
    #[doc = "The data type of the signal does not match the type of the passed generic type."]
    WrongType,
    #[doc = "The signal does not have a value set yet."]
    NoValue,
    #[doc = "The length of the passed slice does not match the vector length of the signal."]
    WrongLengthArg
}

impl Signal {
    /// Get the status of the signal instance.
    /// Calling this function will reset the flags `was_set_remote` and `was_set_local` and return their pre-reset values.
    /// 
    /// # Examples
    /// Use this function to check if you should push or read data from the signal:
    /// ```
    /// use libmapper_rs::device::Device;
    /// use libmapper_rs::signal::Signal;
    /// fn main_loop(dev: &Device, sig: &mut Signal, value: &mut f64) {
    ///     loop {
    ///        dev.poll_and_block(10);
    /// 
    ///        if sig.get_status().was_set_remote() { // check if there's a new value waiting for us
    ///          let (new_value, _) = sig.get_value_single::<f64>().unwrap();
    ///          *value = new_value;
    ///        } else {
    ///          sig.set_value_single(value);
    ///        }
    ///     }
    /// }
    /// ```
    pub fn get_status(&self) -> SignalStatus {
        SignalStatus(unsafe {
            mpr_sig_get_inst_status(self.handle, 0)
        })
    }

    /// Get the type of data this signal is storing.
    pub fn get_type(&self) -> mpr_type {
        self.data_type
    }

    /// Get the length of the vector this signal is storing.
    /// This will be how long the slice returned from Signal::get_value is.
    /// 
    /// If this is 1, you should use Signal::get_value_single instead.
    pub fn get_vector_length(&self) -> u32 {
        self.vector_length
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
    /// Set the value of the signal.
    /// This function will return [SignalError::WrongType](SignalError:WrongType) if the passed generic type doesn't match the signal's type.
    /// 
    /// If this signal is a vector, only the first element of the vector will be set.
    pub fn set_value_single<T: MappableType + Copy>(&mut self, value: &T) -> Result<(), SignalError> {
        if T::get_mpr_type() != self.data_type {
            return Err(SignalError::WrongType);
        }
        unsafe {
            mpr_sig_set_value(self.handle, 0, 1,  self.data_type, value as *const T as *const c_void);
        }
        Ok(())
    }

    /// Get the value of the signal.
    /// This function will return [SignalError::WrongType](SignalError:WrongType) if the passed generic type doesn't match the signal's type.
    /// 
    /// If this signal is a vector, only the first element of the vector will be returned.
    pub fn get_value_single<T: MappableType + Copy>(&self) -> Result<(T, u64), SignalError> {
        let mut time = 0;
        if T::get_mpr_type() != self.data_type {
            return Err(SignalError::WrongType);
        }
        unsafe {
            let ptr = mpr_sig_get_value(self.handle, 0, &mut time);
            if ptr.is_null() {
                return Err(SignalError::NoValue);
            }
            let value = *(ptr as *const T);
            Ok((value, time))
        }
    }

    /// Get the value of the signal.
    /// This function will return [SignalError::WrongType](SignalError:WrongType) if the passed generic type doesn't match the signal's type.
    /// 
    /// The length of the returned slice will be equal to the value returned by [get_vector_length](Signal::get_vector_length).
    pub fn get_value<T: MappableType + Copy>(&self) -> Result<(Vec<T>, u64), SignalError> {
        let mut time = 0;
        if T::get_mpr_type() != self.data_type {
            return Err(SignalError::WrongType);
        }
        unsafe {
            let ptr = mpr_sig_get_value(self.handle, 0, &mut time);
            if ptr.is_null() {
                return Err(SignalError::NoValue);
            }
            let slice = std::slice::from_raw_parts(ptr as *const T, self.vector_length as usize);
            Ok((slice.to_vec(), time))
        }
    }

    /// Set the value of the signal.
    /// This function will return [SignalError::WrongType](SignalError:WrongType) if the passed generic type doesn't match the signal's type.
    /// 
    /// The length of the slice must be equal to the value returned by [get_vector_length](Signal::get_vector_length).
    /// If the lengths are not equal this function return an `Err` of `SignalError::WrongLengthArg`.
    pub fn set_value<T: MappableType + Copy>(&mut self, values: &[T]) -> Result<(), SignalError> {
        if T::get_mpr_type() != self.data_type {
            return Err(SignalError::WrongType);
        }
        if values.len() != self.vector_length as usize {
            return Err(SignalError::WrongLengthArg);
        }
        unsafe {
            mpr_sig_set_value(self.handle, 0, self.vector_length as i32, self.data_type, values.as_ptr() as *const c_void);
        }
        Ok(())
    }
}