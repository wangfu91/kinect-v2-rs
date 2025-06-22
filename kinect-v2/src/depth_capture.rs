use std::sync::Arc;

use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, WAITABLE_HANDLE,
    depth::{DepthFrame, DepthFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to depth frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture depth frames.
pub struct DepthFrameCapture {
    _kinect: KinectSensor,    // keep the kinect sensor instance alive.
    reader: DepthFrameReader, // Used to read depth frames.
}

impl DepthFrameCapture {
    /// Creates a new `DepthFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the depth frame source and reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the depth frame source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let source = kinect.depth_frame_source()?;
        let reader = source.open_reader()?;

        // Ensure the depth frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(DepthFrameCapture {
            _kinect: kinect,
            reader,
        })
    }
    /// Returns an iterator over depth frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<DepthFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<DepthFrameCaptureIter<'a>, Error> {
        let waitable_handle = self.reader.subscribe_frame_arrived()?;
        Ok(DepthFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields depth frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `DepthFrameCapture`.
pub struct DepthFrameCaptureIter<'a> {
    reader: &'a DepthFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for DepthFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(_e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            // Consider logging this error if a logging facade is available.
            // eprintln!("Failed to unsubscribe depth frame arrived event: {:?}", e);
        }
    }
}

impl<'a> Iterator for DepthFrameCaptureIter<'a> {
    type Item = Result<DepthFrameData, Error>;

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
                    let depth_frame = frame_reference.acquire_frame()?;
                    DepthFrameData::new(&depth_frame)
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
pub struct DepthFrameData {
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
    pub depth_min_reliable_distance: u16,
    pub depth_max_reliable_distance: u16,
    pub data: Arc<[u16]>, // Depth data is typically 16-bit unsigned integers
}

impl DepthFrameData {
    pub fn new(depth_frame: &DepthFrame) -> Result<Self, Error> {
        let frame_description = depth_frame.get_frame_description()?;
        let width = frame_description.get_width()? as u32;
        let height = frame_description.get_height()? as u32;
        let timestamp = depth_frame.get_relative_time()? as u64;
        let depth_min_reliable_distance = depth_frame.get_depth_min_reliable_distance()?;
        let depth_max_reliable_distance = depth_frame.get_depth_max_reliable_distance()?;
        let raw_buffer = depth_frame.access_underlying_buffer()?;

        Ok(Self {
            width,
            height,
            timestamp,
            depth_min_reliable_distance,
            depth_max_reliable_distance,
            data: Arc::from(raw_buffer.to_vec()),
        })
    }

    /// Converts depth values to meters.
    ///
    /// The depth values from the Kinect are in millimeters. This method
    /// provides a convenient way to convert them to meters.
    pub fn depth_to_meters(&self, depth_value: u16) -> f32 {
        depth_value as f32 / 1000.0
    }

    /// Checks if a depth value is within the reliable range.
    ///
    /// The Kinect has minimum and maximum reliable distances. This method
    /// helps determine if a depth reading is trustworthy.
    pub fn is_depth_reliable(&self, depth_value: u16) -> bool {
        depth_value >= self.depth_min_reliable_distance
            && depth_value <= self.depth_max_reliable_distance
    }

    /// Gets the depth value at a specific pixel coordinate.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get_depth_at(&self, x: u32, y: u32) -> Option<u16> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.data.get(index).copied()
    }
}

impl Default for DepthFrameData {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            depth_min_reliable_distance: 0,
            depth_max_reliable_distance: 0,
            data: Arc::from([]),
        }
    }
}
