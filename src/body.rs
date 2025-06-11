use crate::bindings::{
    IBodyFrameReference, IBodyFrameSource, IBodyIndexFrameReference, IBodyIndexFrameSource,
};
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

pub struct BodyFrameReference {
    ptr: *mut IBodyFrameReference,
}

impl BodyFrameReference {
    pub(crate) fn new(ptr: *mut IBodyFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Drop for BodyFrameReference {
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

pub struct BodyIndexFrameReference {
    ptr: *mut IBodyIndexFrameReference,
}

impl BodyIndexFrameReference {
    pub(crate) fn new(ptr: *mut IBodyIndexFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Drop for BodyIndexFrameReference {
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
