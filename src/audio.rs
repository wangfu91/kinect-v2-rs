use crate::bindings::{
    AudioBeamMode, BOOLEAN, BYTE, IAudioBeam, IAudioBeamFrame, IAudioBeamFrameArrivedEventArgs,
    IAudioBeamFrameList, IAudioBeamFrameReader, IAudioBeamFrameReference, IAudioBeamList,
    IAudioBeamSubFrame, IAudioBodyCorrelation, IFrameCapturedEventArgs, IKinectSensor,
    KinectAudioCalibrationState, TIMESPAN, UINT64, WAITABLE_HANDLE,
};
use crate::bindings::{IAudioSource, UINT};
use crate::frame::FrameCapturedEventArgs;
use crate::kinect::KinectSensor;
use std::ptr;
use windows::Win32::Foundation::{E_FAIL, E_POINTER, HANDLE};
use windows::Win32::System::Com::IStream;
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

    pub fn open_reader(&self) -> Result<AudioBeamFrameReader, Error> {
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
            Ok(AudioBeamFrameReader::new(reader))
        }
    }

    pub fn get_audio_beams(&self) -> Result<AudioBeamList, Error> {
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
            Ok(AudioBeamList::new(audio_beam_list))
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
        } else if audio_source_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioSource::new(audio_source_ptr))
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

    pub fn open_input_stream(&self) -> Result<*mut IStream, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_stream_fn = vtbl.OpenInputStream.ok_or_else(|| Error::from(E_FAIL))?;
        let mut stream: *mut IStream = ptr::null_mut();
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
        } else if audio_beam_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeam::new(audio_beam_ptr))
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

pub struct AudioBeamFrameList {
    ptr: *mut IAudioBeamFrameList,
}

impl AudioBeamFrameList {
    pub(crate) fn new(ptr: *mut IAudioBeamFrameList) -> Self {
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

    pub fn open_audio_beam_frame(&self, index: UINT) -> Result<AudioBeamFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let open_frame_fn = vtbl.OpenAudioBeamFrame.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_frame_ptr: *mut IAudioBeamFrame = ptr::null_mut();
        let hr = unsafe { open_frame_fn(self.ptr, index, &mut audio_beam_frame_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_frame_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeamFrame::new(audio_beam_frame_ptr))
        }
    }
}

impl Drop for AudioBeamFrameList {
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

pub struct AudioBeamFrame {
    ptr: *mut IAudioBeamFrame,
}

impl AudioBeamFrame {
    pub(crate) fn new(ptr: *mut IAudioBeamFrame) -> Self {
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
        } else if audio_source_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioSource::new(audio_source_ptr))
        }
    }

    pub fn get_duration(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_duration_fn = vtbl.get_Duration.ok_or_else(|| Error::from(E_FAIL))?;
        let mut duration: TIMESPAN = 0;
        let hr = unsafe { get_duration_fn(self.ptr, &mut duration) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(duration)
        }
    }

    pub fn get_audio_beam(&self) -> Result<AudioBeam, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_beam_fn = vtbl.get_AudioBeam.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_ptr: *mut IAudioBeam = ptr::null_mut();
        let hr = unsafe { get_beam_fn(self.ptr, &mut audio_beam_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeam::new(audio_beam_ptr))
        }
    }

    pub fn get_sub_frame_count(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_count_fn = vtbl.get_SubFrameCount.ok_or_else(|| Error::from(E_FAIL))?;
        let mut count: UINT = 0;
        let hr = unsafe { get_count_fn(self.ptr, &mut count) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(count)
        }
    }

    pub fn get_sub_frame(&self, sub_frame_index: UINT) -> Result<AudioBeamSubFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_sub_frame_fn = vtbl.GetSubFrame.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_sub_frame_ptr: *mut IAudioBeamSubFrame = ptr::null_mut();
        let hr =
            unsafe { get_sub_frame_fn(self.ptr, sub_frame_index, &mut audio_beam_sub_frame_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_sub_frame_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeamSubFrame::new(audio_beam_sub_frame_ptr))
        }
    }

    pub fn get_relative_time_start(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_time_fn = vtbl
            .get_RelativeTimeStart
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut relative_time: TIMESPAN = 0;
        let hr = unsafe { get_time_fn(self.ptr, &mut relative_time) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(relative_time)
        }
    }
}

