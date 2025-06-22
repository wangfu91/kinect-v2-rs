use crate::bindings::{
    BOOLEAN, BYTE, IBodyIndexFrame, IBodyIndexFrameArrivedEventArgs, IBodyIndexFrameReader,
    IBodyIndexFrameReference, IBodyIndexFrameSource, IFrameCapturedEventArgs, IFrameDescription,
    IKinectSensor, TIMESPAN, UINT, WAITABLE_HANDLE,
};
use crate::frame::{FrameCapturedEventArgs, FrameDescription};
use crate::kinect::KinectSensor;
use std::ptr;
use windows::Win32::Foundation::{E_FAIL, E_POINTER};
use windows::core::Error;

pub struct BodyIndexFrameReader {
    ptr: *mut IBodyIndexFrameReader,
}

impl BodyIndexFrameReader {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_latest_frame(&self) -> Result<BodyIndexFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IBodyIndexFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            if frame_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
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

    pub fn unsubscribe_frame_arrived(&self, handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_arrived_event_data(
        &self,
        handle: WAITABLE_HANDLE,
    ) -> Result<BodyIndexFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut event_data_ptr: *mut IBodyIndexFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, handle, &mut event_data_ptr) };
        if hr.is_ok() {
            if event_data_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrameArrivedEventArgs::new(event_data_ptr))
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

    pub fn get_body_index_frame_source(&self) -> Result<BodyIndexFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyIndexFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IBodyIndexFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            if source_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}
impl Drop for BodyIndexFrameReader {
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

pub struct BodyIndexFrame {
    ptr: *mut IBodyIndexFrame,
}

impl BodyIndexFrame {
    pub(crate) fn new(ptr: *mut IBodyIndexFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn copy_frame_data_to_array(&self, frame_data: &mut [u8]) -> Result<(), Error> {
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
    pub fn access_underlying_buffer(&self) -> Result<&[u8], Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let access_fn = vtbl.AccessUnderlyingBuffer.ok_or(E_FAIL)?;
        let mut capacity: UINT = 0;
        let mut buffer: *mut BYTE = ptr::null_mut();
        let hr = unsafe { access_fn(self.ptr, &mut capacity, &mut buffer) };
        if hr.is_ok() {
            if buffer.is_null() || capacity == 0 {
                Err(Error::from_hresult(E_POINTER))
            } else {
                // Create a safe slice from the raw pointer
                let slice =
                    unsafe { std::slice::from_raw_parts(buffer as *const u8, capacity as usize) };
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
        let mut fd_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut fd_ptr) };
        if hr.is_ok() {
            if fd_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(FrameDescription::new(fd_ptr))
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

    pub fn get_body_index_frame_source(&self) -> Result<BodyIndexFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyIndexFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IBodyIndexFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            if source_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}
impl Drop for BodyIndexFrame {
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

pub struct BodyIndexFrameReference {
    ptr: *mut IBodyIndexFrameReference,
}

impl BodyIndexFrameReference {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<BodyIndexFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IBodyIndexFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            if frame_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrame::new(frame_ptr))
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
impl Drop for BodyIndexFrameReference {
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

pub struct BodyIndexFrameSource {
    ptr: *mut IBodyIndexFrameSource,
}
impl BodyIndexFrameSource {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameSource) -> Self {
        assert!(!ptr.is_null());
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

    pub fn unsubscribe_frame_captured(&self, handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_captured_event_data(
        &self,
        handle: WAITABLE_HANDLE,
    ) -> Result<FrameCapturedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameCapturedEventData.ok_or(E_FAIL)?;
        let mut event_data_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, handle, &mut event_data_ptr) };
        if hr.is_ok() {
            if event_data_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(FrameCapturedEventArgs::new(event_data_ptr))
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
        let mut is_active: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut is_active) };
        if hr.is_ok() {
            Ok(is_active != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn open_reader(&self) -> Result<BodyIndexFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut IBodyIndexFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, &mut reader_ptr) };
        if hr.is_ok() {
            if reader_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrameReader::new(reader_ptr))
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
        let mut fd_ptr: *mut IFrameDescription = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut fd_ptr) };
        if hr.is_ok() {
            if fd_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(FrameDescription::new(fd_ptr))
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
            if sensor_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(KinectSensor::new(sensor_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}
impl Drop for BodyIndexFrameSource {
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

pub struct BodyIndexFrameArrivedEventArgs {
    ptr: *mut IBodyIndexFrameArrivedEventArgs,
}

impl BodyIndexFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<BodyIndexFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut frame_ref_ptr: *mut IBodyIndexFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_ref_ptr) };
        if hr.is_ok() {
            if frame_ref_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyIndexFrameReference::new(frame_ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for BodyIndexFrameArrivedEventArgs {
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
