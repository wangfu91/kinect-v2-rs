use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, WAITABLE_HANDLE,
    bindings::FrameSourceTypes,
    kinect::{self, KinectSensor},
    multi_source_frame::{MultiSourceFrame, MultiSourceFrameReader},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

use crate::{
    body_capture::BodyFrameData, body_index_capture::BodyIndexFrameData,
    color_capture::ColorFrameData, depth_capture::DepthFrameData,
    infrared_capture::InfraredFrameData,
};

pub struct MultiSourceCapture {
    _kinect: KinectSensor,
    reader: MultiSourceFrameReader,
}

impl MultiSourceCapture {
    /// Creates a new `MultiSourceCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the multi-source frame reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized or opened.
    pub fn new() -> Result<Self, windows::core::Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let enabled_frame_source_types = FrameSourceTypes::Color
            | FrameSourceTypes::Depth
            | FrameSourceTypes::Infrared
            | FrameSourceTypes::BodyIndex
            | FrameSourceTypes::Body;

        let reader = kinect.open_multi_source_frame_reader(enabled_frame_source_types)?;

        Ok(MultiSourceCapture {
            _kinect: kinect,
            reader,
        })
    }

    /// Returns an iterator over multi-source frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<MultiSourceFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<MultiSourceCaptureIter<'a>, Error> {
        let mut waitable_handle = WAITABLE_HANDLE::default();
        self.reader
            .subscribe_multi_source_frame_arrived(&mut waitable_handle)?;
        Ok(MultiSourceCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }
}

/// An iterator that yields multi-source frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `MultiSourceCapture`.
pub struct MultiSourceCaptureIter<'a> {
    reader: &'a MultiSourceFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for MultiSourceCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(_e) = self
            .reader
            .unsubscribe_multi_source_frame_arrived(self.waitable_handle)
        {
            // Consider logging this error if a logging facade is available.
            // eprintln!("Failed to unsubscribe multi-source frame arrived event: {:?}", e);
        }
    }
}

impl<'a> Iterator for MultiSourceCaptureIter<'a> {
    type Item = Result<MultiSourceFrameData, Error>;

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
                        .get_multi_source_frame_arrived_event_data(self.waitable_handle)?;
                    let frame_reference = event_args.get_frame_reference()?;
                    let multi_source_frame = frame_reference.acquire_frame()?;
                    MultiSourceFrameData::new(&multi_source_frame)
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

/// Container for multi-source frame data from the Kinect sensor.
///
/// This struct holds optional frame data for each enabled frame source type.
/// Not all frame types may be available in every multi-source frame.
#[derive(Debug)]
pub struct MultiSourceFrameData {
    pub color_frame: Option<ColorFrameData>,
    pub depth_frame: Option<DepthFrameData>,
    pub infrared_frame: Option<InfraredFrameData>,
    pub body_index_frame: Option<BodyIndexFrameData>,
    pub body_frame: Option<BodyFrameData>,
}

impl MultiSourceFrameData {
    pub fn new(multi_source_frame: &MultiSourceFrame) -> Result<Self, Error> {
        // Try to get each frame type, but don't fail if one is not available
        let color_frame = Self::get_color_frame_data(multi_source_frame);
        let depth_frame = Self::get_depth_frame_data(multi_source_frame);
        let infrared_frame = Self::get_infrared_frame_data(multi_source_frame);
        let body_index_frame = Self::get_body_index_frame_data(multi_source_frame);
        let body_frame = Self::get_body_frame_data(multi_source_frame);

        Ok(Self {
            color_frame,
            depth_frame,
            infrared_frame,
            body_index_frame,
            body_frame,
        })
    }

    fn get_color_frame_data(multi_source_frame: &MultiSourceFrame) -> Option<ColorFrameData> {
        let color_frame_reference = multi_source_frame.get_color_frame_reference().ok()?;
        let color_frame = color_frame_reference.acquire_frame().ok()?;
        ColorFrameData::new(&color_frame).ok()
    }

    fn get_depth_frame_data(multi_source_frame: &MultiSourceFrame) -> Option<DepthFrameData> {
        let depth_frame_reference = multi_source_frame.get_depth_frame_reference().ok()?;
        let depth_frame = depth_frame_reference.acquire_frame().ok()?;
        DepthFrameData::new(&depth_frame).ok()
    }

    fn get_infrared_frame_data(multi_source_frame: &MultiSourceFrame) -> Option<InfraredFrameData> {
        let infrared_frame_reference = multi_source_frame.get_infrared_frame_reference().ok()?;
        let infrared_frame = infrared_frame_reference.acquire_frame().ok()?;
        InfraredFrameData::new(&infrared_frame).ok()
    }

    fn get_body_index_frame_data(
        multi_source_frame: &MultiSourceFrame,
    ) -> Option<BodyIndexFrameData> {
        let body_index_frame_reference =
            multi_source_frame.get_body_index_frame_reference().ok()?;
        let body_index_frame = body_index_frame_reference.acquire_frame().ok()?;
        BodyIndexFrameData::new(&body_index_frame).ok()
    }

    fn get_body_frame_data(multi_source_frame: &MultiSourceFrame) -> Option<BodyFrameData> {
        let body_frame_reference = multi_source_frame.get_body_frame_reference().ok()?;
        let body_frame = body_frame_reference.acquire_frame().ok()?;
        BodyFrameData::new(&body_frame).ok()
    }
}
