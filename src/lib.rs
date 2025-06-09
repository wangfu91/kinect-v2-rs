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

use crate::bindings::{GetDefaultKinectSensor, IKinectSensor};
use std::ptr;

pub fn get_default_kinect_sensor() {
    let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();

    let hr = unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor) };

    println!("GetDefaultKinectSensor returned: {}", hr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_default_kinect_sensor();
    }
}
