use std::sync::Arc;

use kinect_v2_sys::{
    ColorImageFormat, DEFAULT_FRAME_WAIT_TIMEOUT_MS, KINECT_DEFAULT_CAPTURE_FPS, WAITABLE_HANDLE,
    color::{ColorFrame, ColorFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to color frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture color frames.
pub struct ColorFrameCapture {
    _kinect: KinectSensor,    // keep the kinect sensor instance alive.
    reader: ColorFrameReader, // Used to read color frames.
}

impl ColorFrameCapture {
    /// Creates a new `ColorFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the color frame source and reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the color frame source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let source = kinect.color_frame_source()?;
        let reader = source.open_reader()?;

        // Ensure the color frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(ColorFrameCapture {
            _kinect: kinect,
            reader,
        })
    }

    /// Returns an iterator over color frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<ColorFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<ColorFrameCaptureIter<'a>, Error> {
        let mut waitable_handle = WAITABLE_HANDLE::default();
        self.reader.subscribe_frame_arrived(&mut waitable_handle)?;
        Ok(ColorFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields color frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `ColorFrameCapture`.
pub struct ColorFrameCaptureIter<'a> {
    reader: &'a ColorFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for ColorFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(_e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            // Consider logging this error if a logging facade is available.
            // eprintln!("Failed to unsubscribe color frame arrived event: {:?}", e);
        }
    }
}

impl<'a> Iterator for ColorFrameCaptureIter<'a> {
    type Item = Result<ColorFrameData, Error>;

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
                    let color_frame = frame_reference.acquire_frame()?;
                    ColorFrameData::new(&color_frame)
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
pub struct ColorFrameData {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bytes_per_pixel: u32,
    pub timestamp: u64,
    pub image_format: ColorImageFormat,
    pub data: Arc<[u8]>, //Use Arc<[u8]> instead of Vec<u8> to avoid expensive cloning
}

impl ColorFrameData {
    pub fn new(color_frame: &ColorFrame) -> Result<Self, Error> {
        let frame_description = color_frame.get_frame_description()?;
        let width = frame_description.get_width()? as u32;
        let height = frame_description.get_height()? as u32;
        let bytes_per_pixel = frame_description.get_bytes_per_pixel()?;
        let timestamp = color_frame.get_relative_time()? as u64;
        let image_format = color_frame.get_raw_color_image_format()?;
        let buffer_size = width * height * bytes_per_pixel;
        let mut data = Vec::with_capacity(buffer_size as usize);
        let raw_buffer = color_frame.access_raw_underlying_buffer()?;
        assert!(
            raw_buffer.len() as u32 == buffer_size,
            "Raw buffer size does not match expected size"
        );
        data.extend_from_slice(raw_buffer);

        Ok(Self {
            width,
            height,
            fps: KINECT_DEFAULT_CAPTURE_FPS,
            bytes_per_pixel,
            timestamp,
            image_format,
            data: Arc::from(data), // Convert Vec<u8> to Arc<[u8]>
        })
    }
}

impl Default for ColorFrameData {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            fps: KINECT_DEFAULT_CAPTURE_FPS,
            bytes_per_pixel: 0,
            timestamp: 0,
            image_format: ColorImageFormat::None,
            data: Arc::new([]), // Use an empty Arc<[u8]> for default
        }
    }
}
