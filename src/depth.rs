use crate::bindings::IDepthFrameSource;
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
