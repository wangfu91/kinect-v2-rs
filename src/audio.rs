use crate::bindings::IAudioSource;
use std::ptr;

pub struct AudioSource {
    ptr: *mut IAudioSource,
}
impl AudioSource {
    pub(crate) fn new(ptr: *mut IAudioSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for AudioSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
