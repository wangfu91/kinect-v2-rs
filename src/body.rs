use crate::bindings::{IBodyFrameSource, IBodyIndexFrameSource};
use std::ptr;

pub struct BodyFrameSource {
    ptr: *mut IBodyFrameSource,
}
impl BodyFrameSource {
    pub(crate) fn new(ptr: *mut IBodyFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for BodyFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct BodyIndexFrameSource {
    ptr: *mut IBodyIndexFrameSource,
}
impl BodyIndexFrameSource {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for BodyIndexFrameSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
