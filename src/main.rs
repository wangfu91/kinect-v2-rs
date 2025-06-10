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

use crate::bindings::{
    GetDefaultKinectSensor, IKinectSensor, WAIT_TIMEOUT, WAITABLE_HANDLE, WaitForSingleObject,
};
use std::ptr;
use windows_sys::{
    Win32::{
        Foundation::{BOOLEAN, E_FAIL, HANDLE, WAIT_OBJECT_0},
        System::Com::{COINIT_MULTITHREADED, CoInitializeEx, Urlmon::E_PENDING},
    },
    core::HRESULT,
};

pub fn main() {
    if let Err(e) = color_frame_demo() {
        eprintln!("Error in color frame demo: {:?}", e);
    }
}

fn color_frame_demo() -> anyhow::Result<()> {
    // Initialize COM for this new thread as MTA.
    let com_init_result = unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED as u32) };
    if com_init_result != 0 {
        eprintln!(
            "1. Failed to initialize COM MTA for frame event thread: {:?}",
            com_init_result
        );
        return Err(anyhow::anyhow!("Failed to initialize COM MTA"));
    }

    let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();
    let hr = unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor) };
    assert_eq!(
        hr, 0,
        "Failed to get default Kinect sensor. HRESULT: {:#X}",
        hr
    );

    let sensor_instance: &mut IKinectSensor = unsafe { &mut *kinect_sensor_ptr };
    sensor_instance
        .open()
        .expect("Failed to open Kinect sensor");

    let color_frame_source_ptr = sensor_instance
        .color_frame_source()
        .expect("Failed to get color frame source");

    let color_frame_source = unsafe { &mut *color_frame_source_ptr };

    let color_frame_reader_ptr = color_frame_source
        .open_reader()
        .expect("Failed to open color frame reader");

    let color_frame_reader = unsafe { &mut *color_frame_reader_ptr };

    let mut waitable_handle: WAITABLE_HANDLE = WAITABLE_HANDLE::default();

    color_frame_reader
        .subscribe_frame_arrived(&mut waitable_handle)
        .expect("Failed to subscribe to frame arrived event");

    loop {
        let result = unsafe { WaitForSingleObject(waitable_handle as HANDLE, 2) };
        if WAIT_OBJECT_0 == result {
            match color_frame_reader.get_frame_arrived_event_data(waitable_handle) {
                Ok(event_args_ptr) => {
                    let event_args = unsafe { &mut *event_args_ptr };
                    if let Ok(frame_reference_ptr) = event_args.get_frame_reference() {
                        let frame_reference = unsafe { &mut *frame_reference_ptr };
                        if let Ok(color_frame_ptr) = frame_reference.acquire_frame() {
                            let color_frame = unsafe { &mut *color_frame_ptr };
                            let frame_description_ptr = color_frame
                                .get_frame_description()
                                .expect("Failed to get raw color image");

                            let frame_description = unsafe { &mut *frame_description_ptr };
                            let width = frame_description.get_width().expect("Failed to get width");
                            let height = frame_description
                                .get_height()
                                .expect("Failed to get height");

                            eprintln!("====== Color frame dimensions: {}x{}", width, height);
                        } else {
                            eprintln!("Failed to acquire color frame.");
                        }
                    } else {
                        eprintln!("Failed to get frame reference.");
                    }
                }
                Err(hr) => {
                    eprintln!("Failed to get frame arrived event data. HRESULT: {:#X}", hr);
                    break;
                }
            }
        } else if WAIT_TIMEOUT == result {
            // No new frame available, continue waiting
            eprintln!("No new color frame available, waiting...");
            std::thread::sleep(std::time::Duration::from_millis(10));
        } else {
            eprintln!("WaitForSingleObject failed with result: {:#X}", result);
            break;
        }
    }

    /*
    let worker_handle = std::thread::spawn(move || {
        // Initialize COM for this new thread as MTA.
        let com_init_result =
            unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED as u32) };
        if com_init_result != 0 {
            eprintln!(
                "2. Failed to initialize COM MTA for frame event thread: {:?}",
                com_init_result
            );
            return;
        }

        loop {
            let result = unsafe { WaitForSingleObject(waitable_handle as HANDLE, 2) };
            if WAIT_OBJECT_0 == result {
                match color_frame_reader.get_frame_arrived_event_data(waitable_handle) {
                    Ok(event_args_ptr) => {
                        let event_args = unsafe { &mut *event_args_ptr };
                        if let Ok(frame_reference_ptr) = event_args.get_frame_reference() {
                            let frame_reference = unsafe { &mut *frame_reference_ptr };
                            if let Ok(color_frame_ptr) = frame_reference.acquire_frame() {
                                let color_frame = unsafe { &mut *color_frame_ptr };
                                let frame_description_ptr = color_frame
                                    .get_frame_description()
                                    .expect("Failed to get raw color image");

                                let frame_description = unsafe { &mut *frame_description_ptr };
                                let width =
                                    frame_description.get_width().expect("Failed to get width");
                                let height = frame_description
                                    .get_height()
                                    .expect("Failed to get height");

                                eprintln!("====== Color frame dimensions: {}x{}", width, height);
                            } else {
                                eprintln!("Failed to acquire color frame.");
                            }
                        } else {
                            eprintln!("Failed to get frame reference.");
                        }
                    }
                    Err(hr) => {
                        eprintln!("Failed to get frame arrived event data. HRESULT: {:#X}", hr);
                        break;
                    }
                }
            } else if WAIT_TIMEOUT == result {
                // No new frame available, continue waiting
                eprintln!("No new color frame available, waiting...");
                std::thread::sleep(std::time::Duration::from_millis(10));
            } else {
                eprintln!("WaitForSingleObject failed with result: {:#X}", result);
                break;
            }
        }
    });

    worker_handle.join().expect("Worker thread panicked");
    */

    Ok(())
}

