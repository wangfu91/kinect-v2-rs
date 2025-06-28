use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, WAITABLE_HANDLE,
    bindings::{
        CameraSpacePoint, DetectionResult, FrameEdges, HandState, Joint, JointOrientation,
        JointType, PointF, TrackingConfidence, TrackingState, Vector4,
    },
    body::{Body, BodyFrame, BodyFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to body frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture body frames.
pub struct BodyFrameCapture {
    _kinect: KinectSensor,   // keep the kinect sensor instance alive.
    reader: BodyFrameReader, // Used to read body frames.
}

impl BodyFrameCapture {
    /// Creates a new `BodyFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the body frame source and reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the body frame source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let source = kinect.body_frame_source()?;
        let reader = source.open_reader()?;

        // Ensure the body frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(BodyFrameCapture {
            _kinect: kinect,
            reader,
        })
    }

    /// Returns an iterator over body frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<BodyFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<BodyFrameCaptureIter<'a>, Error> {
        let mut waitable_handle = WAITABLE_HANDLE::default();
        self.reader.subscribe_frame_arrived(&mut waitable_handle)?;
        Ok(BodyFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields body frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `BodyFrameCapture`.
pub struct BodyFrameCaptureIter<'a> {
    reader: &'a BodyFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for BodyFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            log::warn!("Failed to unsubscribe body frame arrived event: {e:?}");
        }
    }
}

impl<'a> Iterator for BodyFrameCaptureIter<'a> {
    type Item = Result<BodyFrameData, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let wait_status: WAIT_EVENT =
                unsafe { WaitForSingleObject(self.waitable_handle, self.timeout_ms) };

