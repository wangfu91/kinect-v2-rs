use crate::bindings::{IInfraredFrameSource, ILongExposureInfraredFrameSource};
use std::ptr;

pub struct InfraredFrameSource {
    ptr: *mut IInfraredFrameSource,
}
impl InfraredFrameSource {
    pub(crate) fn new(ptr: *mut IInfraredFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for InfraredFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct LongExposureInfraredFrameSource {
    ptr: *mut ILongExposureInfraredFrameSource,
}
impl LongExposureInfraredFrameSource {
    pub(crate) fn new(ptr: *mut ILongExposureInfraredFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for LongExposureInfraredFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