fn demo() {
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
    use windows_sys::Win32::System::Com::Urlmon::E_PENDING;

    use super::*;

    #[test]
    fn it_works() {
        demo();
    }

    #[test]
    fn color_frame_test() -> anyhow::Result<()> {
        let mut kinect_sensor_ptr: *mut IKinectSensor = ptr::null_mut();
        let hr =
            unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr as *mut *mut IKinectSensor) };
        assert_eq!(
            hr, 0,
            "Failed to get default Kinect sensor. HRESULT: {:#X}",
            hr
        );

        let sensor_instance: &mut IKinectSensor = unsafe { &mut *kinect_sensor_ptr };
        sensor_instance
            .open()
            .expect("Failed to open Kinect sensor");

        let color_frame_source_ptr = sensor_instance
            .color_frame_source()
            .expect("Failed to get color frame source");

        let color_frame_source = unsafe { &mut *color_frame_source_ptr };

        let color_frame_reader_ptr = color_frame_source
            .open_reader()
            .expect("Failed to open color frame reader");

        let color_frame_reader = unsafe { &mut *color_frame_reader_ptr };

        loop {
            match color_frame_reader.acquire_latest_frame() {
                Err(hr) => {
                    if hr == E_PENDING {
                        //eprintln!("No new color frame available, waiting...");
                        //std::thread::sleep(std::time::Duration::from_millis(10));
                        continue;
                    } else {
                        eprintln!("Failed to acquire latest color frame. HRESULT: {:#X}", hr);
                        return Err(anyhow::anyhow!("Failed to acquire latest color frame"));
                    }
                }
                Ok(color_frame_ptr) => {
                    eprintln!("Successfully acquired latest color frame.");
                    let color_frame = unsafe { &mut *color_frame_ptr };
                    let frame_description_ptr = color_frame
                        .get_frame_description()
                        .expect("Failed to get raw color image");

                    let frame_description = unsafe { &mut *frame_description_ptr };
                    let width = frame_description.get_width().expect("Failed to get width");
                    let height = frame_description
                        .get_height()
                        .expect("Failed to get height");

                    eprintln!("=== Color frame dimensions: {}x{}", width, height);
                    continue; // Continue to the next frame
                }
            }
        }

        Ok(())
    }
}
