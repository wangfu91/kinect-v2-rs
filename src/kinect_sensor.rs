use windows_sys::{
    Win32::Foundation::{BOOLEAN, E_FAIL, E_POINTER},
    core::HRESULT,
};

use crate::bindings::{
    DWORD,
    IAudioSource,
    IBodyFrameSource,
    IBodyIndexFrameSource,
    IColorFrameSource,
    ICoordinateMapper,
    IDepthFrameSource,
    IInfraredFrameSource,
    IIsAvailableChangedEventArgs, // Added WAITABLE_HANDLE and IIsAvailableChangedEventArgs
    IKinectSensor,
    ILongExposureInfraredFrameSource,
    IMultiSourceFrameReader,
    UINT,
    WAITABLE_HANDLE,
    WCHAR,
};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

impl IKinectSensor {
    pub fn subscribe_is_available_changed(
        &mut self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(subscribe_fn) = vtbl.SubscribeIsAvailableChanged {
                let hr = unsafe { subscribe_fn(kinect_sensor_ptr, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn unsubscribe_is_available_changed(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(unsubscribe_fn) = vtbl.UnsubscribeIsAvailableChanged {
                let hr = unsafe { unsubscribe_fn(kinect_sensor_ptr, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_is_available_changed_event_data(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<*mut IIsAvailableChangedEventArgs, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_event_data_fn) = vtbl.GetIsAvailableChangedEventData {
                let mut event_data_ptr: *mut IIsAvailableChangedEventArgs = ptr::null_mut();
                let hr = unsafe {
                    get_event_data_fn(kinect_sensor_ptr, waitable_handle, &mut event_data_ptr)
                };
                if hr == 0 { Ok(event_data_ptr) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn open(&mut self) -> Result<(), HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(open_fn) = vtbl.Open {
                let hr = unsafe { open_fn(kinect_sensor_ptr) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn close(&mut self) -> Result<(), HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(close_fn) = vtbl.Close {
                let hr = unsafe { close_fn(kinect_sensor_ptr) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn is_open(&mut self) -> Result<bool, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_is_open_fn) = vtbl.get_IsOpen {
                let mut is_open_val: BOOLEAN = 0;
                let hr = unsafe { get_is_open_fn(kinect_sensor_ptr, &mut is_open_val) };
                if hr == 0 {
                    Ok(is_open_val != 0)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn is_available(&mut self) -> Result<bool, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_is_available_fn) = vtbl.get_IsAvailable {
                let mut is_available: BOOLEAN = 0;
                let hr = unsafe { get_is_available_fn(kinect_sensor_ptr, &mut is_available) };
                if hr == 0 {
                    Ok(is_available != 0)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn color_frame_source(&mut self) -> Result<*mut IColorFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_ColorFrameSource {
                let mut frame_source_ptr: *mut IColorFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn depth_frame_source(&mut self) -> Result<*mut IDepthFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_DepthFrameSource {
                let mut frame_source_ptr: *mut IDepthFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn body_frame_source(&mut self) -> Result<*mut IBodyFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_BodyFrameSource {
                let mut frame_source_ptr: *mut IBodyFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn body_index_frame_source(&mut self) -> Result<*mut IBodyIndexFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_BodyIndexFrameSource {
                let mut frame_source_ptr: *mut IBodyIndexFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn infrared_frame_source(&mut self) -> Result<*mut IInfraredFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_InfraredFrameSource {
                let mut frame_source_ptr: *mut IInfraredFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn long_exposure_infrared_frame_source(
        &mut self,
    ) -> Result<*mut ILongExposureInfraredFrameSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_LongExposureInfraredFrameSource {
                let mut frame_source_ptr: *mut ILongExposureInfraredFrameSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut frame_source_ptr) };
                if hr == 0 {
                    Ok(frame_source_ptr)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn audio_source(&mut self) -> Result<*mut IAudioSource, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_AudioSource {
                let mut source_ptr: *mut IAudioSource = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut source_ptr) };
                if hr == 0 { Ok(source_ptr) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn open_multi_source_frame_reader(
        &mut self,
        enabled_frame_source_types: DWORD,
    ) -> Result<*mut IMultiSourceFrameReader, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(open_fn) = vtbl.OpenMultiSourceFrameReader {
                let mut reader_ptr: *mut IMultiSourceFrameReader = ptr::null_mut();
                let hr = unsafe {
                    open_fn(
                        kinect_sensor_ptr,
                        enabled_frame_source_types,
                        &mut reader_ptr,
                    )
                };
                if hr == 0 { Ok(reader_ptr) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn coordinate_mapper(&mut self) -> Result<*mut ICoordinateMapper, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_CoordinateMapper {
                let mut mapper_ptr: *mut ICoordinateMapper = ptr::null_mut();
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut mapper_ptr) };
                if hr == 0 { Ok(mapper_ptr) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn unique_kinect_id(&mut self) -> Result<String, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        const BUFFER_SIZE: usize = 256; // Recommended buffer size for UniqueKinectId
        let mut buffer: [WCHAR; BUFFER_SIZE] = [0; BUFFER_SIZE];

        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_id_fn) = vtbl.get_UniqueKinectId {
                let hr = unsafe {
                    get_id_fn(kinect_sensor_ptr, BUFFER_SIZE as UINT, buffer.as_mut_ptr())
                };
                if hr == 0 {
                    let len = buffer.iter().position(|&c| c == 0).unwrap_or(BUFFER_SIZE);
                    let id_slice = &buffer[..len];
                    Ok(OsString::from_wide(id_slice).to_string_lossy().into_owned())
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn kinect_capabilities(&mut self) -> Result<DWORD, HRESULT> {
        let kinect_sensor_ptr = self as *mut Self;
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_fn) = vtbl.get_KinectCapabilities {
                let mut capabilities_val: DWORD = 0;
                let hr = unsafe { get_fn(kinect_sensor_ptr, &mut capabilities_val) };
                if hr == 0 {
                    Ok(capabilities_val)
                } else {
                    Err(hr)
                }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}
