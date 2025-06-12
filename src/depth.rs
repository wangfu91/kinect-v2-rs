use crate::bindings::{
    BOOLEAN, IDepthFrame, IDepthFrameArrivedEventArgs, IDepthFrameReader, IDepthFrameReference,
    IDepthFrameSource, IFrameCapturedEventArgs, IFrameDescription, IKinectSensor, TIMESPAN, UINT,
    UINT16, WAITABLE_HANDLE,
};
use crate::frame::{FrameCapturedEventArgs, FrameDescription};
use crate::kinect::KinectSensor;
use std::ptr;
use windows::{
    Win32::Foundation::{E_FAIL, E_POINTER},
    core::Error,
};

pub struct DepthFrameSource {
    ptr: *mut IDepthFrameSource,
}

impl DepthFrameSource {
    pub(crate) fn new(ptr: *mut IDepthFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_frame_captured(&self) -> Result<WAITABLE_HANDLE, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let subscribe_fn = vtbl.SubscribeFrameCaptured.ok_or(E_FAIL)?;
        let mut waitable_handle: WAITABLE_HANDLE = windows::Win32::Foundation::HANDLE::default();
        let hr = unsafe { subscribe_fn(self.ptr, &mut waitable_handle) };
        if hr.is_ok() {
            Ok(waitable_handle)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_frame_captured(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsubscribe_fn = vtbl.UnsubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_captured_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<FrameCapturedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_data_fn = vtbl.GetFrameCapturedEventData.ok_or(E_FAIL)?;
        let mut event_data: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_ok() {
            Ok(FrameCapturedEventArgs::new(event_data))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_active(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_active_fn = vtbl.get_IsActive.ok_or(E_FAIL)?;
        let mut is_active: BOOLEAN = 0;
        let hr = unsafe { get_active_fn(self.ptr, &mut is_active) };
        if hr.is_ok() {
            Ok(is_active != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn open_reader(&self) -> Result<DepthFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut IDepthFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, &mut reader_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameReader::new(reader_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_min_reliable_distance(&self) -> Result<UINT16, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthMinReliableDistance.ok_or(E_FAIL)?;
        let mut distance: UINT16 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut distance) };
        if hr.is_ok() {
            Ok(distance)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_max_reliable_distance(&self) -> Result<UINT16, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthMaxReliableDistance.ok_or(E_FAIL)?;
        let mut distance: UINT16 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut distance) };
        if hr.is_ok() {
            Ok(distance)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameDescription.ok_or(E_FAIL)?;
        let mut frame_description_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_description_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(frame_description_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_kinect_sensor(&self) -> Result<KinectSensor, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_KinectSensor.ok_or(E_FAIL)?;
        let mut sensor_ptr: *mut IKinectSensor = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut sensor_ptr) };
        if hr.is_ok() {
            Ok(KinectSensor::new(sensor_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for DepthFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref() {
                    if let Some(release_fn) = vtbl.Release {
                        release_fn(self.ptr);
                    }
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct DepthFrameReader {
    ptr: *mut IDepthFrameReader,
}

impl DepthFrameReader {
    pub(crate) fn new(ptr: *mut IDepthFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_frame_arrived(&self) -> Result<WAITABLE_HANDLE, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let subscribe_fn = vtbl.SubscribeFrameArrived.ok_or(E_FAIL)?;
        let mut waitable_handle: WAITABLE_HANDLE = windows::Win32::Foundation::HANDLE::default();
        let hr = unsafe { subscribe_fn(self.ptr, &mut waitable_handle) };
        if hr.is_ok() {
            Ok(waitable_handle)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_frame_arrived(&self, waitable_handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsubscribe_fn = vtbl.UnsubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_arrived_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<DepthFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut args_ptr: *mut IDepthFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut args_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameArrivedEventArgs::new(args_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn acquire_latest_frame(&self) -> Result<DepthFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IDepthFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(DepthFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_paused(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsPaused.ok_or(E_FAIL)?;
        let mut is_paused: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut is_paused) };
        if hr.is_ok() {
            Ok(is_paused != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn set_is_paused(&self, is_paused: bool) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_IsPaused.ok_or(E_FAIL)?;
        let paused_value: BOOLEAN = if is_paused { 1 } else { 0 };
        let hr = unsafe { put_fn(self.ptr, paused_value) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_frame_source(&self) -> Result<DepthFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IDepthFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for DepthFrameReader {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref() {
                    if let Some(release_fn) = vtbl.Release {
                        release_fn(self.ptr);
                    }
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct DepthFrameArrivedEventArgs {
    ptr: *mut IDepthFrameArrivedEventArgs,
}

impl DepthFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IDepthFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<DepthFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IDepthFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for DepthFrameArrivedEventArgs {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref() {
                    if let Some(release_fn) = vtbl.Release {
                        release_fn(self.ptr);
                    }
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct DepthFrameReference {
    ptr: *mut IDepthFrameReference,
}

impl DepthFrameReference {
    pub(crate) fn new(ptr: *mut IDepthFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<DepthFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IDepthFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(DepthFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RelativeTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr.is_ok() {
            Ok(time)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for DepthFrameReference {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref() {
                    if let Some(release_fn) = vtbl.Release {
                        release_fn(self.ptr);
                    }
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct DepthFrame {
    ptr: *mut IDepthFrame,
}

impl DepthFrame {
    pub(crate) fn new(ptr: *mut IDepthFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
    pub fn copy_frame_data_to_array(&self, frame_data: &mut [u16]) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let copy_fn = vtbl.CopyFrameDataToArray.ok_or(E_FAIL)?;
        let capacity = frame_data.len() as UINT;
        let hr = unsafe { copy_fn(self.ptr, capacity, frame_data.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }
    pub fn access_underlying_buffer(&self) -> Result<&[u16], Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let access_fn = vtbl.AccessUnderlyingBuffer.ok_or(E_FAIL)?;
        let mut capacity: UINT = 0;
        let mut buffer: *mut UINT16 = ptr::null_mut();
        let hr = unsafe { access_fn(self.ptr, &mut capacity, &mut buffer) };
        if hr.is_ok() {
            if buffer.is_null() || capacity == 0 {
                Err(Error::from_hresult(E_POINTER))
            } else {
                // Create a safe slice from the raw pointer
                let slice =
                    unsafe { std::slice::from_raw_parts(buffer as *const u16, capacity as usize) };
                Ok(slice)
            }
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameDescription.ok_or(E_FAIL)?;
        let mut frame_description_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_description_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(frame_description_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RelativeTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr.is_ok() {
            Ok(time)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_frame_source(&self) -> Result<DepthFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IDepthFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_min_reliable_distance(&self) -> Result<UINT16, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthMinReliableDistance.ok_or(E_FAIL)?;
        let mut distance: UINT16 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut distance) };
        if hr.is_ok() {
            Ok(distance)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_max_reliable_distance(&self) -> Result<UINT16, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthMaxReliableDistance.ok_or(E_FAIL)?;
        let mut distance: UINT16 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut distance) };
        if hr.is_ok() {
            Ok(distance)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for DepthFrame {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref() {
                    if let Some(release_fn) = vtbl.Release {
                        release_fn(self.ptr);
                    }
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{FRAME_WAIT_TIMEOUT_MS, kinect};
    use anyhow::Context;
    use windows::Win32::{
        Foundation::{WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::{Com::Urlmon::E_PENDING, Threading::WaitForSingleObject},
    };

    #[test]
    fn get_latest_depth_frame() -> anyhow::Result<()> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;
        let depth_frame_source = kinect.depth_frame_source()?;
        let depth_frame_reader = depth_frame_source.open_reader()?;

        let mut frame_count = 0;
        loop {
            match depth_frame_reader.acquire_latest_frame() {
                Ok(depth_frame) => {
                    let frame_description = depth_frame.get_frame_description()?;
                    let width = frame_description.get_width()?;
                    let height = frame_description.get_height()?;
                    assert_eq!(width, 512);
                    assert_eq!(height, 424);
                    frame_count += 1;

                    if frame_count > 10 {
                        break; // Exit after getting 10 frames
                    }
                }
                Err(e) => {
                    if e.code() == E_PENDING.into() {
                        // If the frame is not ready yet, wait and try again
                        thread::sleep(Duration::from_millis(100));
                    } else {
                        // If it's a different error, return it
                        return Err(anyhow::Error::new(e));
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn subscribe_depth_frame_arrived_event() -> anyhow::Result<()> {
        let kinect_sensor = kinect::get_default_kinect_sensor()?;
        kinect_sensor.open()?;

        let depth_frame_source = kinect_sensor
            .depth_frame_source()
            .context("Failed to get depth frame source")?;

        let depth_frame_reader = depth_frame_source.open_reader()?;
        let waitable_handle = depth_frame_reader.subscribe_frame_arrived()?;
        let mut frame_count = 0;

        let is_active = depth_frame_source.get_is_active()?;
        assert!(is_active, "Depth frame source should be active");

        loop {
            let result = unsafe { WaitForSingleObject(waitable_handle, FRAME_WAIT_TIMEOUT_MS) };
            if result == WAIT_OBJECT_0 {
                let event_args =
                    depth_frame_reader.get_frame_arrived_event_data(waitable_handle)?;
                let frame_reference = event_args.get_frame_reference()?;
                let depth_frame = frame_reference.acquire_frame()?;
                let frame_description = depth_frame.get_frame_description()?;
                let width = frame_description.get_width()? as u32;
                let height = frame_description.get_height()? as u32;
                let rel_time = frame_reference.get_relative_time()?;
                let bytes_per_pixel = frame_description.get_bytes_per_pixel()?;

                assert_eq!(width, 512);
                assert_eq!(height, 424);
                assert!(rel_time > 0);
                assert_eq!(bytes_per_pixel, 2); // The capacity for CopyFrameDataToArray is the number of UINT16 elements.
                // A depth frame consists of width * height pixels, where each pixel is a UINT16.
                let capacity = width * height;
                let mut frame_data: Vec<u16> = vec![0; capacity as usize];
                depth_frame
                    .copy_frame_data_to_array(&mut frame_data)
                    .context("Failed to copy depth frame data to array")?;
                println!("depth frame data len: {:?}", frame_data.len());

                frame_count += 1;
                if frame_count > 10 {
                    break; // Exit after getting 10 frames
                }
            } else if result == WAIT_TIMEOUT {
                println!("No new depth frame arrived within the timeout period");
            } else {
                eprintln!("Error waiting for depth frame: {:?}", result);
                break;
            }
        }

        Ok(())
    }
}
