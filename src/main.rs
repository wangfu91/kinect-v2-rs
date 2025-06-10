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
pub mod audio;
pub mod body;
pub mod color;
pub mod coordinate;
pub mod depth;
pub mod frame;
pub mod infrared;
pub mod kinect;

use crate::bindings::{WAIT_TIMEOUT, WAITABLE_HANDLE, WaitForSingleObject};
use windows_sys::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};

const FRAME_WAIT_TIMEOUT_MS: u32 = 100;

pub fn main() {
    if let Err(e) = color_frame_demo() {
        eprintln!("Error in color frame demo: {:?}", e);
    }
}

fn color_frame_demo() -> anyhow::Result<()> {
    let kinect_sensor =
        kinect::get_default_kinect_sensor().expect("Failed to get default Kinect sensor");

    kinect_sensor.open().expect("Failed to open Kinect sensor");

    let color_frame_source = kinect_sensor
        .color_frame_source()
        .expect("Failed to get color frame source");

    let color_frame_reader = color_frame_source
        .open_reader()
        .expect("Failed to open color frame reader");

    let mut waitable_handle: WAITABLE_HANDLE = WAITABLE_HANDLE::default();

    color_frame_reader
        .subscribe_frame_arrived(&mut waitable_handle)
        .expect("Failed to subscribe to frame arrived event");

    loop {
        let result =
            unsafe { WaitForSingleObject(waitable_handle as HANDLE, FRAME_WAIT_TIMEOUT_MS) };
        if WAIT_OBJECT_0 == result {
            match color_frame_reader.get_frame_arrived_event_data(waitable_handle) {
                Ok(event_args) => {
                    if let Ok(frame_reference) = event_args.get_frame_reference() {
                        if let Ok(color_frame) = frame_reference.acquire_frame() {
                            let frame_description = color_frame
                                .get_frame_description()
                                .expect("Failed to get frame description");

                            let width = frame_description.get_width().expect("Failed to get width");
                            let height = frame_description
                                .get_height()
                                .expect("Failed to get height");

                            let rel_time = frame_reference
                                .get_relative_time()
                                .expect("Failed to get relative time");

                            eprintln!(
                                "====== Color frame dimensions: {}x{}, time: {}",
                                width, height, rel_time
                            );
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
            eprintln!("No new color frame available, waiting...");
        } else {
            eprintln!("WaitForSingleObject failed with result: {:#X}", result);
            break;
        }
    }

    Ok(())
}
