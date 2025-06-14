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

pub const DEFAULT_FRAME_WAIT_TIMEOUT_MS: u32 = 1000; // Default timeout for waiting for a new frame in milliseconds

pub use crate::bindings::WAITABLE_HANDLE;
