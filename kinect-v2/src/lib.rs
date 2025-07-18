pub mod audio_capture;
pub mod body_capture;
pub mod body_index_capture;
pub mod color_capture;
pub mod depth_capture;
pub mod infrared_capture;
pub mod multi_source_capture;

pub use kinect_v2_sys::bindings::Activity;
pub use kinect_v2_sys::bindings::Appearance;
pub use kinect_v2_sys::bindings::AudioBeamMode;
pub use kinect_v2_sys::bindings::ColorImageFormat;
pub use kinect_v2_sys::bindings::DetectionResult;
pub use kinect_v2_sys::bindings::Expression;
pub use kinect_v2_sys::bindings::FrameCapturedStatus;
pub use kinect_v2_sys::bindings::FrameEdges;
pub use kinect_v2_sys::bindings::FrameSourceTypes;
pub use kinect_v2_sys::bindings::HandState;
pub use kinect_v2_sys::bindings::HandType;
pub use kinect_v2_sys::bindings::JointType;
pub use kinect_v2_sys::bindings::KinectAudioCalibrationState;
pub use kinect_v2_sys::bindings::KinectCapabilities;
pub use kinect_v2_sys::bindings::KinectEngagementMode;
pub use kinect_v2_sys::bindings::KinectGestureSettings;
pub use kinect_v2_sys::bindings::KinectHoldingState;
pub use kinect_v2_sys::bindings::KinectInteractionMode;
pub use kinect_v2_sys::bindings::PointerDeviceType;
pub use kinect_v2_sys::bindings::TrackingConfidence;
pub use kinect_v2_sys::bindings::TrackingState;

#[cfg(test)]
mod tests {
    // TODO: Add tests for the kinect-v2 module
}
