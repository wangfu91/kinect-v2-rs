use crate::bindings::{
    BOOLEAN, IFrameCapturedEventArgs, IFrameDescription, IInfraredFrame,
    IInfraredFrameArrivedEventArgs, IInfraredFrameReader, IInfraredFrameReference,
    IInfraredFrameSource, IKinectSensor, TIMESPAN, UINT, UINT16, WAITABLE_HANDLE,
};
use crate::frame::{FrameCapturedEventArgs, FrameDescription};
use crate::kinect::KinectSensor;
use std::ptr;
use windows::Win32::Foundation::{E_FAIL, E_POINTER};
use windows::core::Error;

pub struct InfraredFrame {
    ptr: *mut IInfraredFrame,
}

impl InfraredFrame {
    pub(crate) fn new(ptr: *mut IInfraredFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn copy_frame_data_to_array(
        &self,
        capacity: UINT,
        frame_data: *mut UINT16,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let copy_data_fn = vtbl
            .CopyFrameDataToArray
            .ok_or_else(|| Error::from(E_FAIL))?;
        let hr = unsafe { copy_data_fn(self.ptr, capacity, frame_data) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn access_underlying_buffer(&self) -> Result<(UINT, *mut UINT16), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let access_buffer_fn = vtbl
            .AccessUnderlyingBuffer
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut capacity: UINT = 0;
        let mut buffer: *mut UINT16 = ptr::null_mut();
        let hr = unsafe { access_buffer_fn(self.ptr, &mut capacity, &mut buffer) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok((capacity, buffer))
        }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_description_fn = vtbl
            .get_FrameDescription
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut frame_description_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_description_fn(self.ptr, &mut frame_description_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(FrameDescription::new(frame_description_ptr))
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_time_fn = vtbl.get_RelativeTime.ok_or_else(|| Error::from(E_FAIL))?;
        let mut relative_time: TIMESPAN = 0;
        let hr = unsafe { get_time_fn(self.ptr, &mut relative_time) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(relative_time)
        }
    }

    pub fn get_infrared_frame_source(&self) -> Result<InfraredFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_source_fn = vtbl
            .get_InfraredFrameSource
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut infrared_frame_source_ptr: *mut IInfraredFrameSource = ptr::null_mut();
        let hr = unsafe { get_source_fn(self.ptr, &mut infrared_frame_source_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if infrared_frame_source_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrameSource::new(infrared_frame_source_ptr))
            }
        }
    }
}

impl Drop for InfraredFrame {
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

pub struct InfraredFrameSource {
    ptr: *mut IInfraredFrameSource,
}
impl InfraredFrameSource {
    pub(crate) fn new(ptr: *mut IInfraredFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_frame_captured(&self) -> Result<WAITABLE_HANDLE, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let subscribe_fn = vtbl
            .SubscribeFrameCaptured
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut waitable_handle: WAITABLE_HANDLE = windows::Win32::Foundation::HANDLE::default();
        let hr = unsafe { subscribe_fn(self.ptr, &mut waitable_handle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(waitable_handle)
        }
    }

    pub fn unsubscribe_frame_captured(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let unsubscribe_fn = vtbl
            .UnsubscribeFrameCaptured
            .ok_or_else(|| Error::from(E_FAIL))?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn get_frame_captured_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<FrameCapturedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_data_fn = vtbl
            .GetFrameCapturedEventData
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut event_args_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_args_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(FrameCapturedEventArgs::new(event_args_ptr))
        }
    }

    pub fn get_is_active(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_active_fn = vtbl.get_IsActive.ok_or_else(|| Error::from(E_FAIL))?;
        let mut is_active: BOOLEAN = 0;
        let hr = unsafe { get_active_fn(self.ptr, &mut is_active) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(is_active != 0)
        }
    }

    pub fn open_reader(&self) -> Result<InfraredFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_reader_fn = vtbl.OpenReader.ok_or_else(|| Error::from(E_FAIL))?;
        let mut reader_ptr: *mut IInfraredFrameReader = ptr::null_mut();
        let hr = unsafe { open_reader_fn(self.ptr, &mut reader_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if reader_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrameReader::new(reader_ptr))
            }
        }
    }

