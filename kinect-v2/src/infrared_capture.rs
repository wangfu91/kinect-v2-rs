use std::sync::Arc;

use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, KINECT_DEFAULT_CAPTURE_FPS, WAITABLE_HANDLE,
    infrared::{InfraredFrame, InfraredFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to infrared frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture infrared frames.
pub struct InfraredFrameCapture {
    _kinect: KinectSensor,       // keep the kinect sensor instance alive.
    reader: InfraredFrameReader, // Used to read infrared frames.
}

impl InfraredFrameCapture {
    /// Creates a new `InfraredFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the infrared frame source and reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the infrared frame source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let source = kinect.infrared_frame_source()?;
        let reader = source.open_reader()?;

        // Ensure the infrared frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(InfraredFrameCapture {
            _kinect: kinect,
            reader,
        })
    }

    /// Returns an iterator over infrared frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<InfraredFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<InfraredFrameCaptureIter<'a>, Error> {
        let waitable_handle = self.reader.subscribe_frame_arrived()?;
        Ok(InfraredFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields infrared frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `InfraredFrameCapture`.
pub struct InfraredFrameCaptureIter<'a> {
    reader: &'a InfraredFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for InfraredFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            log::warn!(
                "Failed to unsubscribe infrared frame arrived event: {:?}",
                e
            );
        }
    }
}

impl<'a> Iterator for InfraredFrameCaptureIter<'a> {
    type Item = Result<InfraredFrameData, Error>;

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
                    let infrared_frame = frame_reference.acquire_frame()?;
                    InfraredFrameData::new(&infrared_frame)
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
pub struct InfraredFrameData {
    pub width: u32,
    pub height: u32,
    pub fps: u32, // Infrared frames typically run at 30 FPS
    pub timestamp: u64,
    pub data: Arc<[u16]>, // Infrared data is typically 16-bit unsigned integers
}

impl InfraredFrameData {
    pub fn new(infrared_frame: &InfraredFrame) -> Result<Self, Error> {
        let frame_description = infrared_frame.get_frame_description()?;
        let width = frame_description.get_width()? as u32;
        let height = frame_description.get_height()? as u32;
        let timestamp = infrared_frame.get_relative_time()? as u64;
        let raw_buffer = infrared_frame.access_underlying_buffer()?;

        Ok(Self {
            width,
            height,
            fps: KINECT_DEFAULT_CAPTURE_FPS,
            timestamp,
            data: Arc::from(raw_buffer.to_vec()),
        })
    }

    /// Gets the infrared intensity at a specific pixel coordinate.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get_intensity_at(&self, x: u32, y: u32) -> Option<u16> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.data.get(index).copied()
    }

    /// Normalizes infrared values to 0.0-1.0 range for visualization.
    ///
    /// This is useful for converting raw infrared values to a normalized
    /// range suitable for image processing or visualization.
    pub fn normalize_intensity(&self, intensity: u16) -> f32 {
        // Typical infrared values range from 0 to 65535 (16-bit)
        intensity as f32 / 65535.0
    }

    /// Converts infrared frame data to 8-bit grayscale for visualization.
    ///
    /// This method scales the 16-bit infrared values down to 8-bit values
    /// suitable for standard image formats.
    pub fn to_grayscale_u8(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|&intensity| (intensity >> 8) as u8) // Simple bit-shift scaling
            .collect()
    }

    /// Gets the minimum and maximum infrared intensity values in the frame.
    ///
    /// This is useful for adaptive scaling and analysis of the infrared data.
    pub fn get_intensity_range(&self) -> (u16, u16) {
        let min_intensity = *self.data.iter().min().unwrap_or(&0);
        let max_intensity = *self.data.iter().max().unwrap_or(&0);
        (min_intensity, max_intensity)
    }
}

impl Default for InfraredFrameData {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            fps: KINECT_DEFAULT_CAPTURE_FPS,
            timestamp: 0,
            data: Arc::from([]),
        }
    }
}
