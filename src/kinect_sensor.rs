use windows_sys::{
    Win32::Foundation::{BOOLEAN, E_FAIL, E_POINTER},
    core::HRESULT,
};

use crate::bindings::{self, DWORD, GetDefaultKinectSensor, UINT, WAITABLE_HANDLE, WCHAR};
use crate::color_frame::{
    AudioSource, BodyFrameSource, BodyIndexFrameSource, ColorFrameSource, CoordinateMapper,
    DepthFrameSource, InfraredFrameSource, IsAvailableChangedEventArgs,
    LongExposureInfraredFrameSource, MultiSourceFrameReader,
};
use std::os::windows::ffi::OsStringExt;
use std::{ffi::OsString, ptr};

pub struct KinectSensor {
    ptr: *mut bindings::IKinectSensor,
}

impl KinectSensor {
    fn new(ptr: *mut bindings::IKinectSensor) -> Self {
        assert!(!ptr.is_null(), "KinectSensor pointer cannot be null");
        Self { ptr }
    }

    pub fn subscribe_is_available_changed(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let subscribe_fn = vtbl.SubscribeIsAvailableChanged.ok_or(E_FAIL)?;
        let hr = unsafe { subscribe_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn unsubscribe_is_available_changed(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsubscribe_fn = vtbl.UnsubscribeIsAvailableChanged.ok_or(E_FAIL)?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn get_is_available_changed_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<IsAvailableChangedEventArgs, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_event_data_fn = vtbl.GetIsAvailableChangedEventData.ok_or(E_FAIL)?;
        let mut event_data_ptr: *mut bindings::IIsAvailableChangedEventArgs = ptr::null_mut();
        let hr = unsafe { get_event_data_fn(self.ptr, waitable_handle, &mut event_data_ptr) };
        if hr == 0 {
            Ok(IsAvailableChangedEventArgs::new(event_data_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn open(&self) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.Open.ok_or(E_FAIL)?;
        let hr = unsafe { open_fn(self.ptr) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn close(&self) -> Result<(), HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let close_fn = vtbl.Close.ok_or(E_FAIL)?;
        let hr = unsafe { close_fn(self.ptr) };
        if hr == 0 { Ok(()) } else { Err(hr) }
    }

    pub fn is_open(&self) -> Result<bool, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_is_open_fn = vtbl.get_IsOpen.ok_or(E_FAIL)?;
        let mut is_open_val: BOOLEAN = 0;
        let hr = unsafe { get_is_open_fn(self.ptr, &mut is_open_val) };
        if hr == 0 {
            Ok(is_open_val != 0)
        } else {
            Err(hr)
        }
    }

    pub fn is_available(&self) -> Result<bool, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_is_available_fn = vtbl.get_IsAvailable.ok_or(E_FAIL)?;
        let mut is_available: BOOLEAN = 0;
        let hr = unsafe { get_is_available_fn(self.ptr, &mut is_available) };
        if hr == 0 {
            Ok(is_available != 0)
        } else {
            Err(hr)
        }
    }

    pub fn color_frame_source(&self) -> Result<ColorFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ColorFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::IColorFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(ColorFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn depth_frame_source(&self) -> Result<DepthFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DepthFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::IDepthFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(DepthFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn body_frame_source(&self) -> Result<BodyFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::IBodyFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(BodyFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn body_index_frame_source(&self) -> Result<BodyIndexFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyIndexFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::IBodyIndexFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(BodyIndexFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn infrared_frame_source(&self) -> Result<InfraredFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_InfraredFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::IInfraredFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(InfraredFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn long_exposure_infrared_frame_source(
        &self,
    ) -> Result<LongExposureInfraredFrameSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_LongExposureInfraredFrameSource.ok_or(E_FAIL)?;
        let mut frame_source_ptr: *mut bindings::ILongExposureInfraredFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_source_ptr) };
        if hr == 0 {
            Ok(LongExposureInfraredFrameSource::new(frame_source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn audio_source(&self) -> Result<AudioSource, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_AudioSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut bindings::IAudioSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr == 0 {
            Ok(AudioSource::new(source_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn open_multi_source_frame_reader(
        &self,
        enabled_frame_source_types: DWORD,
    ) -> Result<MultiSourceFrameReader, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenMultiSourceFrameReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut bindings::IMultiSourceFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, enabled_frame_source_types, &mut reader_ptr) };
        if hr == 0 {
            Ok(MultiSourceFrameReader::new(reader_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn coordinate_mapper(&self) -> Result<CoordinateMapper, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_CoordinateMapper.ok_or(E_FAIL)?;
        let mut mapper_ptr: *mut bindings::ICoordinateMapper = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut mapper_ptr) };
        if hr == 0 {
            Ok(CoordinateMapper::new(mapper_ptr))
        } else {
            Err(hr)
        }
    }

    pub fn unique_kinect_id(&self) -> Result<String, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        const BUFFER_SIZE: usize = 256; // Recommended buffer size for UniqueKinectId
        let mut buffer: [WCHAR; BUFFER_SIZE] = [0; BUFFER_SIZE];

        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_id_fn = vtbl.get_UniqueKinectId.ok_or(E_FAIL)?;
        let hr = unsafe { get_id_fn(self.ptr, BUFFER_SIZE as UINT, buffer.as_mut_ptr()) };
        if hr == 0 {
            let len = buffer.iter().position(|&c| c == 0).unwrap_or(BUFFER_SIZE);
            let id_slice = &buffer[..len];
            Ok(OsString::from_wide(id_slice).to_string_lossy().into_owned())
        } else {
            Err(hr)
        }
    }

    pub fn kinect_capabilities(&self) -> Result<DWORD, HRESULT> {
        if self.ptr.is_null() {
            return Err(E_POINTER);
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_KinectCapabilities.ok_or(E_FAIL)?;
        let mut capabilities_val: DWORD = 0;
        let hr = unsafe { get_fn(self.ptr, &mut capabilities_val) };
        if hr == 0 {
            Ok(capabilities_val)
        } else {
            Err(hr)
        }
    }
}

impl Drop for KinectSensor {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("KinectSensor VTable pointer is null in Drop");
                if let Some(release_fn) = vtbl.Release {
                    release_fn(self.ptr);
                } else {
                    eprintln!("KinectSensor Release function pointer is null in Drop");
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub fn get_default_kinect_sensor() -> Result<KinectSensor, HRESULT> {
    let mut kinect_sensor_ptr: *mut bindings::IKinectSensor = ptr::null_mut();
    let hr = unsafe { GetDefaultKinectSensor(&mut kinect_sensor_ptr) };
    if hr == 0 {
        if kinect_sensor_ptr.is_null() {
            Err(E_POINTER)
        } else {
            Ok(KinectSensor::new(kinect_sensor_ptr))
        }
    } else {
        Err(hr)
    }
}
