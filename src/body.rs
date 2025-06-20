use crate::bindings::{
    BOOLEAN, DetectionResult, FrameEdges, HandState, IBody, IBodyFrame, IBodyFrameArrivedEventArgs,
    IBodyFrameReader, IBodyFrameReference, IBodyFrameSource, IBodyHandPair,
    IFrameCapturedEventArgs, IKinectSensor, INT32, Joint, JointOrientation, TIMESPAN,
    TrackingConfidence, UINT, WAITABLE_HANDLE,
};
use crate::frame::FrameCapturedEventArgs;
use crate::kinect::KinectSensor;
use std::ptr;
use windows::Win32::Foundation::{E_FAIL, E_POINTER};
use windows::core::Error;

pub struct Body {
    ptr: *mut IBody,
}

impl Body {
    pub(crate) fn new(ptr: *mut IBody) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_joints(&self, joints: &mut [Joint]) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetJoints.ok_or(E_FAIL)?;
        let capacity = joints.len() as UINT;
        let hr = unsafe { get_fn(self.ptr, capacity, joints.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_joint_orientations(
        &self,
        joint_orientations: &mut [JointOrientation],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetJointOrientations.ok_or(E_FAIL)?;
        let capacity = joint_orientations.len() as UINT;
        let hr = unsafe { get_fn(self.ptr, capacity, joint_orientations.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_engaged(&self) -> Result<DetectionResult, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Engaged.ok_or(E_FAIL)?;
        let mut detection_result: DetectionResult = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut detection_result) };
        if hr.is_ok() {
            Ok(detection_result)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_expression_detection_results(
        &self,
        detection_results: &mut [DetectionResult],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetExpressionDetectionResults.ok_or(E_FAIL)?;
        let capacity = detection_results.len() as UINT;
        let hr = unsafe { get_fn(self.ptr, capacity, detection_results.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_activity_detection_results(
        &self,
        detection_results: &mut [DetectionResult],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetActivityDetectionResults.ok_or(E_FAIL)?;
        let capacity = detection_results.len() as UINT;
        let hr = unsafe { get_fn(self.ptr, capacity, detection_results.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_appearance_detection_results(
        &self,
        detection_results: &mut [DetectionResult],
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetAppearanceDetectionResults.ok_or(E_FAIL)?;
        let capacity = detection_results.len() as UINT;
        let hr = unsafe { get_fn(self.ptr, capacity, detection_results.as_mut_ptr()) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_hand_left_state(&self) -> Result<HandState, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HandLeftState.ok_or(E_FAIL)?;
        let mut hand_state: HandState = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut hand_state) };
        if hr.is_ok() {
            Ok(hand_state)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_hand_left_confidence(&self) -> Result<TrackingConfidence, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HandLeftConfidence.ok_or(E_FAIL)?;
        let mut confidence: TrackingConfidence = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut confidence) };
        if hr.is_ok() {
            Ok(confidence)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_hand_right_state(&self) -> Result<HandState, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HandRightState.ok_or(E_FAIL)?;
        let mut hand_state: HandState = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut hand_state) };
        if hr.is_ok() {
            Ok(hand_state)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_hand_right_confidence(&self) -> Result<TrackingConfidence, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HandRightConfidence.ok_or(E_FAIL)?;
        let mut confidence: TrackingConfidence = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut confidence) };
        if hr.is_ok() {
            Ok(confidence)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_clipped_edges(&self) -> Result<FrameEdges, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_ClippedEdges.ok_or(E_FAIL)?;
        let mut clipped_edges: FrameEdges = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut clipped_edges) };
        if hr.is_ok() {
            Ok(clipped_edges)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_tracking_id(&self) -> Result<u64, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_TrackingId.ok_or(E_FAIL)?;
        let mut tracking_id: crate::bindings::UINT64 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut tracking_id) };
        if hr.is_ok() {
            Ok(tracking_id)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_tracked(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsTracked.ok_or(E_FAIL)?;
        let mut tracked: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut tracked) };
        if hr.is_ok() {
            Ok(tracked != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_restricted(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsRestricted.ok_or(E_FAIL)?;
        let mut is_restricted: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut is_restricted) };
        if hr.is_ok() {
            Ok(is_restricted != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_lean(&self) -> Result<crate::bindings::PointF, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_Lean.ok_or(E_FAIL)?;
        let mut amount: crate::bindings::PointF = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut amount) };
        if hr.is_ok() {
            Ok(amount)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_lean_tracking_state(&self) -> Result<crate::bindings::TrackingState, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_LeanTrackingState.ok_or(E_FAIL)?;
        let mut tracking_state: crate::bindings::TrackingState = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut tracking_state) };
        if hr.is_ok() {
            Ok(tracking_state)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for Body {
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

pub struct BodyHandPair {
    ptr: *mut IBodyHandPair,
}

impl BodyHandPair {
    pub(crate) fn new(ptr: *mut IBodyHandPair) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_body_tracking_id(&self) -> Result<u64, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyTrackingId.ok_or(E_FAIL)?;
        let mut value: crate::bindings::UINT64 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut value) };
        if hr.is_ok() {
            Ok(value)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn put_body_tracking_id(&self, value: u64) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_BodyTrackingId.ok_or(E_FAIL)?;
        let hr = unsafe { put_fn(self.ptr, value) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_hand_type(&self) -> Result<crate::bindings::HandType, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_HandType.ok_or(E_FAIL)?;
        let mut value: crate::bindings::HandType = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut value) };
        if hr.is_ok() {
            Ok(value)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn put_hand_type(&self, value: crate::bindings::HandType) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_HandType.ok_or(E_FAIL)?;
        let hr = unsafe { put_fn(self.ptr, value) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for BodyHandPair {
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

pub struct BodyFrameSource {
    ptr: *mut IBodyFrameSource,
}
impl BodyFrameSource {
    pub(crate) fn new(ptr: *mut IBodyFrameSource) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn subscribe_frame_captured(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_frame_captured(&self, handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameCaptured.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_captured_event_data(
        &self,
        handle: WAITABLE_HANDLE,
    ) -> Result<FrameCapturedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameCapturedEventData.ok_or(E_FAIL)?;
        let mut event_data_ptr: *mut IFrameCapturedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, handle, &mut event_data_ptr) };
        if hr.is_ok() {
            if event_data_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(FrameCapturedEventArgs::new(event_data_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_active(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsActive.ok_or(E_FAIL)?;
        let mut is_active: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut is_active) };
        if hr.is_ok() {
            Ok(is_active != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_body_count(&self) -> Result<i32, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyCount.ok_or(E_FAIL)?;
        let mut count: INT32 = 0;
        let hr = unsafe { get_fn(self.ptr, &mut count) };
        if hr.is_ok() {
            Ok(count)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn open_reader(&self) -> Result<BodyFrameReader, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let open_fn = vtbl.OpenReader.ok_or(E_FAIL)?;
        let mut reader_ptr: *mut IBodyFrameReader = ptr::null_mut();
        let hr = unsafe { open_fn(self.ptr, &mut reader_ptr) };
        if hr.is_ok() {
            if reader_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrameReader::new(reader_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_kinect_sensor(&self) -> Result<KinectSensor, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_KinectSensor.ok_or(E_FAIL)?;
        let mut sensor_ptr: *mut IKinectSensor = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut sensor_ptr) };
        if hr.is_ok() {
            if sensor_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(KinectSensor::new(sensor_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn override_hand_tracking(&self, tracking_id: u64) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let override_fn = vtbl.OverrideHandTracking.ok_or(E_FAIL)?;
        let hr = unsafe { override_fn(self.ptr, tracking_id) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn override_and_replace_hand_tracking(
        &self,
        old_tracking_id: u64,
        new_tracking_id: u64,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let override_fn = vtbl.OverrideAndReplaceHandTracking.ok_or(E_FAIL)?;
        let hr = unsafe { override_fn(self.ptr, old_tracking_id, new_tracking_id) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}
impl Drop for BodyFrameSource {
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

pub struct BodyFrameReference {
    ptr: *mut IBodyFrameReference,
}

impl BodyFrameReference {
    pub(crate) fn new(ptr: *mut IBodyFrameReference) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_frame(&self) -> Result<BodyFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IBodyFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            if frame_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrame::new(frame_ptr))
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

pub struct BodyFrameReader {
    ptr: *mut IBodyFrameReader,
}

impl BodyFrameReader {
    pub(crate) fn new(ptr: *mut IBodyFrameReader) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn acquire_latest_frame(&self) -> Result<BodyFrame, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let acquire_fn = vtbl.AcquireLatestFrame.ok_or(E_FAIL)?;
        let mut frame_ptr: *mut IBodyFrame = ptr::null_mut();
        let hr = unsafe { acquire_fn(self.ptr, &mut frame_ptr) };
        if hr.is_ok() {
            if frame_ptr.is_null() {
                return Err(Error::from_hresult(hr));
            }
            Ok(BodyFrame::new(frame_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn subscribe_frame_arrived(
        &self,
        waitable_handle: &mut WAITABLE_HANDLE,
    ) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let sub_fn = vtbl.SubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { sub_fn(self.ptr, waitable_handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn unsubscribe_frame_arrived(&self, handle: WAITABLE_HANDLE) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let unsub_fn = vtbl.UnsubscribeFrameArrived.ok_or(E_FAIL)?;
        let hr = unsafe { unsub_fn(self.ptr, handle) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_frame_arrived_event_data(
        &self,
        handle: WAITABLE_HANDLE,
    ) -> Result<BodyFrameArrivedEventArgs, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetFrameArrivedEventData.ok_or(E_FAIL)?;
        let mut event_data_ptr: *mut IBodyFrameArrivedEventArgs = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, handle, &mut event_data_ptr) };
        if hr.is_ok() {
            if event_data_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrameArrivedEventArgs::new(event_data_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_is_paused(&self) -> Result<bool, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_IsPaused.ok_or(E_FAIL)?;
        let mut is_paused: BOOLEAN = 0;
        let hr = unsafe { get_fn(self.ptr, &mut is_paused) };
        if hr.is_ok() {
            Ok(is_paused != 0)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn put_is_paused(&self, is_paused: bool) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let put_fn = vtbl.put_IsPaused.ok_or(E_FAIL)?;
        let hr = unsafe { put_fn(self.ptr, is_paused as BOOLEAN) };
        if hr.is_ok() {
            Ok(())
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_body_frame_source(&self) -> Result<BodyFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IBodyFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            if source_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrameSource::new(source_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}
impl Drop for BodyFrameReader {
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

pub struct BodyFrame {
    ptr: *mut IBodyFrame,
}

impl BodyFrame {
    pub(crate) fn new(ptr: *mut IBodyFrame) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_floor_clip_plane(&self) -> Result<crate::bindings::Vector4, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FloorClipPlane.ok_or(E_FAIL)?;
        let mut floor_clip_plane: crate::bindings::Vector4 = unsafe { std::mem::zeroed() };
        let hr = unsafe { get_fn(self.ptr, &mut floor_clip_plane) };
        if hr.is_ok() {
            Ok(floor_clip_plane)
        } else {
            Err(Error::from_hresult(hr))
        }
    }
    pub fn get_and_refresh_body_data(&self) -> Result<Vec<Body>, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }

        // Get the body frame source to determine body count
        let body_source = self.get_body_frame_source()?;
        let body_count = body_source.get_body_count()? as usize;

        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.GetAndRefreshBodyData.ok_or(E_FAIL)?;

        // Create null pointers for the API to fill
        let mut raw_ptrs: Vec<*mut IBody> = vec![ptr::null_mut(); body_count];
        let hr = unsafe { get_fn(self.ptr, body_count as UINT, raw_ptrs.as_mut_ptr()) };

        if hr.is_ok() {
            // Convert non-null pointers to Body objects
            let bodies: Vec<Body> = raw_ptrs
                .into_iter()
                .filter(|&ptr| !ptr.is_null())
                .map(Body::new)
                .collect();
            Ok(bodies)
        } else {
            Err(Error::from_hresult(hr))
        }
    }

    pub fn get_body_frame_source(&self) -> Result<BodyFrameSource, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_BodyFrameSource.ok_or(E_FAIL)?;
        let mut source_ptr: *mut IBodyFrameSource = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut source_ptr) };
        if hr.is_ok() {
            if source_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrameSource::new(source_ptr))
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
impl Drop for BodyFrame {
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

pub struct BodyFrameArrivedEventArgs {
    ptr: *mut IBodyFrameArrivedEventArgs,
}

impl BodyFrameArrivedEventArgs {
    pub(crate) fn new(ptr: *mut IBodyFrameArrivedEventArgs) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn get_frame_reference(&self) -> Result<BodyFrameReference, Error> {
        if self.ptr.is_null() {
            return Err(Error::from_hresult(E_POINTER));
        }
        let vtbl = unsafe { (*self.ptr).lpVtbl.as_ref() }.ok_or(E_POINTER)?;
        let get_fn = vtbl.get_FrameReference.ok_or(E_FAIL)?;
        let mut frame_ref_ptr: *mut IBodyFrameReference = ptr::null_mut();
        let hr = unsafe { get_fn(self.ptr, &mut frame_ref_ptr) };
        if hr.is_ok() {
            if frame_ref_ptr.is_null() {
                return Err(Error::from_hresult(E_FAIL));
            }
            Ok(BodyFrameReference::new(frame_ref_ptr))
        } else {
            Err(Error::from_hresult(hr))
        }
    }
}

impl Drop for BodyFrameArrivedEventArgs {
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
