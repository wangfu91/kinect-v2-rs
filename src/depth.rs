use crate::bindings::{IDepthFrameReference, IDepthFrameSource};
use std::ptr;

pub struct DepthFrameSource {
    ptr: *mut IDepthFrameSource,
}
impl DepthFrameSource {
    pub(crate) fn new(ptr: *mut IDepthFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for DepthFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct DepthFrameReference {
    ptr: *mut IDepthFrameReference,
}

impl DepthFrameReference {
    pub(crate) fn new(ptr: *mut IDepthFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Drop for DepthFrameReference {
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
