use crate::bindings::{
    BOOLEAN, IAudioBeamFrameReader, IAudioBeamList, IFrameCapturedEventArgs, IKinectSensor,
    KinectAudioCalibrationState, TIMESPAN, WAITABLE_HANDLE,
};
use crate::bindings::{IAudioSource, UINT};
use std::ptr;
use windows::Win32::Foundation::{E_FAIL, E_POINTER, HANDLE};
use windows::core::Error;

pub struct AudioSource {
    ptr: *mut IAudioSource,
}
impl AudioSource {
    pub(crate) fn new(ptr: *mut IAudioSource) -> Self {
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
        let mut waitable_handle: WAITABLE_HANDLE = HANDLE::default();
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
    ) -> Result<*mut IFrameCapturedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_data_fn = vtbl
            .GetFrameCapturedEventData
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut event_data: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(event_data)
        }
    }

    pub fn get_kinect_sensor(&self) -> Result<*mut IKinectSensor, Error> {
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
            Ok(sensor)
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

    pub fn get_sub_frame_length_in_bytes(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_length_fn = vtbl
            .get_SubFrameLengthInBytes
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut length: UINT = 0;
        let hr = unsafe { get_length_fn(self.ptr, &mut length) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(length)
        }
    }

    pub fn get_sub_frame_duration(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_duration_fn = vtbl
            .get_SubFrameDuration
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut duration: TIMESPAN = 0;
        let hr = unsafe { get_duration_fn(self.ptr, &mut duration) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(duration)
        }
    }

    pub fn get_max_sub_frame_count(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_count_fn = vtbl
            .get_MaxSubFrameCount
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut count: UINT = 0;
        let hr = unsafe { get_count_fn(self.ptr, &mut count) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(count)
        }
    }

    pub fn open_reader(&self) -> Result<*mut IAudioBeamFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_reader_fn = vtbl.OpenReader.ok_or_else(|| Error::from(E_FAIL))?;
        let mut reader: *mut IAudioBeamFrameReader = ptr::null_mut();
        let hr = unsafe { open_reader_fn(self.ptr, &mut reader) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(reader)
        }
    }

    pub fn get_audio_beams(&self) -> Result<*mut IAudioBeamList, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_beams_fn = vtbl.get_AudioBeams.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_list: *mut IAudioBeamList = ptr::null_mut();
        let hr = unsafe { get_beams_fn(self.ptr, &mut audio_beam_list) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(audio_beam_list)
        }
    }

    pub fn get_audio_calibration_state(&self) -> Result<KinectAudioCalibrationState, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_state_fn = vtbl
            .get_AudioCalibrationState
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_calibration_state: KinectAudioCalibrationState =
            KinectAudioCalibrationState::KinectAudioCalibrationState_Unknown;
        let hr = unsafe { get_state_fn(self.ptr, &mut audio_calibration_state) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(audio_calibration_state)
        }
    }
}

impl Drop for AudioSource {
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
