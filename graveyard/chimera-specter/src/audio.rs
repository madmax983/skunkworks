use anyhow::Result;
use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const FFT_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 44100;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioSystem {
    samples: Arc<Mutex<Vec<f32>>>,
    #[cfg(feature = "audio")]
    _stream: Option<cpal::Stream>,
    ghost_mode: bool,
    start_time: Instant,
    planner: FftPlanner<f32>,
}

impl AudioSystem {
    pub fn new() -> Result<Self> {
        let samples = Arc::new(Mutex::new(vec![0.0; FFT_SIZE]));
        let start_time = Instant::now();
        let planner = FftPlanner::new();

        #[cfg(feature = "audio")]
        let (stream, ghost_mode) = {
            let host = cpal::default_host();
            let device = host.default_input_device();
            let samples_clone = samples.clone();

            if let Some(device) = device {
                log::info!(
                    "Found input device: {}",
                    device.name().unwrap_or("Unknown".to_string())
                );

                let config: cpal::StreamConfig = device
                    .default_input_config()
                    .map(|c| c.into())
                    .unwrap_or_else(|_| cpal::StreamConfig {
                        channels: 1,
                        sample_rate: cpal::SampleRate(SAMPLE_RATE),
                        buffer_size: cpal::BufferSize::Default,
                    });

                let stream_result = device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &_| {
                        if let Ok(mut buffer) = samples_clone.lock() {
                            let len = buffer.len();
                            let data_len = data.len();

                            if data_len >= len {
                                buffer.copy_from_slice(&data[data_len - len..]);
                            } else {
                                buffer.drain(0..data_len);
                                buffer.extend_from_slice(data);
                            }
                        }
                    },
                    move |err| {
                        log::error!("Stream error: {}", err);
                    },
                    None,
                );

                match stream_result {
                    Ok(s) => {
                        s.play().ok();
                        log::info!("Audio stream started successfully");
                        (Some(s), false)
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to build input stream: {}, falling back to Ghost Mode",
                            e
                        );
                        (None, true)
                    }
                }
            } else {
                log::warn!("No input device found, falling back to Ghost Mode");
                (None, true)
            }
        };

        #[cfg(not(feature = "audio"))]
        let ghost_mode = true;

        Ok(Self {
            samples,
            #[cfg(feature = "audio")]
            _stream: stream,
            ghost_mode,
            start_time,
            planner,
        })
    }

    pub fn get_spectrum(&mut self) -> Vec<f32> {
        if self.ghost_mode {
            return self.generate_ghost_spectrum();
        }

        let samples = {
            let s = self.samples.lock().unwrap();
            s.clone()
        };

        self.compute_fft(&samples)
    }

    fn generate_ghost_spectrum(&self) -> Vec<f32> {
        let t = self.start_time.elapsed().as_secs_f32();
        let mut spectrum = Vec::with_capacity(FFT_SIZE / 2);

        for i in 0..(FFT_SIZE / 2) {
            let freq = i as f32 * SAMPLE_RATE as f32 / FFT_SIZE as f32;
            let mut val = 0.01;

            let lfo1 = (t * 0.5).sin() * 200.0 + 300.0;
            let lfo2 = (t * 1.3).cos() * 500.0 + 1000.0;

            let dist1 = (freq - lfo1).abs();
            if dist1 < 50.0 {
                val += 1.0 - (dist1 / 50.0);
            }

            let dist2 = (freq - lfo2).abs();
            if dist2 < 100.0 {
                val += 0.5 * (1.0 - (dist2 / 100.0));
            }

            if freq < 100.0 {
                let beat = (t * 4.0).sin();
                if beat > 0.8 {
                    val += 2.0;
                }
            }

            spectrum.push(val);
        }

        spectrum
    }

    fn compute_fft(&mut self, samples: &[f32]) -> Vec<f32> {
        let fft = self.planner.plan_fft_forward(FFT_SIZE);

        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .map(|&s| Complex { re: s, im: 0.0 })
            .collect();

        for (i, sample) in buffer.iter_mut().enumerate() {
            let window = 0.5
                * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE as f32 - 1.0)).cos());
            sample.re *= window;
        }

        fft.process(&mut buffer);

        buffer.iter().take(FFT_SIZE / 2).map(|c| c.norm()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_mode() {
        let mut audio = AudioSystem::new().expect("Failed to create AudioSystem");
        let spectrum = audio.get_spectrum();
        assert_eq!(spectrum.len(), FFT_SIZE / 2);
        let sum: f32 = spectrum.iter().sum();
        assert!(sum > 0.0, "Spectrum is all zeros");
    }
}
