pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(trivial_casts)]
    #![allow(clippy::all)]
    #![allow(unsafe_op_in_unsafe_fn)]
    #![allow(unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use windows_sys::{
    Win32::Foundation::{BOOLEAN, E_FAIL, E_POINTER},
    core::HRESULT,
};

use crate::bindings::{GetDefaultKinectSensor, IKinectSensor};
use std::ptr;

// Add an impl block for IKinectSensor to provide a helper method for Open
impl IKinectSensor {
    pub fn open(&mut self) -> Result<(), HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(open_fn) = vtbl.Open {
                let hr = unsafe { open_fn(kinect_sensor_ptr) };
                if hr == 0 {
                    Ok(())
                } else {
                    eprintln!(
                        "IKinectSensor::open: Failed to open Kinect sensor. HRESULT: {:#X}",
                        hr
                    );
                    Err(hr)
                }
            } else {
                eprintln!("IKinectSensor::open: Open method not found in VTable (is None).");
                Err(E_FAIL)
            }
        } else {
            eprintln!("IKinectSensor::open: VTable pointer (lpVtbl) is null.");
            Err(E_POINTER)
        }
    }

    pub fn is_available(&mut self) -> Result<bool, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_is_available_fn) = vtbl.get_IsAvailable {
                let mut is_available: BOOLEAN = 0;
                let hr = unsafe { get_is_available_fn(kinect_sensor_ptr, &mut is_available) };
                if hr == 0 {
                    Ok(is_available != 0)
                } else {
                    Err(hr)
                }
            } else {
                eprintln!("get_IsAvailable method not found in IKinectSensor VTable (is None).");
                Err(E_FAIL)
            }
        } else {
            eprintln!("IKinectSensor::get_IsAvailable: VTable pointer (lpVtbl) is null.");
            Err(E_POINTER)
        }
    }
}

pub fn get_default_kinect_sensor() {
    let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();

    // Call the FFI function to get the default Kinect sensor
    let hr = unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor) };

    eprintln!("====== GetDefaultKinectSensor returned: HRESULT {:#X}", hr);
    if hr != 0 {
        eprintln!(
            "=== Failed to get default Kinect sensor. HRESULT: {:#X}",
            hr
        );
        return;
    }

    // At this point, kinect_sensor_ptr is considered valid and points to an IKinectSensor instance.
    // We need to convert `*mut IKinectSensor` to `&mut IKinectSensor` to call the method.
    // This is an unsafe operation as it involves dereferencing a raw pointer.
    // It assumes kinect_sensor_ptr is valid, non-null, and properly aligned.
    let sensor_instance: &mut IKinectSensor = unsafe { &mut *kinect_sensor_ptr };
    if let Err(hr) = sensor_instance.open() {
        eprintln!("=== Failed to open Kinect sensor. HRESULT: {:#X}", hr);
        return;
    } else {
        eprintln!("=== Successfully opened Kinect sensor.");
    }

    match sensor_instance.is_available() {
        Err(hr) => {
            eprintln!("=== Kinect sensor is not available. HRESULT: {:#X}", hr);
        }
        Ok(is_available) => {
            eprintln!("=== Kinect sensor available: {:?}.", is_available);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_default_kinect_sensor();
    }
}
