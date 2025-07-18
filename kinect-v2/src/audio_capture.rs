use std::sync::Arc;

use kinect_v2_sys::{
    DEFAULT_FRAME_WAIT_TIMEOUT_MS, WAITABLE_HANDLE,
    audio::{AudioBeamFrameList, AudioBeamFrameReader, AudioSource},
    kinect::{self, KinectSensor},
};
use windows::Win32::Foundation::{E_FAIL, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::{Win32::Foundation::WAIT_EVENT, core::Error};

/// Manages the Kinect sensor and provides access to audio frame data.
///
/// This struct is responsible for initializing and holding the necessary Kinect
/// resources to capture audio frames from audio beams.
pub struct AudioFrameCapture {
    _kinect: KinectSensor,        // keep the kinect sensor instance alive.
    reader: AudioBeamFrameReader, // Used to read audio beam frames.
    audio_source: AudioSource,    // Audio source for configuration.
}

impl AudioFrameCapture {
    /// Creates a new `AudioFrameCapture` instance.
    ///
    /// This function initializes the default Kinect sensor, opens it,
    /// and sets up the audio source and audio beam frame reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the Kinect sensor cannot be initialized,
    /// opened, or if the audio source is not active.
    pub fn new() -> Result<Self, Error> {
        let kinect = kinect::get_default_kinect_sensor()?;
        kinect.open()?;

        let audio_source = kinect.audio_source()?;
        let reader = audio_source.open_reader()?;

        // Ensure the audio source is active.
        // If not, event subscription and frame acquisition might fail.
        if !audio_source.get_is_active()? {
            return Err(Error::from_hresult(E_FAIL));
        }

        Ok(AudioFrameCapture {
            _kinect: kinect,
            reader,
            audio_source,
        })
    }

    /// Returns an iterator over audio frames.
    ///
    /// The iterator will block waiting for new frames. Each item yielded by
    /// the iterator is a `Result<AudioFrameData, Error>`, allowing for error
    /// handling during frame acquisition.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to subscribe to the frame arrived event,
    /// which is necessary for the iterator to function.
    pub fn iter<'a>(&'a self) -> Result<AudioFrameCaptureIter<'a>, Error> {
        let waitable_handle = self.reader.subscribe_frame_arrived()?;
        Ok(AudioFrameCaptureIter {
            reader: &self.reader,
            waitable_handle,
            timeout_ms: DEFAULT_FRAME_WAIT_TIMEOUT_MS,
        })
    }

    /// Gets the audio source for configuration purposes.
    pub fn audio_source(&self) -> &AudioSource {
        &self.audio_source
    }
}

/// An iterator that yields audio frames from a Kinect sensor.
///
/// This iterator blocks until a new frame is available or an error occurs.
/// It is created by calling the `iter` method on `AudioFrameCapture`.
pub struct AudioFrameCaptureIter<'a> {
    reader: &'a AudioBeamFrameReader,
    waitable_handle: WAITABLE_HANDLE,
    timeout_ms: u32,
}

impl<'a> Drop for AudioFrameCaptureIter<'a> {
    fn drop(&mut self) {
        // Best effort to unsubscribe from the frame arrived event.
        // Errors in `drop` are typically logged or ignored, as panicking in drop is problematic.
        if let Err(e) = self.reader.unsubscribe_frame_arrived(self.waitable_handle) {
            log::warn!("Failed to unsubscribe audio frame arrived event: {e:?}");
        }
    }
}

