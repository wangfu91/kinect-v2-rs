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

pub mod color_frame;
pub mod kinect_sensor;

use crate::bindings::{GetDefaultKinectSensor, IKinectSensor};
use std::ptr;

pub fn demo() {
    let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();

    // Call the FFI function to get the default Kinect sensor
    let hr = unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor) };

    eprintln!("GetDefaultKinectSensor returned: HRESULT {:#X}", hr);
    if hr != 0 {
        eprintln!("Failed to get default Kinect sensor. HRESULT: {:#X}", hr);
        return;
    }

    // At this point, kinect_sensor_ptr is considered valid and points to an IKinectSensor instance.
    // We need to convert `*mut IKinectSensor` to `&mut IKinectSensor` to call the method.
    // This is an unsafe operation as it involves dereferencing a raw pointer.
    // It assumes kinect_sensor_ptr is valid, non-null, and properly aligned.
    let sensor_instance: &mut IKinectSensor = unsafe { &mut *kinect_sensor_ptr };
    if let Err(hr) = sensor_instance.open() {
        eprintln!("Failed to open Kinect sensor. HRESULT: {:#X}", hr);
        return;
    } else {
        eprintln!("Successfully opened Kinect sensor.");
    }

    match sensor_instance.is_available() {
        Err(hr) => {
            eprintln!("Kinect sensor is not available. HRESULT: {:#X}", hr);
        }
        Ok(is_available) => {
            eprintln!("Kinect sensor available: {:?}.", is_available);
        }
    }

    match sensor_instance.unique_kinect_id() {
        Err(hr) => {
            eprintln!("Failed to get unique Kinect ID. HRESULT: {:#X}", hr);
        }
        Ok(unique_id) => {
            eprintln!("Unique Kinect ID: {}", unique_id);
        }
    }

    match sensor_instance.is_open() {
        Err(hr) => {
            eprintln!(
                "Failed to check if Kinect sensor is open. HRESULT: {:#X}",
                hr
            );
        }
        Ok(is_open) => {
            eprintln!("Kinect sensor is open: {:?}", is_open);
        }
    }

    match sensor_instance.kinect_capabilities() {
        Err(hr) => {
            eprintln!("Failed to get Kinect capabilities. HRESULT: {:#X}", hr);
        }
        Ok(capabilities) => {
            eprintln!("Kinect capabilities: {:#X}", capabilities);
        }
    }

    match sensor_instance.audio_source() {
        Err(hr) => {
            eprintln!("Failed to get audio source. HRESULT: {:#X}", hr);
        }
        Ok(audio_source) => {
            eprintln!("Audio source obtained successfully: {:?}", audio_source);
        }
    }

    match sensor_instance.color_frame_source() {
        Err(hr) => {
            eprintln!("Failed to get color frame source. HRESULT: {:#X}", hr);
        }
        Ok(color_frame_source) => {
            eprintln!(
                "Color frame source obtained successfully: {:?}",
                color_frame_source
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        demo();
    }
}