    pub fn get_frame_description(&self) -> Result<FrameDescription, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_description_fn = vtbl
            .get_FrameDescription
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut frame_description_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_description_fn(self.ptr, &mut frame_description_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(FrameDescription::new(frame_description_ptr))
        }
    }

    pub fn get_kinect_sensor(&self) -> Result<KinectSensor, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_sensor_fn = vtbl.get_KinectSensor.ok_or_else(|| Error::from(E_FAIL))?;
        let mut sensor: *mut IKinectSensor = ptr::null_mut();
        let hr = unsafe { get_sensor_fn(self.ptr, &mut sensor) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(KinectSensor::new(sensor))
        }
    }
}

impl Drop for InfraredFrameSource {
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

pub struct InfraredFrameReader {
    ptr: *mut IInfraredFrameReader,
}

impl InfraredFrameReader {
    pub(crate) fn new(ptr: *mut IInfraredFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_frame_arrived(&self) -> Result<WAITABLE_HANDLE, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let subscribe_fn = vtbl
            .SubscribeFrameArrived
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut waitable_handle: WAITABLE_HANDLE = windows::Win32::Foundation::HANDLE::default();
        let hr = unsafe { subscribe_fn(self.ptr, &mut waitable_handle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(waitable_handle)
        }
    }

    pub fn unsubscribe_frame_arrived(&self, waitable_handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let unsubscribe_fn = vtbl
            .UnsubscribeFrameArrived
            .ok_or_else(|| Error::from(E_FAIL))?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }
    pub fn get_frame_arrived_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<InfraredFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_data_fn = vtbl
            .GetFrameArrivedEventData
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut event_data: *mut IInfraredFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if event_data.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrameArrivedEventArgs::new(event_data))
            }
        }
    }

    pub fn acquire_latest_frame(&self) -> Result<InfraredFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or_else(|| Error::from(E_FAIL))?;
        let mut frame_ptr: *mut IInfraredFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if frame_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrame::new(frame_ptr))
            }
        }
    }

    pub fn get_is_paused(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_paused_fn = vtbl.get_IsPaused.ok_or_else(|| Error::from(E_FAIL))?;
        let mut is_paused: BOOLEAN = 0;
        let hr = unsafe { get_paused_fn(self.ptr, &mut is_paused) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(is_paused != 0)
        }
    }

    pub fn put_is_paused(&self, is_paused: bool) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let put_paused_fn = vtbl.put_IsPaused.ok_or_else(|| Error::from(E_FAIL))?;
        let paused_value: BOOLEAN = if is_paused { 1 } else { 0 };
        let hr = unsafe { put_paused_fn(self.ptr, paused_value) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn get_infrared_frame_source(&self) -> Result<InfraredFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_source_fn = vtbl
            .get_InfraredFrameSource
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut source_ptr: *mut IInfraredFrameSource = ptr::null_mut();
        let hr = unsafe { get_source_fn(self.ptr, &mut source_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if source_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrameSource::new(source_ptr))
            }
        }
    }
}

impl Drop for InfraredFrameReader {
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

pub struct InfraredFrameReference {
    ptr: *mut IInfraredFrameReference,
}

impl InfraredFrameReference {
    pub(crate) fn new(ptr: *mut IInfraredFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<InfraredFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let acquire_frame_fn = vtbl.AcquireFrame.ok_or_else(|| Error::from(E_FAIL))?;
        let mut frame_ptr: *mut IInfraredFrame = ptr::null_mut();
        let hr = unsafe { acquire_frame_fn(self.ptr, &mut frame_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if frame_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrame::new(frame_ptr))
            }
        }
    }

    pub fn get_relative_time(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_time_fn = vtbl.get_RelativeTime.ok_or_else(|| Error::from(E_FAIL))?;
        let mut relative_time: TIMESPAN = 0;
        let hr = unsafe { get_time_fn(self.ptr, &mut relative_time) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(relative_time)
        }
    }
}

impl Drop for InfraredFrameReference {
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

pub struct InfraredFrameArrivedEventArgs {
    ptr: *mut IInfraredFrameArrivedEventArgs,
}

impl InfraredFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IInfraredFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<InfraredFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_frame_reference_fn = vtbl.get_FrameReference.ok_or_else(|| Error::from(E_FAIL))?;
        let mut frame_reference_ptr: *mut IInfraredFrameReference = ptr::null_mut();
        let hr = unsafe { get_frame_reference_fn(self.ptr, &mut frame_reference_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if frame_reference_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(InfraredFrameReference::new(frame_reference_ptr))
            }
        }
    }
}

impl Drop for InfraredFrameArrivedEventArgs {
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
