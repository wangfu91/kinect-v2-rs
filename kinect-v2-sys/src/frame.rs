use crate::bindings::{
    FrameCapturedStatus, FrameSourceTypes, IFrameCapturedEventArgs, IFrameDescription, TIMESPAN,
};
use std::ptr;
use windows::{
    Win32::Foundation::{E_FAIL, E_POINTER},
    core::Error,
};

#[derive(Debug, Clone)]
pub struct FrameDescription {
    ptr: *mut IFrameDescription,
}

impl FrameDescription {
    pub(crate) fn new(ptr: *mut IFrameDescription) -> Self {
        assert!(!ptr.is_null(), "FrameDescription pointer cannot be null");
        Self { ptr }
    }

    pub fn get_width(&self) -> Result<i32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Width.ok_or(E_FAIL)?;
        let mut width: i32 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut width) };
        if hr.is_ok() {
            Ok(width)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_height(&self) -> Result<i32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Height.ok_or(E_FAIL)?;
        let mut height: i32 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut height) };
        if hr.is_ok() {
            Ok(height)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_horizontal_field_of_view(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HorizontalFieldOfView.ok_or(E_FAIL)?;
        let mut fov: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut fov) };
        if hr.is_ok() {
            Ok(fov)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_vertical_field_of_view(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_VerticalFieldOfView.ok_or(E_FAIL)?;
        let mut fov: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut fov) };
        if hr.is_ok() {
            Ok(fov)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_diagonal_field_of_view(&self) -> Result<f32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_DiagonalFieldOfView.ok_or(E_FAIL)?;
        let mut fov: f32 = 0.0;
        let hr = unsafe { get_fn(self.ptr, &mut fov) };
        if hr.is_ok() {
            Ok(fov)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_length_in_pixels(&self) -> Result<u32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_LengthInPixels.ok_or(E_FAIL)?;
        let mut length: u32 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut length) };
        if hr.is_ok() {
            Ok(length)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_bytes_per_pixel(&self) -> Result<u32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BytesPerPixel.ok_or(E_FAIL)?;
        let mut bpp: u32 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut bpp) };
        if hr.is_ok() {
            Ok(bpp)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for FrameDescription {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct FrameCapturedEventArgs {
    ptr: *mut IFrameCapturedEventArgs,
}

impl FrameCapturedEventArgs {
    pub(crate) fn new(ptr: *mut IFrameCapturedEventArgs) -> Self {
        assert!(
            !ptr.is_null(),
            "FrameCapturedEventArgs pointer cannot be null"
        );
        Self { ptr }
    }

    pub fn get_frame_type(&self) -> Result<FrameSourceTypes, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameType.ok_or(E_FAIL)?;
        let mut frame_type: FrameSourceTypes = FrameSourceTypes::None;
        let hr = unsafe { get_fn(self.ptr, &mut frame_type) };
        if hr.is_ok() {
            Ok(frame_type)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_status(&self) -> Result<FrameCapturedStatus, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameStatus.ok_or(E_FAIL)?;
        let mut status: FrameCapturedStatus = FrameCapturedStatus::Unknown;
        let hr = unsafe { get_fn(self.ptr, &mut status) };
        if hr.is_ok() {
            Ok(status)
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

impl Drop for FrameCapturedEventArgs {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let vtbl = (*self.ptr)
                    .lpVtbl
                    .as_ref()
                    .expect("VTable pointer is null in Drop");
                let release_fn = vtbl
                    .Release
                    .expect("Release function pointer is null in Drop");
                release_fn(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
