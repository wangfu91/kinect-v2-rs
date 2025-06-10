use crate::bindings::{
    AudioBeamMode, BOOLEAN, IAudioBeam, IAudioBeamFrameReader, IAudioBeamList,
    IFrameCapturedEventArgs, IKinectSensor, KinectAudioCalibrationState, TIMESPAN, WAITABLE_HANDLE,
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

pub struct AudioBeam {
    ptr: *mut IAudioBeam,
}

impl AudioBeam {
    pub(crate) fn new(ptr: *mut IAudioBeam) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_audio_source(&self) -> Result<AudioSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_source_fn = vtbl.get_AudioSource.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_source_ptr: *mut IAudioSource = ptr::null_mut();
        let hr = unsafe { get_source_fn(self.ptr, &mut audio_source_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if audio_source_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(AudioSource::new(audio_source_ptr))
            }
        }
    }

    pub fn get_audio_beam_mode(&self) -> Result<AudioBeamMode, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_mode_fn = vtbl.get_AudioBeamMode.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_mode: AudioBeamMode = AudioBeamMode::AudioBeamMode_Automatic; // Default
        let hr = unsafe { get_mode_fn(self.ptr, &mut audio_beam_mode) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(audio_beam_mode)
        }
    }

    pub fn put_audio_beam_mode(&self, audio_beam_mode: AudioBeamMode) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let put_mode_fn = vtbl.put_AudioBeamMode.ok_or_else(|| Error::from(E_FAIL))?;
        let hr = unsafe { put_mode_fn(self.ptr, audio_beam_mode) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn get_beam_angle(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_angle_fn = vtbl.get_BeamAngle.ok_or_else(|| Error::from(E_FAIL))?;
        let mut beam_angle: f32 = 0.0;
        let hr = unsafe { get_angle_fn(self.ptr, &mut beam_angle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(beam_angle)
        }
    }

    pub fn put_beam_angle(&self, beam_angle: f32) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let put_angle_fn = vtbl.put_BeamAngle.ok_or_else(|| Error::from(E_FAIL))?;
        let hr = unsafe { put_angle_fn(self.ptr, beam_angle) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn get_beam_angle_confidence(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_confidence_fn = vtbl
            .get_BeamAngleConfidence
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut beam_angle_confidence: f32 = 0.0;
        let hr = unsafe { get_confidence_fn(self.ptr, &mut beam_angle_confidence) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(beam_angle_confidence)
        }
    }

    pub fn open_input_stream(&self) -> Result<*mut windows::Win32::System::Com::IStream, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_stream_fn = vtbl.OpenInputStream.ok_or_else(|| Error::from(E_FAIL))?;
        let mut stream: *mut windows::Win32::System::Com::IStream = ptr::null_mut();
        let hr = unsafe { open_stream_fn(self.ptr, &mut stream) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(stream)
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

impl Drop for AudioBeam {
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

pub struct AudioBeamList {
    ptr: *mut IAudioBeamList,
}

impl AudioBeamList {
    pub(crate) fn new(ptr: *mut IAudioBeamList) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_beam_count(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_count_fn = vtbl.get_BeamCount.ok_or_else(|| Error::from(E_FAIL))?;
        let mut count: UINT = 0;
        let hr = unsafe { get_count_fn(self.ptr, &mut count) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(count)
        }
    }

    pub fn open_audio_beam(&self, index: UINT) -> Result<AudioBeam, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_beam_fn = vtbl.OpenAudioBeam.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_ptr: *mut IAudioBeam = ptr::null_mut();
        let hr = unsafe { open_beam_fn(self.ptr, index, &mut audio_beam_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            if audio_beam_ptr.is_null() {
                Err(Error::from(E_POINTER))
            } else {
                Ok(AudioBeam::new(audio_beam_ptr))
            }
        }
    }
}

impl Drop for AudioBeamList {
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
