use crate::bindings::ICoordinateMapper;
use std::ptr;

pub struct CoordinateMapper {
    ptr: *mut ICoordinateMapper,
}
impl CoordinateMapper {
    pub(crate) fn new(ptr: *mut ICoordinateMapper) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}
impl Drop for CoordinateMapper {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).Release.unwrap())(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
