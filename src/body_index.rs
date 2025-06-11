use crate::bindings::{
    BOOLEAN, IBodyIndexFrame, IBodyIndexFrameReader, IBodyIndexFrameReference,
    IBodyIndexFrameSource, IFrameCapturedEventArgs, IFrameDescription, IKinectSensor, TIMESPAN,
    WAITABLE_HANDLE,
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
    // TODO: Implement methods for BodyIndexFrameReader
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
    // TODO: Implement methods for BodyIndexFrame
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
