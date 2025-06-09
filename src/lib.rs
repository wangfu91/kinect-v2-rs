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

    eprintln!("====== GetDefaultKinectSensor returned: {}", hr);
    if hr != 0 || kinect_sensor_ptr.is_null() {
        eprintln!(
            "=== Failed to get default Kinect sensor. HRESULT: {}. Pointer: {:?}",
            hr, kinect_sensor_ptr
        );
        return;
    }

    // IKinectSensor is a COM interface pointer. Methods are called via its vtable (lpVtbl).
    // kinect_sensor_ptr is *mut IKinectSensor.
    // The IKinectSensor struct contains `lpVtbl: *mut IKinectSensorVtbl`.
    // We need to access this vtable.
    let vtbl_ptr = unsafe { (*kinect_sensor_ptr).lpVtbl };

    if vtbl_ptr.is_null() {
        eprintln!("=== IKinectSensor's vtable (lpVtbl) is null.");
        // It's good practice to also call Release on kinect_sensor_ptr if it was acquired,
        // but GetDefaultKinectSensor might not require a matching Release if it's a singleton
        // or if the lifetime is managed differently. For now, just return.
        return;
    }

    // Dereference the vtable pointer to get the actual vtable struct.
    let vtbl = unsafe { &*vtbl_ptr };

    // The Open method is a field in the vtable, of type Option<unsafe extern "C" fn(...)>.
    if let Some(open_method) = vtbl.Open {
        // Call the Open method. The first argument to COM methods (This) is the interface pointer itself.
        let hr_open = unsafe { open_method(kinect_sensor_ptr) };
        if hr_open != 0 {
            eprintln!("=== Failed to open Kinect sensor. HRESULT: {}", hr_open);
            // Consider releasing kinect_sensor_ptr here if applicable
            return;
        }
        eprintln!("=== Kinect sensor opened successfully.");
        // TODO: Add code to use the sensor and then Close it.
        // For example, to close:
        // if let Some(close_method) = vtbl.Close {
        //     unsafe { close_method(kinect_sensor_ptr) };
        //     eprintln!("Kinect sensor closed.");
        // }
    } else {
        eprintln!("=== IKinectSensor_Open method not found in vtable (it's None).");
        // Consider releasing kinect_sensor_ptr here if applicable
        return;
    }

    // After use, the sensor should be closed and the interface pointer released.
    // This is a simplified example. Proper resource management (RAII) would be better.
    // For now, we are not closing or releasing in this example function.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_default_kinect_sensor();
    }
}
