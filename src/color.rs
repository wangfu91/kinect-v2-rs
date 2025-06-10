use crate::{
    bindings::{
        ColorImageFormat, IColorCameraSettings, IColorFrame, IColorFrameArrivedEventArgs,
        IColorFrameReader, IColorFrameReference, IColorFrameSource, IFrameCapturedEventArgs,
        IFrameDescription, TIMESPAN, WAITABLE_HANDLE,
    },
    frame::{FrameCapturedEventArgs, FrameDescription},
};
use std::ptr;
use windows_sys::{
    Win32::Foundation::{BOOLEAN, E_FAIL, E_POINTER},
    core::HRESULT,
};

pub struct ColorFrame {
    ptr: *mut IColorFrame,
}

impl ColorFrame {
    pub(crate) fn new(ptr: *mut IColorFrame) -> Self {
        assert!(!ptr.is_null(), "ColorFrame pointer cannot be null");
        Self { ptr }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut desc_ptr) };
        if hr == 0 {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_raw_color_image_format(&self) -> Result<ColorImageFormat, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RawColorImageFormat.ok_or(E_FAIL)?;
        let mut format: ColorImageFormat = ColorImageFormat::ColorImageFormat_None;
        let hr = unsafe { get_fn(self.ptr, &mut format) };
        if hr == 0 { Ok(format) } else { Err(hr) }
    }

    pub fn copy_raw_frame_data_to_array(
        &self,
        capacity: u32,
        frame_data: *mut u8,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let copy_fn = vtbl.CopyRawFrameDataToArray.ok_or(E_FAIL)?;
        let hr = unsafe { copy_fn(self.ptr, capacity, frame_data) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn access_raw_underlying_buffer(
        &self,
        capacity: &mut u32,
        buffer: &mut *mut u8,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let access_fn = vtbl.AccessRawUnderlyingBuffer.ok_or(E_FAIL)?;
        let hr = unsafe { access_fn(self.ptr, capacity, buffer) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn copy_converted_frame_data_to_array(
        &self,
        capacity: u32,
        frame_data: *mut u8,
        color_format: ColorImageFormat,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let copy_fn = vtbl.CopyConvertedFrameDataToArray.ok_or(E_FAIL)?;
        let hr = unsafe { copy_fn(self.ptr, capacity, frame_data, color_format) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn create_frame_description(
        &self,
        format: ColorImageFormat,
    ) -> Result<FrameDescription, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let create_fn = vtbl.CreateFrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { create_fn(self.ptr, format, &mut desc_ptr) };
        if hr == 0 {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_color_camera_settings(&self) -> Result<ColorCameraSettings, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorCameraSettings.ok_or(E_FAIL)?;
        let mut settings_ptr: *mut IColorCameraSettings = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut settings_ptr) };
        if hr == 0 {
            Ok(ColorCameraSettings::new(settings_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RelativeTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr == 0 { Ok(time) } else { Err(hr) }
    }

    pub fn get_color_frame_source(&self) -> Result<ColorFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr == 0 {
            Ok(ColorFrameSource::new(source_ptr))
        } else {
            Err(hr)
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

    pub fn get_frame_reference(&self) -> Result<ColorFrameReference, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IColorFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr == 0 {
            Ok(ColorFrameReference::new(ref_ptr))
        } else {
            Err(hr)
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

    pub fn acquire_frame(&self) -> Result<ColorFrame, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr == 0 {
            Ok(ColorFrame::new(frame_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_RelativeTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr == 0 { Ok(time) } else { Err(hr) }
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

    pub fn get_exposure_time(&self) -> Result<TIMESPAN, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ExposureTime.ok_or(E_FAIL)?;
        let mut time: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut time) };
        if hr == 0 { Ok(time) } else { Err(hr) }
    }

    pub fn get_frame_interval(&self) -> Result<TIMESPAN, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameInterval.ok_or(E_FAIL)?;
        let mut interval: TIMESPAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut interval) };
        if hr == 0 { Ok(interval) } else { Err(hr) }
    }

    pub fn get_gain(&self) -> Result<f32, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Gain.ok_or(E_FAIL)?;
        let mut gain: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut gain) };
        if hr == 0 { Ok(gain) } else { Err(hr) }
    }

    pub fn get_gamma(&self) -> Result<f32, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Gamma.ok_or(E_FAIL)?;
        let mut gamma: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut gamma) };
        if hr == 0 { Ok(gamma) } else { Err(hr) }
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
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn unsubscribe_frame_arrived(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn get_frame_arrived_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<ColorFrameArrivedEventArgs, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut args_ptr: *mut IColorFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut args_ptr) };
        if hr == 0 {
            Ok(ColorFrameArrivedEventArgs::new(args_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn acquire_latest_frame(&self) -> Result<ColorFrame, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr == 0 {
            Ok(ColorFrame::new(frame_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_is_paused(&self) -> Result<bool, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsPaused.ok_or(E_FAIL)?;
        let mut paused: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut paused) };
        if hr == 0 { Ok(paused != 0) } else { Err(hr) }
    }

    pub fn put_is_paused(&self, is_paused: bool) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_IsPaused.ok_or(E_FAIL)?;
        let hr = unsafe { put_fn(self.ptr, is_paused as BOOLEAN) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn get_color_frame_source(&self) -> Result<ColorFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr == 0 {
            Ok(ColorFrameSource::new(source_ptr))
        } else {
            Err(hr)
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
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn unsubscribe_frame_captured(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn get_frame_captured_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<FrameCapturedEventArgs, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameCapturedEventData.ok_or(E_FAIL)?;
        let mut args_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut args_ptr) };
        if hr == 0 {
            Ok(FrameCapturedEventArgs::new(args_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_is_active(&self) -> Result<bool, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsActive.ok_or(E_FAIL)?;
        let mut active: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut active) };
        if hr == 0 { Ok(active != 0) } else { Err(hr) }
    }

    pub fn open_reader(&self) -> Result<ColorFrameReader, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut IColorFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, &mut reader_ptr) };
        if hr == 0 {
            Ok(ColorFrameReader::new(reader_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn create_frame_description(
        &self,
        format: ColorImageFormat,
    ) -> Result<FrameDescription, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let create_fn = vtbl.CreateFrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { create_fn(self.ptr, format, &mut desc_ptr) };
        if hr == 0 {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameDescription.ok_or(E_FAIL)?;
        let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut desc_ptr) };
        if hr == 0 {
            Ok(FrameDescription::new(desc_ptr))
        } else {
            Err(hr)
        }
    }

    // Not including get_kinect_sensor to avoid circular dependencies with KinectSensor wrapper
    // If needed, it should return Result<*mut IKinectSensor, HRESULT> or a weak reference.
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
