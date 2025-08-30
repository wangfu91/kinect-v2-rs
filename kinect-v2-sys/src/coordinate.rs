use crate::bindings::{
    CameraIntrinsics, CameraSpacePoint, ColorSpacePoint, DepthSpacePoint, ICoordinateMapper,
    ICoordinateMappingChangedEventArgs, PointF, UINT, UINT16, WAITABLE_HANDLE,
};
use std::ptr;
use windows::{
    Win32::Foundation::{E_FAIL, E_POINTER},
    core::Error,
};

pub struct CoordinateMapper {
    ptr: *mut ICoordinateMapper,
}

impl CoordinateMapper {
    pub(crate) fn new(ptr: *mut ICoordinateMapper) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_coordinate_mapping_changed(&self) -> Result<WAITABLE_HANDLE, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let subscribe_fn = vtbl.SubscribeCoordinateMappingChanged.ok_or(E_FAIL)?;
        let mut waitable_handle: WAITABLE_HANDLE = windows::Win32::Foundation::HANDLE::default();
        let hr = unsafe { subscribe_fn(self.ptr, &mut waitable_handle) };
        if hr.is_ok() {
            Ok(waitable_handle)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_coordinate_mapping_changed(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsubscribe_fn = vtbl.UnsubscribeCoordinateMappingChanged.ok_or(E_FAIL)?;
        let hr = unsafe { unsubscribe_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_coordinate_mapping_changed_event_data(
        &self,
        waitable_handle: WAITABLE_HANDLE,
    ) -> Result<CoordinateMappingChangedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetCoordinateMappingChangedEventData.ok_or(E_FAIL)?;
        let mut event_data: *mut ICoordinateMappingChangedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, waitable_handle, &mut event_data) };
        if hr.is_ok() {
            Ok(CoordinateMappingChangedEventArgs::new(event_data))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_camera_point_to_depth_space(
        &self,
        camera_point: CameraSpacePoint,
    ) -> Result<DepthSpacePoint, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapCameraPointToDepthSpace.ok_or(E_FAIL)?;
        let mut depth_point = DepthSpacePoint { X: 0.0, Y: 0.0 };
        let hr = unsafe { map_fn(self.ptr, camera_point, &mut depth_point) };
        if hr.is_ok() {
            Ok(depth_point)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_camera_point_to_color_space(
        &self,
        camera_point: CameraSpacePoint,
    ) -> Result<ColorSpacePoint, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapCameraPointToColorSpace.ok_or(E_FAIL)?;
        let mut color_point = ColorSpacePoint { X: 0.0, Y: 0.0 };
        let hr = unsafe { map_fn(self.ptr, camera_point, &mut color_point) };
        if hr.is_ok() {
            Ok(color_point)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_point_to_camera_space(
        &self,
        depth_point: DepthSpacePoint,
        depth: UINT16,
    ) -> Result<CameraSpacePoint, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthPointToCameraSpace.ok_or(E_FAIL)?;
        let mut camera_point = CameraSpacePoint {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
        };
        let hr = unsafe { map_fn(self.ptr, depth_point, depth, &mut camera_point) };
        if hr.is_ok() {
            Ok(camera_point)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_point_to_color_space(
        &self,
        depth_point: DepthSpacePoint,
        depth: UINT16,
    ) -> Result<ColorSpacePoint, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthPointToColorSpace.ok_or(E_FAIL)?;
        let mut color_point = ColorSpacePoint { X: 0.0, Y: 0.0 };
        let hr = unsafe { map_fn(self.ptr, depth_point, depth, &mut color_point) };
        if hr.is_ok() {
            Ok(color_point)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_camera_points_to_depth_space(
        &self,
        camera_points: &[CameraSpacePoint],
        depth_points: &mut [DepthSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if camera_points.len() != depth_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapCameraPointsToDepthSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                camera_points.len() as UINT,
                camera_points.as_ptr(),
                depth_points.len() as UINT,
                depth_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_camera_points_to_color_space(
        &self,
        camera_points: &[CameraSpacePoint],
        color_points: &mut [ColorSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if camera_points.len() != color_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapCameraPointsToColorSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                camera_points.len() as UINT,
                camera_points.as_ptr(),
                color_points.len() as UINT,
                color_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_points_to_camera_space(
        &self,
        depth_points: &[DepthSpacePoint],
        depths: &[UINT16],
        camera_points: &mut [CameraSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if depth_points.len() != depths.len() || depth_points.len() != camera_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthPointsToCameraSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_points.len() as UINT,
                depth_points.as_ptr(),
                depths.len() as UINT,
                depths.as_ptr(),
                camera_points.len() as UINT,
                camera_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_points_to_color_space(
        &self,
        depth_points: &[DepthSpacePoint],
        depths: &[UINT16],
        color_points: &mut [ColorSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if depth_points.len() != depths.len() || depth_points.len() != color_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthPointsToColorSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_points.len() as UINT,
                depth_points.as_ptr(),
                depths.len() as UINT,
                depths.as_ptr(),
                color_points.len() as UINT,
                color_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_frame_to_camera_space(
        &self,
        depth_frame_data: &[UINT16],
        camera_space_points: &mut [CameraSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if depth_frame_data.len() != camera_space_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthFrameToCameraSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_frame_data.len() as UINT,
                depth_frame_data.as_ptr(),
                camera_space_points.len() as UINT,
                camera_space_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_depth_frame_to_color_space(
        &self,
        depth_frame_data: &[UINT16],
        color_space_points: &mut [ColorSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        if depth_frame_data.len() != color_space_points.len() {
            return Err(Error::from_hresult(E_FAIL));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapDepthFrameToColorSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_frame_data.len() as UINT,
                depth_frame_data.as_ptr(),
                color_space_points.len() as UINT,
                color_space_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_color_frame_to_depth_space(
        &self,
        depth_frame_data: &[UINT16],
        depth_space_points: &mut [DepthSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapColorFrameToDepthSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_frame_data.len() as UINT,
                depth_frame_data.as_ptr(),
                depth_space_points.len() as UINT,
                depth_space_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn map_color_frame_to_camera_space(
        &self,
        depth_frame_data: &[UINT16],
        camera_space_points: &mut [CameraSpacePoint],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let map_fn = vtbl.MapColorFrameToCameraSpace.ok_or(E_FAIL)?;
        let hr = unsafe {
            map_fn(
                self.ptr,
                depth_frame_data.len() as UINT,
                depth_frame_data.as_ptr(),
                camera_space_points.len() as UINT,
                camera_space_points.as_mut_ptr(),
            )
        };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_frame_to_camera_space_table(&self) -> Result<(UINT, *mut PointF), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetDepthFrameToCameraSpaceTable.ok_or(E_FAIL)?;
        let mut table_entry_count: UINT = 0;
        let mut table_entries: *mut PointF = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut table_entry_count, &mut table_entries) };
        if hr.is_ok() {
            Ok((table_entry_count, table_entries))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_depth_camera_intrinsics(&self) -> Result<CameraIntrinsics, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetDepthCameraIntrinsics.ok_or(E_FAIL)?;
        let mut camera_intrinsics = CameraIntrinsics::default();
        let hr = unsafe { get_fn(self.ptr, &mut camera_intrinsics) };
        if hr.is_ok() {
            Ok(camera_intrinsics)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for CoordinateMapper {
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

pub struct CoordinateMappingChangedEventArgs {
    ptr: *mut ICoordinateMappingChangedEventArgs,
}

impl CoordinateMappingChangedEventArgs {
    pub(crate) fn new(ptr: *mut ICoordinateMappingChangedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Drop for CoordinateMappingChangedEventArgs {
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
