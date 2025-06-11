use crate::bindings::IMultiSourceFrameReader;
use std::ptr;

pub struct MultiSourceFrameReader {
    ptr: *mut IMultiSourceFrameReader,
}
impl MultiSourceFrameReader {
    pub(crate) fn new(ptr: *mut IMultiSourceFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for MultiSourceFrameReader {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