impl Drop for AudioBeamFrame {
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

pub struct AudioBeamSubFrame {
    ptr: *mut IAudioBeamSubFrame,
}

impl AudioBeamSubFrame {
    pub(crate) fn new(ptr: *mut IAudioBeamSubFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_length_in_bytes(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_length_fn = vtbl
            .get_FrameLengthInBytes
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut length: UINT = 0;
        let hr = unsafe { get_length_fn(self.ptr, &mut length) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(length)
        }
    }

    pub fn get_duration(&self) -> Result<TIMESPAN, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_duration_fn = vtbl.get_Duration.ok_or_else(|| Error::from(E_FAIL))?;
        let mut duration: TIMESPAN = 0;
        let hr = unsafe { get_duration_fn(self.ptr, &mut duration) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(duration)
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

    pub fn get_audio_body_correlation_count(&self) -> Result<UINT, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_count_fn = vtbl
            .get_AudioBodyCorrelationCount
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut count: UINT = 0;
        let hr = unsafe { get_count_fn(self.ptr, &mut count) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(count)
        }
    }

    pub fn get_audio_body_correlation(&self, index: UINT) -> Result<AudioBodyCorrelation, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_correlation_fn = vtbl
            .GetAudioBodyCorrelation
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_body_correlation: *mut IAudioBodyCorrelation = ptr::null_mut();
        let hr = unsafe { get_correlation_fn(self.ptr, index, &mut audio_body_correlation) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(AudioBodyCorrelation::new(audio_body_correlation))
        }
    }

    pub fn copy_frame_data_to_array(&self, buffer: &mut [u8]) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let copy_data_fn = vtbl
            .CopyFrameDataToArray
            .ok_or_else(|| Error::from(E_FAIL))?;

        let capacity = buffer.len() as UINT;
        let buffer_ptr = buffer.as_mut_ptr() as *mut BYTE;

        let hr = unsafe { copy_data_fn(self.ptr, capacity, buffer_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(())
        }
    }

    pub fn access_underlying_buffer(&self) -> Result<&[u8], Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let access_buffer_fn = vtbl
            .AccessUnderlyingBuffer
            .ok_or_else(|| Error::from(E_FAIL))?;

