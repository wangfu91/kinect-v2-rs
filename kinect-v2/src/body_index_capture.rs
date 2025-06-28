use std::sync::Arc;

use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, WAITABLE_HANDLE,
    body_index::{BodyIndexFrame, BodyIndexFrameReader},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to body index frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture body index frames.
pub struct BodyIndexFrameCapture {
    _kinect: KinectSensor,        // keep the kinect sensor instance alive.
    reader: BodyIndexFrameReader, // Used to read body index frames.
}

impl BodyIndexFrameCapture {
    /// Creates a new `BodyIndexFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the body index frame source and reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the body index frame source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let source = kinect.body_index_frame_source()?;
        let reader = source.open_reader()?;

        // Ensure the body index frame source is active.
        // If not, event subscription and frame acquisition might fail.
        if !source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(BodyIndexFrameCapture {
            _kinect: kinect,
            reader,
        })
    }

    /// Returns an iterator over body index frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<BodyIndexFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<BodyIndexFrameCaptureIter<'a>, Error> {
        let mut waitable_handle = WAITABLE_HANDLE::default();
        self.reader.subscribe_frame_arrived(&mut waitable_handle)?;
        Ok(BodyIndexFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields body index frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `BodyIndexFrameCapture`.
pub struct BodyIndexFrameCaptureIter<'a> {
    reader: &'a BodyIndexFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for BodyIndexFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            log::warn!("Failed to unsubscribe body index frame arrived event: {e:?}");
        }
    }
}

impl<'a> Iterator for BodyIndexFrameCaptureIter<'a> {
    type Item = Result<BodyIndexFrameData, Error>;

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
                    let body_index_frame = frame_reference.acquire_frame()?;
                    BodyIndexFrameData::new(&body_index_frame)
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
pub struct BodyIndexStatistics {
    /// Number of pixels for each body (indices 0-5)
    pub body_pixel_counts: [u32; 6],
    /// Number of background pixels (body index 255)
    pub background_pixels: u32,
    /// Number of pixels with unknown body index (not 0-5 or 255)
    pub unknown_pixels: u32,
    /// Total number of pixels belonging to any body
    pub total_body_pixels: u32,
    /// Total number of pixels in the frame
    pub total_pixels: u32,
    /// Percentage of frame covered by bodies
    pub body_coverage_percentage: f32,
}

impl BodyIndexStatistics {
    /// Returns the number of actively tracked bodies (bodies with > 0 pixels)
    pub fn active_body_count(&self) -> usize {
        self.body_pixel_counts
            .iter()
            .filter(|&&count| count > 0)
            .count()
    }

    /// Returns the body index with the most pixels, if any bodies are present
    pub fn dominant_body_index(&self) -> Option<usize> {
        self.body_pixel_counts
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .and_then(|(index, count)| if *count > 0 { Some(index) } else { None })
    }
}

#[derive(Debug, Clone)]
pub struct BodyIndexFrameData {
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
    pub data: Arc<[u8]>,
}

impl BodyIndexFrameData {
    pub fn new(body_index_frame: &BodyIndexFrame) -> Result<Self, Error> {
        let frame_description = body_index_frame.get_frame_description()?;
        let width = frame_description.get_width()? as u32;
        let height = frame_description.get_height()? as u32;
        let timestamp = body_index_frame.get_relative_time()? as u64;

        // Body index frames contain one byte per pixel indicating which body
        // the pixel belongs to (0-5 for up to 6 tracked bodies, 255 for background)
        let buffer_size = width * height;
        let mut data = Vec::with_capacity(buffer_size as usize);

        let raw_buffer = body_index_frame.access_underlying_buffer()?;
        assert!(
            raw_buffer.len() as u32 == buffer_size,
            "Raw buffer size does not match expected size"
        );
        data.extend_from_slice(raw_buffer);

        Ok(Self {
            width,
            height,
            timestamp,
            data: Arc::from(data),
        })
    }

    /// Gets the body index for a specific pixel (x, y).
    /// Returns None if the coordinates are out of bounds.
    /// Returns Some(body_index) where body_index is 0-5 for tracked bodies,
    /// or 255 for background pixels.
    pub fn get_body_index_at(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.data.get(index).copied()
    }

    /// Returns true if the pixel at (x, y) belongs to a tracked body.
    pub fn is_body_pixel(&self, x: u32, y: u32) -> bool {
        self.get_body_index_at(x, y)
            .map(|body_index| body_index < 6) // 0-5 are valid body indices
            .unwrap_or(false)
    }

    /// Returns true if the pixel at (x, y) is background.
    pub fn is_background_pixel(&self, x: u32, y: u32) -> bool {
        self.get_body_index_at(x, y)
            .map(|body_index| body_index == 255) // 255 is background
            .unwrap_or(false)
    }

    /// Gets statistics about body coverage in the frame.
    pub fn get_body_statistics(&self) -> BodyIndexStatistics {
        let mut body_pixel_counts = [0u32; 6]; // Count for each body (0-5)
        let mut background_pixels = 0u32;
        let mut unknown_pixels = 0u32;

        for &body_index in self.data.iter() {
            match body_index {
                0..=5 => body_pixel_counts[body_index as usize] += 1,
                255 => background_pixels += 1,
                _ => unknown_pixels += 1,
            }
        }

        let total_pixels = self.width * self.height;
        let total_body_pixels: u32 = body_pixel_counts.iter().sum();

        BodyIndexStatistics {
            body_pixel_counts,
            background_pixels,
            unknown_pixels,
            total_body_pixels,
            total_pixels,
            body_coverage_percentage: (total_body_pixels as f32 / total_pixels as f32) * 100.0,
        }
    }
}

impl Default for BodyIndexFrameData {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            data: Arc::from([]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::sync::mpsc;

    #[test]
    fn body_index_capture_test() -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel::<BodyIndexFrameData>();
        let max_frames_to_capture = 10;
        let capture_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            let capture = BodyIndexFrameCapture::new()?;
            for (frame_count, frame) in capture.iter()?.enumerate() {
                if frame_count >= max_frames_to_capture {
                    break;
                }
                let data = frame.map_err(|e| anyhow!("Error capturing body index frame: {}", e))?;
                if tx.send(data).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let processing_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            for _ in 0..max_frames_to_capture {
                let frame_data = match rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };
                println!(
                    "Received body index frame: {}x{}, {} bytes, timestamp: {}",
                    frame_data.width,
                    frame_data.height,
                    frame_data.data.len(),
                    frame_data.timestamp
                );
                anyhow::ensure!(
                    frame_data.width > 0,
                    "Unexpected width: {}",
                    frame_data.width
                );
                anyhow::ensure!(
                    frame_data.height > 0,
                    "Unexpected height: {}",
                    frame_data.height
                );
                anyhow::ensure!(!frame_data.data.is_empty(), "Frame data is empty");
                anyhow::ensure!(frame_data.timestamp > 0, "Timestamp is not positive");
            }
            Ok(())
        });

        capture_thread
            .join()
            .map_err(|e| anyhow!("Body index capture thread join error: {:?}", e))??;
        processing_thread
            .join()
            .map_err(|e| anyhow!("Processing thread join error: {:?}", e))??;
        Ok(())
    }
}
