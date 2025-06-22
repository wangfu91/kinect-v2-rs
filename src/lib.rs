pub mod audio;
pub mod bindings;
pub mod body;
pub mod body_index;
pub mod color;
pub mod coordinate;
pub mod depth;
pub mod frame;
pub mod infrared;
pub mod kinect;
pub mod long_exposure_infrared;
pub mod multi_source_frame;
pub mod utils;

/// Default timeout for waiting for a new frame in milliseconds
pub const DEFAULT_FRAME_WAIT_TIMEOUT_MS: u32 = 1000;

/// Default FPS for Kinect Color/Infrared/Depth streams
pub const KINECT_DEFAULT_CAPTURE_FPS: u32 = 30;

pub use crate::bindings::Activity;
pub use crate::bindings::Appearance;
pub use crate::bindings::AudioBeamMode;
pub use crate::bindings::ColorImageFormat;
pub use crate::bindings::DetectionResult;
pub use crate::bindings::Expression;
pub use crate::bindings::FrameCapturedStatus;
pub use crate::bindings::FrameEdges;
pub use crate::bindings::FrameSourceTypes;
pub use crate::bindings::HandState;
pub use crate::bindings::HandType;
pub use crate::bindings::JointType;
pub use crate::bindings::KinectAudioCalibrationState;
pub use crate::bindings::KinectCapabilities;
pub use crate::bindings::KinectEngagementMode;
pub use crate::bindings::KinectGestureSettings;
pub use crate::bindings::KinectHoldingState;
pub use crate::bindings::KinectInteractionMode;
pub use crate::bindings::PointerDeviceType;
pub use crate::bindings::TrackingConfidence;
pub use crate::bindings::TrackingState;
pub use crate::bindings::WAITABLE_HANDLE;