        let mut capacity: UINT = 0;
        let mut buffer: *mut BYTE = ptr::null_mut();
        let hr = unsafe { access_buffer_fn(self.ptr, &mut capacity, &mut buffer) };

        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if buffer.is_null() || capacity == 0 {
            Err(Error::from(E_POINTER))
        } else {
            // Create a safe slice from the raw pointer
            let slice =
                unsafe { std::slice::from_raw_parts(buffer as *const u8, capacity as usize) };
            Ok(slice)
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

impl Drop for AudioBeamSubFrame {
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

pub struct AudioBodyCorrelation {
    ptr: *mut IAudioBodyCorrelation,
}

impl AudioBodyCorrelation {
    pub(crate) fn new(ptr: *mut IAudioBodyCorrelation) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_body_tracking_id(&self) -> Result<UINT64, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_body_id_fn = vtbl.get_BodyTrackingId.ok_or_else(|| Error::from(E_FAIL))?;
        let mut body_tracking_id: UINT64 = 0;
        let hr = unsafe { get_body_id_fn(self.ptr, &mut body_tracking_id) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(body_tracking_id)
        }
    }
}

impl Drop for AudioBodyCorrelation {
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

pub struct AudioBeamFrameReference {
    ptr: *mut IAudioBeamFrameReference,
}

impl AudioBeamFrameReference {
    pub(crate) fn new(ptr: *mut IAudioBeamFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_beam_frame(&self) -> Result<AudioBeamFrameList, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let acquire_frame_fn = vtbl.AcquireBeamFrames.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_frame_list_ptr: *mut IAudioBeamFrameList = ptr::null_mut();
        let hr = unsafe { acquire_frame_fn(self.ptr, &mut audio_beam_frame_list_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_frame_list_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeamFrameList::new(audio_beam_frame_list_ptr))
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

impl Drop for AudioBeamFrameReference {
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

pub struct AudioBeamFrameArrivedEventArgs {
    ptr: *mut IAudioBeamFrameArrivedEventArgs,
}

impl AudioBeamFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IAudioBeamFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<AudioBeamFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_frame_fn = vtbl.get_FrameReference.ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_frame_reference_ptr: *mut IAudioBeamFrameReference = ptr::null_mut();
        let hr = unsafe { get_frame_fn(self.ptr, &mut audio_beam_frame_reference_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_frame_reference_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeamFrameReference::new(audio_beam_frame_reference_ptr))
        }
    }
}

impl Drop for AudioBeamFrameArrivedEventArgs {
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

pub struct AudioBeamFrameReader {
    ptr: *mut IAudioBeamFrameReader,
}

impl AudioBeamFrameReader {
    pub(crate) fn new(ptr: *mut IAudioBeamFrameReader) -> Self {
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
        let mut waitable_handle: WAITABLE_HANDLE = HANDLE::default();
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
    ) -> Result<AudioBeamFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let get_data_fn = vtbl
            .GetFrameArrivedEventData
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut event_data: *mut IAudioBeamFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_data_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else {
            Ok(AudioBeamFrameArrivedEventArgs::new(event_data))
        }
    }

    pub fn acquire_latest_beam_frames(&self) -> Result<AudioBeamFrameList, Error> {
        if self.ptr.is_null() {
            return Err(Error::from(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or_else(|| Error::from(E_POINTER))?;
        let acquire_fn = vtbl
            .AcquireLatestBeamFrames
            .ok_or_else(|| Error::from(E_FAIL))?;
        let mut audio_beam_frame_list_ptr: *mut IAudioBeamFrameList = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut audio_beam_frame_list_ptr) };
        if hr.is_err() {
            Err(Error::from_hresult(hr))
        } else if audio_beam_frame_list_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioBeamFrameList::new(audio_beam_frame_list_ptr))
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
        } else if audio_source_ptr.is_null() {
            Err(Error::from(E_POINTER))
        } else {
            Ok(AudioSource::new(audio_source_ptr))
        }
    }
}

impl Drop for AudioBeamFrameReader {
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

    use super::*;
    use windows::Win32::{
        Foundation::{E_POINTER, WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::{Com::Urlmon::E_PENDING, Threading::WaitForSingleObject},
    };

    #[test]
    fn get_latest_audio_frame() -> anyhow::Result<()> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;
        let audio_source = kinect.audio_source()?;
        let audio_beam_frame_reader = audio_source.open_reader()?;

        let mut frame_counter = 0;
        loop {
            match audio_beam_frame_reader.acquire_latest_beam_frames() {
                Ok(frame_list) => {
                    let beam_count = frame_list.get_beam_count()?;
                    assert_eq!(beam_count, 1, "Expected only 1 audio beam");

                    let audio_beam_frame = frame_list.open_audio_beam_frame(0)?;
                    let duration = audio_beam_frame.get_duration()?;
                    let sub_frame_count = audio_beam_frame.get_sub_frame_count()?;
                    let audio_beam = audio_beam_frame.get_audio_beam()?;
                    let beam_angle = audio_beam.get_beam_angle()?;
                    let beam_angle_confidence = audio_beam.get_beam_angle_confidence()?;
                    let relative_time = audio_beam.get_relative_time()?;

                    println!(
                        "AudioFrame Duration: {}, SubFrameCount:{}, Beam Angle: {}, Confidence: {}, Relative Time: {}",
                        duration, sub_frame_count, beam_angle, beam_angle_confidence, relative_time
                    );

                    assert!(
                        sub_frame_count >= 1,
                        "Expected at least 1 sub-frame in audio beam frame"
                    );
                    assert!(
                        beam_angle >= -180.0 && beam_angle <= 180.0,
                        "Beam angle out of range"
                    );
                    assert!(
                        beam_angle_confidence >= 0.0 && beam_angle_confidence <= 1.0,
                        "Beam angle confidence out of range"
                    );
                    assert!(relative_time >= 0, "Relative time should be non-negative");

                    frame_counter += 1;
                    if frame_counter >= 10 {
                        break; // Limit to 10 frames for testing
                    }
                }
                Err(e) => {
                    if e.code() == E_PENDING.into() {
                        println!("Audio frame not ready yet, waiting...");
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
    fn subscribe_audio_frame_arrived_event() -> anyhow::Result<()> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;
        let audio_source = kinect.audio_source()?;
        let audio_frame_reader = audio_source.open_reader()?;
        let waitable_handle = audio_frame_reader.subscribe_frame_arrived()?;
        let is_active = audio_source.get_is_active()?;
        assert!(is_active, "Audio source should be active");

        let mut frame_counter = 0;
        loop {
            let wait_result =
                unsafe { WaitForSingleObject(waitable_handle, FRAME_WAIT_TIMEOUT_MS) };
            if WAIT_OBJECT_0 == wait_result {
                let event_args =
                    audio_frame_reader.get_frame_arrived_event_data(waitable_handle)?;

                let frame_reference = event_args.get_frame_reference()?;
                let frame_list = frame_reference.acquire_beam_frame()?;
                let beam_count = frame_list.get_beam_count()?;
                assert_eq!(beam_count, 1, "Expected only 1 audio beam");

                let audio_beam_frame = frame_list.open_audio_beam_frame(0)?;
                let duration = audio_beam_frame.get_duration()?;
                let sub_frame_count = audio_beam_frame.get_sub_frame_count()?;
                let audio_beam = audio_beam_frame.get_audio_beam()?;
                let beam_mode = audio_beam.get_audio_beam_mode()?;
                let beam_angle = audio_beam.get_beam_angle()?;
                let beam_angle_confidence = audio_beam.get_beam_angle_confidence()?;
                let relative_time = audio_beam.get_relative_time()?;

                println!(
                    "AudioFrame Duration ticks: {}, SubFrameCount:{}, Beam Mode: {:?}, Beam Angle: {}, Confidence: {}, Relative Time: {}",
                    duration,
                    sub_frame_count,
                    beam_mode,
                    beam_angle,
                    beam_angle_confidence,
                    relative_time
                );

                assert!(
                    sub_frame_count >= 1,
                    "Expected at least 1 sub-frame in audio beam frame"
                );
                assert!(
                    beam_angle >= -180.0 && beam_angle <= 180.0,
                    "Beam angle out of range"
                );
                assert!(
                    beam_angle_confidence >= 0.0 && beam_angle_confidence <= 1.0,
                    "Beam angle confidence out of range"
                );
                assert!(relative_time >= 0, "Relative time should be non-negative");

                for i in 0..sub_frame_count {
                    let sub_frame = audio_beam_frame.get_sub_frame(i)?;
                    let sub_frame_duration = sub_frame.get_duration()?;
                    assert!(
                        sub_frame_duration > 0,
                        "Sub-frame duration should be positive"
                    );

                    let sub_frame_bytes = sub_frame.get_frame_length_in_bytes()?;
                    assert!(
                        sub_frame_bytes > 0,
                        "Sub-frame length in bytes should be positive"
                    );

                    let sub_frame_data = sub_frame.access_underlying_buffer()?;
                    assert!(
                        !sub_frame_data.is_empty(),
                        "Sub-frame data should not be empty"
                    );
                    println!(
                        "SubFrame {}: Duration ticks: {}, Length: {}, Data Size: {} bytes",
                        i,
                        sub_frame_duration,
                        sub_frame_bytes,
                        sub_frame_data.len()
                    );
                }

                frame_counter += 1;
                if frame_counter >= 10 {
                    break; // Limit to 10 frames for testing
                }
            } else if WAIT_TIMEOUT == wait_result {
                println!("No new audio frame available, waiting...");
            } else {
                return Err(anyhow::anyhow!(
                    "WaitForSingleObject failed with result: {:?}",
                    wait_result
                ));
            }
        }

        // Unsubscribe from the event
        audio_frame_reader.unsubscribe_frame_arrived(waitable_handle)?;

        Ok(())
    }
}
