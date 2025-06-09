mod bindings;

use std::ptr;

use crate::bindings::{GetDefaultKinectSensor, IKinectSensor};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn get_default_kinect_sensor() {
    let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();

    let hr = unsafe {
        GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor)
    };

    println!("GetDefaultKinectSensor returned: {}", hr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
