pub mod audio_capture;
pub mod body_capture;
pub mod body_index_capture;
pub mod color_capture;
pub mod depth_capture;
pub mod infrared_capture;
pub mod multi_source_capture;

#[cfg(test)]
mod tests {
    use crate::audio_capture::AudioFrameData;
    use crate::color_capture::ColorFrameData;
    use crate::depth_capture::DepthFrameData;
    use crate::infrared_capture::InfraredFrameData;
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn capture_all_test() -> anyhow::Result<()> {
        let (color_tx, color_rx) = mpsc::channel::<ColorFrameData>();
        let (depth_tx, depth_rx) = mpsc::channel::<DepthFrameData>();
        let (infrared_tx, infrared_rx) = mpsc::channel::<InfraredFrameData>();
        let (audio_tx, audio_rx) = mpsc::channel::<AudioFrameData>();

        let color_capture_thread = std::thread::spawn(move || {
            let color_capture = color_capture::ColorFrameCapture::new().unwrap();
            for frame in color_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        color_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing color frame: {}", e);
                    }
                }
            }
        });

        let depth_capture_thread = std::thread::spawn(move || {
            let depth_capture = depth_capture::DepthFrameCapture::new().unwrap();
            for frame in depth_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        depth_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing depth frame: {}", e);
                    }
                }
            }
        });

        let infrared_capture_thread = std::thread::spawn(move || {
            let infrared_capture = infrared_capture::InfraredFrameCapture::new().unwrap();
            for frame in infrared_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        infrared_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing infrared frame: {}", e);
                    }
                }
            }
        });

        let audio_capture_thread = std::thread::spawn(move || {
            let audio_capture = audio_capture::AudioFrameCapture::new().unwrap();
            for frame in audio_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        audio_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing audio frame: {}", e);
                    }
                }
            }
        });

        // Here you can process the captured frames from all channels
        // This is a simple example that just logs frame information
        let processing_thread = std::thread::spawn(move || {
            loop {
                // Try to receive color frames (non-blocking)
                if let Ok(color_frame) = color_rx.try_recv() {
                    log::debug!(
                        "Captured color frame: {}x{}, {} bytes, timestamp: {}",
                        color_frame.width,
                        color_frame.height,
                        color_frame.data.len(),
                        color_frame.timestamp
                    );
                }

                // Try to receive depth frames (non-blocking)
                if let Ok(depth_frame) = depth_rx.try_recv() {
                    log::debug!(
                        "Captured depth frame: {}x{}, {} depth values, timestamp: {}, depth range: {}-{}mm",
                        depth_frame.width,
                        depth_frame.height,
                        depth_frame.data.len(),
                        depth_frame.timestamp,
                        depth_frame.depth_min_reliable_distance,
                        depth_frame.depth_max_reliable_distance
                    );
                }

                // Try to receive infrared frames (non-blocking)
                if let Ok(infrared_frame) = infrared_rx.try_recv() {
                    let (min_intensity, max_intensity) = infrared_frame.get_intensity_range();
                    log::debug!(
                        "Captured infrared frame: {}x{}, {} intensity values, timestamp: {}, intensity range: {}-{}",
                        infrared_frame.width,
                        infrared_frame.height,
                        infrared_frame.data.len(),
                        infrared_frame.timestamp,
                        min_intensity,
                        max_intensity
                    );
                }

                // Try to receive audio frames (non-blocking)
                if let Ok(audio_frame) = audio_rx.try_recv() {
                    let beam_angle = audio_frame.beam_angle;
                    let beam_angle_confidence = audio_frame.beam_angle_confidence;
                    let audio_data = audio_frame.data;

                    log::debug!(
                        "Captured audio frame: beam_angle: {}, beam_angle_confidence: {}, timestamp: {}, audio data size: {} bytes",
                        beam_angle,
                        beam_angle_confidence,
                        audio_frame.timestamp,
                        audio_data.len()
                    );
                }

                // Small delay to prevent busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        // Wait for the capture threads to finish
        color_capture_thread.join().unwrap();
        depth_capture_thread.join().unwrap();
        infrared_capture_thread.join().unwrap();
        audio_capture_thread.join().unwrap();
        processing_thread.join().unwrap();

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
            let audio_capture = audio_capture::AudioFrameCapture::new().unwrap();
            for frame in audio_capture.iter().unwrap() {
                match frame {
                    Ok(data) => {
                        audio_tx.send(data).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error capturing audio frame: {}", e);
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
