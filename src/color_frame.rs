use crate::bindings::{
    ColorImageFormat, FrameCapturedStatus, FrameSourceTypes, IColorCameraSettings, IColorFrame,
    IColorFrameArrivedEventArgs, IColorFrameReader, IColorFrameReference, IColorFrameSource,
    IFrameCapturedEventArgs, IFrameDescription, IKinectSensor, TIMESPAN, WAITABLE_HANDLE,
};
use std::ptr;
use windows_sys::{
    Win32::Foundation::{BOOLEAN, E_FAIL, E_POINTER},
    core::HRESULT,
};

impl IFrameDescription {
    pub fn get_width(&mut self) -> Result<i32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_width_fn) = vtbl.get_Width {
                let mut width_val: i32 = 0;
                let hr = unsafe { get_width_fn(self, &mut width_val) };
                if hr == 0 { Ok(width_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_height(&mut self) -> Result<i32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_height_fn) = vtbl.get_Height {
                let mut height_val: i32 = 0;
                let hr = unsafe { get_height_fn(self, &mut height_val) };
                if hr == 0 { Ok(height_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_horizontal_field_of_view(&mut self) -> Result<f32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_hfov_fn) = vtbl.get_HorizontalFieldOfView {
                let mut hfov_val: f32 = 0.0;
                let hr = unsafe { get_hfov_fn(self, &mut hfov_val) };
                if hr == 0 { Ok(hfov_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_vertical_field_of_view(&mut self) -> Result<f32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_vfov_fn) = vtbl.get_VerticalFieldOfView {
                let mut vfov_val: f32 = 0.0;
                let hr = unsafe { get_vfov_fn(self, &mut vfov_val) };
                if hr == 0 { Ok(vfov_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_diagonal_field_of_view(&mut self) -> Result<f32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_dfov_fn) = vtbl.get_DiagonalFieldOfView {
                let mut dfov_val: f32 = 0.0;
                let hr = unsafe { get_dfov_fn(self, &mut dfov_val) };
                if hr == 0 { Ok(dfov_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_length_in_pixels(&mut self) -> Result<u32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_length_fn) = vtbl.get_LengthInPixels {
                let mut length_val: u32 = 0;
                let hr = unsafe { get_length_fn(self, &mut length_val) };
                if hr == 0 { Ok(length_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_bytes_per_pixel(&mut self) -> Result<u32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_bpp_fn) = vtbl.get_BytesPerPixel {
                let mut bpp_val: u32 = 0;
                let hr = unsafe { get_bpp_fn(self, &mut bpp_val) };
                if hr == 0 { Ok(bpp_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IFrameDescription {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!("Failed to release IFrameDescription: HRESULT = {:#X}", hr);
        }
    }
}

impl IFrameCapturedEventArgs {
    pub fn get_frame_type(&mut self) -> Result<FrameSourceTypes, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_frame_type_fn) = vtbl.get_FrameType {
                let mut frame_type_val: FrameSourceTypes = FrameSourceTypes::FrameSourceTypes_None;
                let hr = unsafe { get_frame_type_fn(self, &mut frame_type_val) };
                if hr == 0 { Ok(frame_type_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_frame_status(&mut self) -> Result<FrameCapturedStatus, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_frame_status_fn) = vtbl.get_FrameStatus {
                let mut frame_status_val: FrameCapturedStatus =
                    FrameCapturedStatus::FrameCapturedStatus_Unknown;
                let hr = unsafe { get_frame_status_fn(self, &mut frame_status_val) };
                if hr == 0 {
                    Ok(frame_status_val)
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

    pub fn get_relative_time(&mut self) -> Result<TIMESPAN, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_relative_time_fn) = vtbl.get_RelativeTime {
                let mut relative_time_val: TIMESPAN = 0;
                let hr = unsafe { get_relative_time_fn(self, &mut relative_time_val) };
                if hr == 0 {
                    Ok(relative_time_val)
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

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IFrameCapturedEventArgs {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!(
                "Failed to release IFrameCapturedEventArgs: HRESULT = {:#X}",
                hr
            );
        }
    }
}

impl IColorFrame {
    pub fn get_frame_description(&mut self) -> Result<IFrameDescription, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_desc_fn) = vtbl.get_FrameDescription {
                let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
                let hr = unsafe { get_desc_fn(self, &mut desc_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(desc_ptr) })
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

    pub fn get_raw_color_image_format(&mut self) -> Result<ColorImageFormat, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_format_fn) = vtbl.get_RawColorImageFormat {
                let mut format_val: ColorImageFormat = ColorImageFormat::ColorImageFormat_None;
                let hr = unsafe { get_format_fn(self, &mut format_val) };
                if hr == 0 { Ok(format_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn copy_raw_frame_data_to_array(
        &mut self,
        capacity: u32,
        frame_data: *mut u8,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(copy_fn) = vtbl.CopyRawFrameDataToArray {
                let hr = unsafe { copy_fn(self, capacity, frame_data) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn access_raw_underlying_buffer(
        &mut self,
        capacity: &mut u32,
        buffer: &mut *mut u8,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(access_fn) = vtbl.AccessRawUnderlyingBuffer {
                let hr = unsafe { access_fn(self, capacity, buffer) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn copy_converted_frame_data_to_array(
        &mut self,
        capacity: u32,
        frame_data: *mut u8,
        color_format: ColorImageFormat,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(copy_fn) = vtbl.CopyConvertedFrameDataToArray {
                let hr = unsafe { copy_fn(self, capacity, frame_data, color_format) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn create_frame_description(
        &mut self,
        format: ColorImageFormat,
    ) -> Result<IFrameDescription, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(create_desc_fn) = vtbl.CreateFrameDescription {
                let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
                let hr = unsafe { create_desc_fn(self, format, &mut desc_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(desc_ptr) })
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

    pub fn get_color_camera_settings(&mut self) -> Result<IColorCameraSettings, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_settings_fn) = vtbl.get_ColorCameraSettings {
                let mut settings_ptr: *mut IColorCameraSettings = ptr::null_mut();
                let hr = unsafe { get_settings_fn(self, &mut settings_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(settings_ptr) })
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

    pub fn get_relative_time(&mut self) -> Result<TIMESPAN, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_time_fn) = vtbl.get_RelativeTime {
                let mut time_val: TIMESPAN = 0;
                let hr = unsafe { get_time_fn(self, &mut time_val) };
                if hr == 0 { Ok(time_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_color_frame_source(&mut self) -> Result<IColorFrameSource, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_source_fn) = vtbl.get_ColorFrameSource {
                let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
                let hr = unsafe { get_source_fn(self, &mut source_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(source_ptr) })
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

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorFrame {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!("Failed to release IColorFrame: HRESULT = {:#X}", hr);
        }
    }
}

impl IColorFrameArrivedEventArgs {
    pub fn get_frame_reference(&mut self) -> Result<IColorFrameReference, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_ref_fn) = vtbl.get_FrameReference {
                let mut ref_ptr: *mut IColorFrameReference = ptr::null_mut();
                let hr = unsafe { get_ref_fn(self, &mut ref_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(ref_ptr) })
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

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorFrameArrivedEventArgs {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!(
                "Failed to release IColorFrameArrivedEventArgs: HRESULT = {:#X}",
                hr
            );
        }
    }
}

impl IColorFrameReference {
    pub fn acquire_frame(&mut self) -> Result<IColorFrame, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(acquire_frame_fn) = vtbl.AcquireFrame {
                let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
                let hr = unsafe { acquire_frame_fn(self, &mut frame_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(frame_ptr) })
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

    pub fn get_relative_time(&mut self) -> Result<TIMESPAN, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_time_fn) = vtbl.get_RelativeTime {
                let mut time_val: TIMESPAN = 0;
                let hr = unsafe { get_time_fn(self, &mut time_val) };
                if hr == 0 { Ok(time_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorFrameReference {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!(
                "Failed to release IColorFrameReference: HRESULT = {:#X}",
                hr
            );
        }
    }
}

impl IColorCameraSettings {
    pub fn get_exposure_time(&mut self) -> Result<TIMESPAN, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_time_fn) = vtbl.get_ExposureTime {
                let mut time_val: TIMESPAN = 0;
                let hr = unsafe { get_time_fn(self, &mut time_val) };
                if hr == 0 { Ok(time_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_frame_interval(&mut self) -> Result<TIMESPAN, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_interval_fn) = vtbl.get_FrameInterval {
                let mut interval_val: TIMESPAN = 0;
                let hr = unsafe { get_interval_fn(self, &mut interval_val) };
                if hr == 0 { Ok(interval_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_gain(&mut self) -> Result<f32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_gain_fn) = vtbl.get_Gain {
                let mut gain_val: f32 = 0.0;
                let hr = unsafe { get_gain_fn(self, &mut gain_val) };
                if hr == 0 { Ok(gain_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_gamma(&mut self) -> Result<f32, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_gamma_fn) = vtbl.get_Gamma {
                let mut gamma_val: f32 = 0.0;
                let hr = unsafe { get_gamma_fn(self, &mut gamma_val) };
                if hr == 0 { Ok(gamma_val) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorCameraSettings {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!(
                "Failed to release IColorCameraSettings: HRESULT = {:#X}",
                hr
            );
        }
    }
}

impl IColorFrameReader {
    pub fn subscribe_frame_arrived(
        &mut self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(subscribe_fn) = vtbl.SubscribeFrameArrived {
                let hr = unsafe { subscribe_fn(self, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn unsubscribe_frame_arrived(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(unsubscribe_fn) = vtbl.UnsubscribeFrameArrived {
                let hr = unsafe { unsubscribe_fn(self, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_frame_arrived_event_data(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<IColorFrameArrivedEventArgs, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_data_fn) = vtbl.GetFrameArrivedEventData {
                let mut event_data_ptr: *mut IColorFrameArrivedEventArgs = ptr::null_mut();
                let hr = unsafe { get_data_fn(self, waitable_handle, &mut event_data_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(event_data_ptr) })
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

    pub fn acquire_latest_frame(&mut self) -> Result<IColorFrame, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(acquire_fn) = vtbl.AcquireLatestFrame {
                let mut frame_ptr: *mut IColorFrame = ptr::null_mut();
                let hr = unsafe { acquire_fn(self, &mut frame_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(frame_ptr) })
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

    pub fn get_is_paused(&mut self) -> Result<bool, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_paused_fn) = vtbl.get_IsPaused {
                let mut paused_val: BOOLEAN = 0;
                let hr = unsafe { get_paused_fn(self, &mut paused_val) };
                if hr == 0 {
                    Ok(paused_val != 0)
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

    pub fn put_is_paused(&mut self, is_paused: bool) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(put_paused_fn) = vtbl.put_IsPaused {
                let hr = unsafe { put_paused_fn(self, if is_paused { 1 } else { 0 }) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_color_frame_source(&mut self) -> Result<IColorFrameSource, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_source_fn) = vtbl.get_ColorFrameSource {
                let mut source_ptr: *mut IColorFrameSource = ptr::null_mut();
                let hr = unsafe { get_source_fn(self, &mut source_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(source_ptr) })
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

    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorFrameReader {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!("Failed to release IColorFrameReader: HRESULT = {:#X}", hr);
        }
    }
}

impl IColorFrameSource {
    pub fn subscribe_frame_captured(
        &mut self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(subscribe_fn) = vtbl.SubscribeFrameCaptured {
                let hr = unsafe { subscribe_fn(self, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn unsubscribe_frame_captured(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(unsubscribe_fn) = vtbl.UnsubscribeFrameCaptured {
                let hr = unsafe { unsubscribe_fn(self, waitable_handle) };
                if hr == 0 { Ok(()) } else { Err(hr) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }

    pub fn get_frame_captured_event_data(
        &mut self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<IFrameCapturedEventArgs, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_data_fn) = vtbl.GetFrameCapturedEventData {
                let mut event_data_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
                let hr = unsafe { get_data_fn(self, waitable_handle, &mut event_data_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(event_data_ptr) })
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

    pub fn get_is_active(&mut self) -> Result<bool, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_is_active_fn) = vtbl.get_IsActive {
                let mut is_active_val: BOOLEAN = 0;
                let hr = unsafe { get_is_active_fn(self, &mut is_active_val) };
                if hr == 0 {
                    Ok(is_active_val != 0)
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

    pub fn open_reader(&mut self) -> Result<IColorFrameReader, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(open_reader_fn) = vtbl.OpenReader {
                let mut reader_ptr: *mut IColorFrameReader = ptr::null_mut();
                let hr = unsafe { open_reader_fn(self, &mut reader_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(reader_ptr) })
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

    pub fn create_frame_description(
        &mut self,
        format: ColorImageFormat,
    ) -> Result<IFrameDescription, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(create_desc_fn) = vtbl.CreateFrameDescription {
                let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
                let hr = unsafe { create_desc_fn(self, format, &mut desc_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(desc_ptr) })
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

    pub fn get_frame_description(&mut self) -> Result<IFrameDescription, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_desc_fn) = vtbl.get_FrameDescription {
                let mut desc_ptr: *mut IFrameDescription = ptr::null_mut();
                let hr = unsafe { get_desc_fn(self, &mut desc_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(desc_ptr) })
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

    pub fn get_kinect_sensor(&mut self) -> Result<IKinectSensor, HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(get_sensor_fn) = vtbl.get_KinectSensor {
                let mut sensor_ptr: *mut IKinectSensor = ptr::null_mut();
                let hr = unsafe { get_sensor_fn(self, &mut sensor_ptr) };
                if hr == 0 {
                    Ok(unsafe { ptr::read(sensor_ptr) })
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
    pub fn release(&mut self) -> Result<(), HRESULT> {
        if let Some(vtbl) = unsafe { self.lpVtbl.as_mut() } {
            if let Some(release_fn) = vtbl.Release {
                let hr = unsafe { release_fn(self) };
                if hr == 0 { Ok(()) } else { Err(hr as i32) }
            } else {
                Err(E_FAIL)
            }
        } else {
            Err(E_POINTER)
        }
    }
}

impl Drop for IColorFrameSource {
    fn drop(&mut self) {
        if let Err(hr) = self.release() {
            eprintln!("Failed to release IColorFrameSource: HRESULT = {:#X}", hr);
        }
    }
}
