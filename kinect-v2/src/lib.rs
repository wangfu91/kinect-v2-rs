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
    use std::sync::mpsc;

    use super::*;

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