            if wait_status == WAIT_OBJECT_0 {
                // Frame event was signaled.
                // Use a closure and the `?` operator for cleaner error handling.
                let result = (|| {
                    let event_args = self
                        .reader
                        .get_frame_arrived_event_data(self.waitable_handle)?;
                    let frame_reference = event_args.get_frame_reference()?;
                    let body_frame = frame_reference.acquire_frame()?;
                    BodyFrameData::new(&body_frame)
                })(); // Immediately invoke the closure
                return Some(result);
            } else if wait_status == WAIT_TIMEOUT {
                // No new frame arrived within the timeout period.
                // Continue waiting as this is a blocking iterator.
                continue;
            } else {
                return Some(Err(Error::from_hresult(E_FAIL)));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyData {
    pub tracking_id: u64,
    pub is_tracked: bool,
    pub is_restricted: bool,
    pub hand_left_state: HandState,
    pub hand_right_state: HandState,
    pub hand_left_confidence: TrackingConfidence,
    pub hand_right_confidence: TrackingConfidence,
    pub clipped_edges: FrameEdges,
    pub lean: PointF,
    pub lean_tracking_state: TrackingState,
    pub is_engaged: bool,
    pub joints: Vec<Joint>,
    pub joint_orientations: Vec<JointOrientation>,
}

impl BodyData {
    pub fn new(body: Body) -> Result<Self, Error> {
        use kinect_v2_sys::bindings::{Joint, JointOrientation};

        // Get basic body properties
        let tracking_id = body.get_tracking_id()?;

        let is_tracked = body.get_is_tracked()?;

        let is_restricted = body.get_is_restricted()?;

        let hand_left_state = body.get_hand_left_state()?;

        let hand_right_state = body.get_hand_right_state()?;

        let hand_left_confidence = body.get_hand_left_confidence()?;

        let hand_right_confidence = body.get_hand_right_confidence()?;

        let clipped_edges = body.get_clipped_edges()?;

        let is_engaged = body.get_engaged()? == DetectionResult::Yes;

        // Get lean data
        let lean = body.get_lean()?;
        let lean_tracking_state = body.get_lean_tracking_state()?;

        // Get joints (25 joints in Kinect v2)
        const JOINT_COUNT: usize = 25;
        let mut joints: [Joint; JOINT_COUNT] = unsafe { std::mem::zeroed() };
        body.get_joints(joints.as_mut())?;
        let joints: Vec<Joint> = joints.to_vec();

        // Get joint orientations
        let mut joint_orientations: [JointOrientation; JOINT_COUNT] = unsafe { std::mem::zeroed() };
        body.get_joint_orientations(joint_orientations.as_mut())?;
        let joint_orientations: Vec<JointOrientation> = joint_orientations.to_vec();

        Ok(Self {
            tracking_id,
            is_tracked,
            is_restricted,
            hand_left_state,
            hand_right_state,
            hand_left_confidence,
            hand_right_confidence,
            clipped_edges,
            lean,
            lean_tracking_state,
            is_engaged,
            joints,
            joint_orientations,
        })
    }

    /// Gets the position of a specific joint by joint type.
    pub fn get_joint_position(&self, joint_type: JointType) -> Option<CameraSpacePoint> {
        self.joints
            .iter()
            .find(|joint| joint.JointType == joint_type)
            .map(|joint| joint.Position.clone())
    }

    /// Gets the orientation of a specific joint by joint type.
    pub fn get_joint_orientation(&self, joint_type: JointType) -> Option<Vector4> {
        self.joint_orientations
            .iter()
            .find(|joint_orientation| joint_orientation.JointType == joint_type)
            .map(|joint_orientation| joint_orientation.Orientation.clone())
    }

    /// Checks if the body is actively being tracked.
    pub fn is_actively_tracked(&self) -> bool {
        self.is_tracked && !self.is_restricted
    }
}

#[derive(Debug, Clone, Default)]
pub struct BodyFrameData {
    pub timestamp: u64,
    pub floor_clip_plane: Vector4,
    pub body_count: u32,
    pub bodies: Vec<BodyData>,
}

impl BodyFrameData {
    pub fn new(body_frame: &BodyFrame) -> Result<Self, Error> {
        let timestamp = body_frame.get_relative_time()? as u64;
        // Get floor clip plane
        let floor_clip_plane = body_frame.get_floor_clip_plane()?;
        // Get body count from the body frame source
        let body_frame_source = body_frame.get_body_frame_source()?;
        let body_count = body_frame_source.get_body_count()? as u32;

        // Get and refresh body data
        let bodies = body_frame.get_and_refresh_body_data()?;
        let bodies = bodies
            .into_iter()
            .map(BodyData::new)
            .collect::<Result<Vec<BodyData>, Error>>()?;

        Ok(Self {
            timestamp,
            floor_clip_plane,
            body_count,
            bodies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::sync::mpsc;

    #[test]
    fn body_capture_test() -> anyhow::Result<()> {
        let (body_tx, body_rx) = mpsc::channel::<BodyFrameData>();
        let max_frames_to_capture = 10;
        let body_capture_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            let body_capture = BodyFrameCapture::new()?;
            for (frame_count, frame) in body_capture.iter()?.enumerate() {
                if frame_count >= max_frames_to_capture {
                    break;
                }
                let data = frame.map_err(|e| anyhow!("Error capturing body frame: {}", e))?;
                if body_tx.send(data).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let processing_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            for _ in 0..max_frames_to_capture {
                let frame_data = match body_rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };
                println!(
                    "Received body frame: timestamp: {}, body_count: {}",
                    frame_data.timestamp, frame_data.body_count
                );
                anyhow::ensure!(frame_data.body_count > 0, "No bodies detected");
                anyhow::ensure!(frame_data.timestamp > 0, "Timestamp is not positive");
            }
            Ok(())
        });

        body_capture_thread
            .join()
            .map_err(|e| anyhow!("Body capture thread join error: {:?}", e))??;
        processing_thread
            .join()
            .map_err(|e| anyhow!("Processing thread join error: {:?}", e))??;
        Ok(())
    }
}
