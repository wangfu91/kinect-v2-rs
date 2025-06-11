use windows::{
    Win32::Foundation::{E_FAIL, E_POINTER},
    core::Error,
};

use crate::{
    bindings::{
        BOOLEAN, ColorImageFormat, IColorCameraSettings, IColorFrame, IColorFrameArrivedEventArgs,
        IColorFrameReader, IColorFrameReference, IColorFrameSource, IFrameCapturedEventArgs,
        IFrameDescription, TIMESPAN, WAITABLE_HANDLE,
    },
    frame::{FrameCapturedEventArgs, FrameDescription},
};
use std::ptr;

pub struct ColorFrame {
    ptr: *mut IColorFrame,
}

impl ColorFrame {
    pub(crate) fn new(ptr: *mut IColorFrame) -> Self {
        assert!(!ptr.is_null(), "ColorFrame pointer cannot be null");
        Self { ptr }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut desc_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_raw_color_image_format(&self) -> Result<ColorImageFormat, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RawColorImageFormat.ok_or(E_FAIL)?;
        let mut format: ColorImageFormat = ColorImageFormat::ColorImageFormat_None;
        let hr = unsafe { get_fn(self.ptr, &mut format) };
        if hr.is_ok() {
            Ok(format)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn copy_raw_frame_data_to_array(
        &self,
        capacity: u32,
        frame_data: *mut u8,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let copy_fn = vtbl.CopyRawFrameDataToArray.ok_or(E_FAIL)?;
        let hr = unsafe { copy_fn(self.ptr, capacity, frame_data) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn access_raw_underlying_buffer(
        &self,
        capacity: &mut u32,
        buffer: &mut *mut u8,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let access_fn = vtbl.AccessRawUnderlyingBuffer.ok_or(E_FAIL)?;
        let hr = unsafe { access_fn(self.ptr, capacity, buffer) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn copy_converted_frame_data_to_array(
        &self,
        capacity: u32,
        frame_data: *mut u8,
        color_format: ColorImageFormat,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let copy_fn = vtbl.CopyConvertedFrameDataToArray.ok_or(E_FAIL)?;
        let hr = unsafe { copy_fn(self.ptr, capacity, frame_data, color_format) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn create_frame_description(
        &self,
        format: ColorImageFormat,
    ) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let create_fn = vtbl.CreateFrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { create_fn(self.ptr, format, &mut desc_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_color_camera_settings(&self) -> Result<ColorCameraSettings, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorCameraSettings.ok_or(E_FAIL)?;
        let mut settings_ptr: *mut IColorCameraSettings = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut settings_ptr) };
        if hr.is_ok() {
            Ok(ColorCameraSettings::new(settings_ptr))
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

    pub fn get_color_frame_source(&self) -> Result<ColorFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for ColorFrame {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct ColorFrameArrivedEventArgs {
    ptr: *mut IColorFrameArrivedEventArgs,
}

impl ColorFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IColorFrameArrivedEventArgs) -> Self {
        assert!(
            !ptr.is_null(),
            "ColorFrameArrivedEventArgs pointer cannot be null"
        );
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<ColorFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IColorFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for ColorFrameArrivedEventArgs {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct ColorFrameReference {
    ptr: *mut IColorFrameReference,
}

impl ColorFrameReference {
    pub(crate) fn new(ptr: *mut IColorFrameReference) -> Self {
        assert!(!ptr.is_null(), "ColorFrameReference pointer cannot be null");
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<ColorFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(ColorFrame::new(frame_ptr))
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

impl Drop for ColorFrameReference {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct ColorCameraSettings {
    ptr: *mut IColorCameraSettings,
}

impl ColorCameraSettings {
    pub(crate) fn new(ptr: *mut IColorCameraSettings) -> Self {
        assert!(!ptr.is_null(), "ColorCameraSettings pointer cannot be null");
        Self { ptr }
    }

    pub fn get_exposure_time(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ExposureTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr.is_ok() {
            Ok(time)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_interval(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameInterval.ok_or(E_FAIL)?;
        let mut interval: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut interval) };
        if hr.is_ok() {
            Ok(interval)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_gain(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Gain.ok_or(E_FAIL)?;
        let mut gain: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut gain) };
        if hr.is_ok() {
            Ok(gain)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_gamma(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Gamma.ok_or(E_FAIL)?;
        let mut gamma: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut gamma) };
        if hr.is_ok() {
            Ok(gamma)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for ColorCameraSettings {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct ColorFrameReader {
    ptr: *mut IColorFrameReader,
}

impl ColorFrameReader {
    pub(crate) fn new(ptr: *mut IColorFrameReader) -> Self {
        assert!(!ptr.is_null(), "ColorFrameReader pointer cannot be null");
        Self { ptr }
    }

    pub fn subscribe_frame_arrived(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_frame_arrived(&self, waitable_handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_arrived_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<ColorFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut args_ptr: *mut IColorFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut args_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameArrivedEventArgs::new(args_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn acquire_latest_frame(&self) -> Result<ColorFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(ColorFrame::new(frame_ptr))
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
        let mut paused: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut paused) };
        if hr.is_ok() {
            Ok(paused != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn put_is_paused(&self, is_paused: bool) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_IsPaused.ok_or(E_FAIL)?;
        let hr = unsafe { put_fn(self.ptr, is_paused as BOOLEAN) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_color_frame_source(&self) -> Result<ColorFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for ColorFrameReader {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct ColorFrameSource {
    pub(crate) ptr: *mut IColorFrameSource, // Made pub(crate) for kinect_sensor.rs
}

impl ColorFrameSource {
    pub(crate) fn new(ptr: *mut IColorFrameSource) -> Self {
        assert!(!ptr.is_null(), "ColorFrameSource pointer cannot be null");
        Self { ptr }
    }

    pub fn subscribe_frame_captured(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
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
        let unsub_fn = vtbl.UnsubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, waitable_handle) };
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
        let get_fn = vtbl.GetFrameCapturedEventData.ok_or(E_FAIL)?;
        let mut args_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut args_ptr) };
        if hr.is_ok() {
            Ok(FrameCapturedEventArgs::new(args_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_active(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsActive.ok_or(E_FAIL)?;
        let mut active: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut active) };
        if hr.is_ok() {
            Ok(active != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn open_reader(&self) -> Result<ColorFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut IColorFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, &mut reader_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameReader::new(reader_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn create_frame_description(
        &self,
        format: ColorImageFormat,
    ) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let create_fn = vtbl.CreateFrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { create_fn(self.ptr, format, &mut desc_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(desc_ptr))
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
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut desc_ptr) };
        if hr.is_ok() {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    // Not including get_kinect_sensor to avoid circular dependencies with KinectSensor wrapper
    // If needed, it should return Result<*mut IKinectSensor, Error> or a weak reference.
}

impl Drop for ColorFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    use crate::{FRAME_WAIT_TIMEOUT_MS, kinect};
    use anyhow::Context;
    use windows::Win32::{
        Foundation::{WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::{Com::Urlmon::E_PENDING, Threading::WaitForSingleObject},
    };

    #[test]
    fn get_latest_color_frame() -> anyhow::Result<()> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;
        let color_frame_source = kinect.color_frame_source()?;
        let color_frame_reader = color_frame_source.open_reader()?;

        let mut frame_count = 0;
        loop {
            match color_frame_reader.acquire_latest_frame() {
                Ok(color_frame) => {
                    let frame_description = color_frame.get_frame_description()?;
                    let width = frame_description.get_width()?;
                    let height = frame_description.get_height()?;
                    assert_eq!(width, 1920);
                    assert_eq!(height, 1080);
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
    fn subscribe_frame_arrived_event() -> anyhow::Result<()> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;
        let color_frame_source = kinect.color_frame_source()?;
        let color_frame_reader = color_frame_source.open_reader()?;
        let mut waitable_handle: WAITABLE_HANDLE = WAITABLE_HANDLE::default();
        color_frame_reader.subscribe_frame_arrived(&mut waitable_handle)?;
        let mut frame_count = 0;
        loop {
            let result = unsafe { WaitForSingleObject(waitable_handle, FRAME_WAIT_TIMEOUT_MS) };
            if WAIT_OBJECT_0 == result {
                let event_args = color_frame_reader
                    .get_frame_arrived_event_data(waitable_handle)
                    .context("Failed to get frame arrived event data")?;

                let frame_reference = event_args.get_frame_reference()?;
                let color_frame = frame_reference.acquire_frame()?;
                let frame_description = color_frame.get_frame_description()?;
                let width = frame_description.get_width()?;
                let height = frame_description.get_height()?;
                let rel_time = frame_reference.get_relative_time()?;

                assert_eq!(width, 1920);
                assert_eq!(height, 1080);
                assert!(rel_time > 0);

                frame_count += 1;
                if frame_count > 10 {
                    break; // Exit after getting 10 frames
                }
            } else if WAIT_TIMEOUT == result {
                eprintln!("No new color frame available, waiting...");
            } else {
                return Err(anyhow::anyhow!(
                    "WaitForSingleObject failed with result: {:?}",
                    result
                ));
            }
        }

        // Unsubscribe after testing
        color_frame_reader.unsubscribe_frame_arrived(waitable_handle)?;

        Ok(())
    }

    // Additional tests can be added here for other methods and functionalities
}
