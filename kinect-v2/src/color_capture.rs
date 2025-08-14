use std::sync::Arc;

use kinect_v2_sys::{
    ColorImageFormat, DEFAULT_FRAME_WAIT_TIMEOUT_MS, KINECT_DEFAULT_CAPTURE_FPS, WAITABLE_HANDLE,
    color::{ColorFrame, ColorFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, E_INVALIDARG, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to color frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture color frames.
pub struct ColorFrameCapture {
    kinect: KinectSensor,             // keep the kinect sensor instance alive.
    format: Option<ColorImageFormat>, // The desired color image format.
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

        Ok(ColorFrameCapture {
            kinect,
            format: None,
        })
    }

    /// Creates a new `ColorFrameCapture` instance with the specified color image format.
    pub fn new_with_format(color_image_format: ColorImageFormat) -> Result<Self, Error> {
        let mut capture = Self::new()?;
        capture.format = Some(color_image_format);
        Ok(capture)
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
    pub fn iter(&self) -> Result<ColorFrameCaptureIter, Error> {
        let source = self.kinect.color_frame_source()?;
        // Open the reader to active the source.
        let reader = source.open_reader()?;
        // Ensure the color frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            log::warn!(
                "Color frame source is not active, cannot subscribe to frame arrived event."
            );
            return Err(Error::from_hresult(E_FAIL));
        }
        let mut waitable_handle = WAITABLE_HANDLE::default();

        reader.subscribe_frame_arrived(&mut waitable_handle)?;
        Ok(ColorFrameCaptureIter {
            reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
            format: self.format.clone(),
        })
    }
}

/// An iterator that yields color frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `ColorFrameCapture`.
pub struct ColorFrameCaptureIter {
    reader: ColorFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
    format: Option<ColorImageFormat>,
}

impl Drop for ColorFrameCaptureIter {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            log::warn!("Failed to unsubscribe color frame arrived event: {e:?}");
        }
    }
}

impl Iterator for ColorFrameCaptureIter {
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
                    ColorFrameData::new(&color_frame, self.format.clone())
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

fn get_bytes_per_pixel_for_format(format: ColorImageFormat) -> u32 {
    match format {
        ColorImageFormat::Rgba | ColorImageFormat::Bgra | ColorImageFormat::Bayer => 4,
        ColorImageFormat::Yuy2 | ColorImageFormat::Yuv => 2,
        ColorImageFormat::None => 0,
    }
}

impl ColorFrameData {
    pub fn new(
        color_frame: &ColorFrame,
        format_opt: Option<ColorImageFormat>,
    ) -> Result<Self, Error> {
        let frame_description = color_frame.get_frame_description()?;
        let width = frame_description.get_width()? as u32;
        let height = frame_description.get_height()? as u32;
        let bytes_per_pixel = frame_description.get_bytes_per_pixel()?;
        let buffer_size = width * height * bytes_per_pixel;
        let timestamp = color_frame.get_relative_time()? as u64;
        let image_format = color_frame.get_raw_color_image_format()?;
        if let Some(desired_format) = format_opt {
            if image_format != desired_format {
                // If the image format does not match the required format, we need to convert it.
                let desired_bpp = get_bytes_per_pixel_for_format(desired_format.clone());
                if desired_bpp == 0 {
                    return Err(Error::from_hresult(E_INVALIDARG));
                }
                let converted_buffer_size = width * height * desired_bpp;
                let mut converted_data = vec![0u8; converted_buffer_size as usize];
                color_frame.copy_converted_frame_data_to_array(
                    &mut converted_data,
                    desired_format.clone(),
                )?;
                return Ok(Self {
                    width,
                    height,
                    fps: KINECT_DEFAULT_CAPTURE_FPS,
                    bytes_per_pixel: desired_bpp,
                    timestamp,
                    image_format: desired_format,
                    data: Arc::from(converted_data), // Convert Vec<u8> to Arc<[u8]>
                });
            }
        }

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

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use anyhow::anyhow;

    use super::*;

    #[test]
    fn color_capture_test() -> anyhow::Result<()> {
        let (color_tx, color_rx) = mpsc::channel::<ColorFrameData>();
        let max_frames_to_capture = 10;
        let color_capture_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            let color_capture = ColorFrameCapture::new()?;
            for (frame_count, frame) in color_capture.iter()?.enumerate() {
                if frame_count >= max_frames_to_capture {
                    break;
                }
                let data = frame.map_err(|e| anyhow!("Error capturing color frame: {}", e))?;
                if color_tx.send(data).is_err() {
                    // Receiver dropped, exit thread
                    break;
                }
            }
            Ok(())
        });

        let processing_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            for _ in 0..max_frames_to_capture {
                let frame_data = match color_rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };
                println!(
                    "Received color frame: {}x{}, {} bytes, timestamp: {}, format: {:?}, bytes_per_pixel: {}",
                    frame_data.width,
                    frame_data.height,
                    frame_data.data.len(),
                    frame_data.timestamp,
                    frame_data.image_format,
                    frame_data.bytes_per_pixel
                );

                anyhow::ensure!(
                    frame_data.width == 1920,
                    "Unexpected width: {}",
                    frame_data.width
                );
                anyhow::ensure!(
                    frame_data.height == 1080,
                    "Unexpected height: {}",
                    frame_data.height
                );
                anyhow::ensure!(
                    [2, 4].contains(&frame_data.bytes_per_pixel),
                    "Unexpected bytes_per_pixel: {}",
                    frame_data.bytes_per_pixel
                );
                anyhow::ensure!(!frame_data.data.is_empty(), "Frame data is empty");
                anyhow::ensure!(frame_data.timestamp > 0, "Timestamp is not positive");
                anyhow::ensure!(
                    frame_data.fps == KINECT_DEFAULT_CAPTURE_FPS,
                    "Unexpected FPS: {}",
                    frame_data.fps
                );
            }
            Ok(())
        });

        color_capture_thread
            .join()
            .map_err(|e| anyhow!("Color capture thread join error: {:?}", e))??;
        processing_thread
            .join()
            .map_err(|e| anyhow!("Processing thread join error: {:?}", e))??;

        Ok(())
    }
}
