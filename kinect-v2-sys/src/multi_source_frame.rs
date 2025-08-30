use crate::bindings::{
    BOOLEAN, IBodyFrameReference, IBodyIndexFrameReference, IColorFrameReference,
    IDepthFrameReference, IInfraredFrameReference, IKinectSensor,
    ILongExposureInfraredFrameReference, IMultiSourceFrame, IMultiSourceFrameArrivedEventArgs,
    IMultiSourceFrameReader, IMultiSourceFrameReference, ULONG, WAITABLE_HANDLE,
};
use crate::body::BodyFrameReference;
use crate::body_index::BodyIndexFrameReference;
use crate::depth::DepthFrameReference;
use crate::kinect::KinectSensor;
use crate::{
    color::ColorFrameReference, infrared::InfraredFrameReference,
    long_exposure_infrared::LongExposureInfraredFrameReference,
};
use std::ptr;
use windows::{
    Win32::Foundation::{E_FAIL, E_POINTER},
    core::Error,
};

pub struct MultiSourceFrameReader {
    ptr: *mut IMultiSourceFrameReader,
}

impl MultiSourceFrameReader {
    pub(crate) fn new(ptr: *mut IMultiSourceFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_multi_source_frame_arrived(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let subscribe_fn = vtbl.SubscribeMultiSourceFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { subscribe_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_multi_source_frame_arrived(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsubscribe_fn = vtbl.UnsubscribeMultiSourceFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_multi_source_frame_arrived_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<MultiSourceFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_data_fn = vtbl.GetMultiSourceFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut event_data: *mut IMultiSourceFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_ok() {
            Ok(MultiSourceFrameArrivedEventArgs::new(event_data))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn acquire_latest_frame(&self) -> Result<MultiSourceFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IMultiSourceFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(MultiSourceFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_source_types(&self) -> Result<ULONG, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameSourceTypes.ok_or(E_FAIL)?;
        let mut frame_source_types: ULONG = 0;
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_types) };
        if hr.is_ok() {
            Ok(frame_source_types)
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

impl Drop for MultiSourceFrameReader {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref()
                    && let Some(release_fn) = vtbl.Release
                {
                    release_fn(self.ptr);
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct MultiSourceFrameArrivedEventArgs {
    ptr: *mut IMultiSourceFrameArrivedEventArgs,
}

impl MultiSourceFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IMultiSourceFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<MultiSourceFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IMultiSourceFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(MultiSourceFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for MultiSourceFrameArrivedEventArgs {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref()
                    && let Some(release_fn) = vtbl.Release
                {
                    release_fn(self.ptr);
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct MultiSourceFrameReference {
    ptr: *mut IMultiSourceFrameReference,
}

impl MultiSourceFrameReference {
    pub(crate) fn new(ptr: *mut IMultiSourceFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<MultiSourceFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IMultiSourceFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            Ok(MultiSourceFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for MultiSourceFrameReference {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref()
                    && let Some(release_fn) = vtbl.Release
                {
                    release_fn(self.ptr);
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct MultiSourceFrame {
    ptr: *mut IMultiSourceFrame,
}

impl MultiSourceFrame {
    pub(crate) fn new(ptr: *mut IMultiSourceFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_color_frame_reference(&self) -> Result<ColorFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IColorFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(ColorFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_frame_reference(&self) -> Result<DepthFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IDepthFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(DepthFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_body_frame_reference(&self) -> Result<BodyFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IBodyFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(BodyFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_body_index_frame_reference(&self) -> Result<BodyIndexFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyIndexFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IBodyIndexFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(BodyIndexFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_infrared_frame_reference(&self) -> Result<InfraredFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_InfraredFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut IInfraredFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(InfraredFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_long_exposure_infrared_frame_reference(
        &self,
    ) -> Result<LongExposureInfraredFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_LongExposureInfraredFrameReference.ok_or(E_FAIL)?;
        let mut ref_ptr: *mut ILongExposureInfraredFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut ref_ptr) };
        if hr.is_ok() {
            Ok(LongExposureInfraredFrameReference::new(ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for MultiSourceFrame {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                if let Some(vtbl) = (*self.ptr).lpVtbl.as_ref()
                    && let Some(release_fn) = vtbl.Release
                {
                    release_fn(self.ptr);
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}