impl<'a> Iterator for AudioFrameCaptureIter<'a> {
    type Item = Result<AudioFrameData, Error>;

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
                    let audio_beam_frame_list = frame_reference.acquire_beam_frame()?;
                    AudioFrameData::new(&audio_beam_frame_list)
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
pub struct AudioBeamData {
    pub beam_angle: f32,
    pub beam_angle_confidence: f32,
    pub sub_frames: Vec<AudioSubFrameData>,
}

#[derive(Debug, Clone)]
pub struct AudioSubFrameData {
    pub duration: i64, // TIMESPAN
    pub beam_angle: f32,
    pub beam_angle_confidence: f32,
    pub relative_time: i64, // TIMESPAN
    pub audio_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct AudioFrameData {
    pub timestamp: u64,
    pub beam_angle: f32,
    pub beam_angle_confidence: f32,
    pub data: Arc<[u8]>,
}

impl AudioFrameData {
    pub fn new(audio_beam_frame_list: &AudioBeamFrameList) -> Result<Self, Error> {
        let beam_count = audio_beam_frame_list.get_beam_count()?;
        assert_eq!(beam_count, 1, "Kinect V2 only supports one audio beam");

        let audio_beam_frame = audio_beam_frame_list.open_audio_beam_frame(0)?;
        let timestamp = audio_beam_frame.get_relative_time_start()? as u64;
        let audio_beam = audio_beam_frame.get_audio_beam()?;
        let beam_angle = audio_beam.get_beam_angle()?;
        let beam_angle_confidence = audio_beam.get_beam_angle_confidence()?;

        let sub_frame_count = audio_beam_frame.get_sub_frame_count()?;
        let mut sub_frames = Vec::new();

        for sub_frame_index in 0..sub_frame_count {
            let audio_sub_frame = audio_beam_frame.get_sub_frame(sub_frame_index)?;

            let duration = audio_sub_frame.get_duration()?;
            let sub_beam_angle = audio_sub_frame.get_beam_angle()?;
            let sub_beam_angle_confidence = audio_sub_frame.get_beam_angle_confidence()?;
            let relative_time = audio_sub_frame.get_relative_time()?;

            // Get audio data from the sub-frame
            let audio_data = audio_sub_frame.access_underlying_buffer()?.to_vec();

            sub_frames.push(AudioSubFrameData {
                duration,
                beam_angle: sub_beam_angle,
                beam_angle_confidence: sub_beam_angle_confidence,
                relative_time,
                audio_data,
            });
        }

        let mut data = Vec::new();
        for sub_frame in sub_frames {
            data.extend_from_slice(&sub_frame.audio_data);
        }

        Ok(Self {
            timestamp,
            beam_angle,
            beam_angle_confidence,
            data: Arc::from(data), // Use Arc to avoid expensive cloning
        })
    }
}

impl Default for AudioFrameData {
    fn default() -> Self {
        Self {
            timestamp: 0,
            beam_angle: 0.0,
            beam_angle_confidence: 0.0,
            data: Arc::new([]), // Use an empty Arc<[u8]> for default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::sync::mpsc;

    #[test]
    fn audio_capture_test() -> anyhow::Result<()> {
        let (audio_tx, audio_rx) = mpsc::channel::<AudioFrameData>();
        let max_frames_to_capture = 10;
        let audio_capture_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            let audio_capture = AudioFrameCapture::new()?;
            for (frame_count, frame) in audio_capture.iter()?.enumerate() {
                if frame_count >= max_frames_to_capture {
                    break;
                }
                let data = frame.map_err(|e| anyhow!("Error capturing audio frame: {}", e))?;
                if audio_tx.send(data).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let processing_thread = std::thread::spawn(move || -> anyhow::Result<()> {
            for _ in 0..max_frames_to_capture {
                let frame_data = match audio_rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };
                println!(
                    "Received audio frame: timestamp: {}, beam_angle: {}, confidence: {}, data_len: {}",
                    frame_data.timestamp,
                    frame_data.beam_angle,
                    frame_data.beam_angle_confidence,
                    frame_data.data.len()
                );
                anyhow::ensure!(!frame_data.data.is_empty(), "Audio data is empty");
                anyhow::ensure!(frame_data.timestamp > 0, "Timestamp is not positive");
            }
            Ok(())
        });

        audio_capture_thread
            .join()
            .map_err(|e| anyhow!("Audio capture thread join error: {:?}", e))??;
        processing_thread
            .join()
            .map_err(|e| anyhow!("Processing thread join error: {:?}", e))??;
        Ok(())
    }

    #[test]
    fn record_wav_audio_test() -> anyhow::Result<()> {
        let wav_spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,  // Kinect's audio sample rate
            bits_per_sample: 32, // 32-bit float samples
            sample_format: hound::SampleFormat::Float,
        };
        let mut wav_writer = hound::WavWriter::create("test_audio.wav", wav_spec)?;

        let (audio_tx, audio_rx) = mpsc::channel::<AudioFrameData>();
        std::thread::spawn(move || {
            let audio_capture = AudioFrameCapture::new().unwrap();
            for frame in audio_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        audio_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing audio frame: {e}");
                    }
                }
            }
        });

        let mut frame_counter = 0;
        println!("Starting audio capture...");
        loop {
            if let Ok(frame) = audio_rx.recv() {
                // Interpret as 32-bit float samples (Kinect's native format)
                let f32_samples: Vec<f32> = frame
                    .data
                    .chunks_exact(4)
                    .map(|chunk| f32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                    .collect();

                for sample in f32_samples {
                    wav_writer.write_sample(sample)?;
                }

                frame_counter += 1;
                if frame_counter >= 512 {
                    // Limit to 512 frames for testing
                    break;
                }
            }
        }

        wav_writer.finalize()?;
        println!("Audio data written to test_audio.wav");

        Ok(())
    }
}
