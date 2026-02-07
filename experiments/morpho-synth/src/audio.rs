use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioEngine {
    _stream: Option<cpal::Stream>, // Keep the stream alive
    pub active_frequencies: Arc<Mutex<Vec<f32>>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let active_frequencies = Arc::new(Mutex::new(Vec::new()));
        let active_frequencies_clone = active_frequencies.clone();

        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(d) => d,
            None => {
                println!("Audio disabled: No output device found (Ghost Mode active)");
                return Self {
                    _stream: None,
                    active_frequencies,
                };
            }
        };

        let config = match device.default_output_config() {
            Ok(c) => c,
            Err(e) => {
                println!("Audio disabled: Error getting config: {} (Ghost Mode active)", e);
                return Self {
                    _stream: None,
                    active_frequencies,
                };
            }
        };

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;
        let mut phase = 0.0;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, sample_rate, &mut phase, &active_frequencies_clone)
                },
                err_fn,
                None,
            ),
            _ => {
                println!("Audio disabled: Unsupported sample format (Ghost Mode active)");
                 return Self {
                    _stream: None,
                    active_frequencies,
                };
            }
        };

        let stream = match stream {
             Ok(s) => s,
             Err(e) => {
                 println!("Audio disabled: Stream build error: {} (Ghost Mode active)", e);
                 return Self {
                    _stream: None,
                    active_frequencies,
                };
             }
        };

        match stream.play() {
             Ok(_) => (),
             Err(e) => println!("Audio disabled: Stream play error: {}", e),
        }

        Self {
            _stream: Some(stream),
            active_frequencies,
        }
    }

    pub fn update_active_frequencies(&self, freqs: Vec<f32>) {
        let mut active = self.active_frequencies.lock().unwrap();
        *active = freqs;
    }
}

fn write_data(output: &mut [f32], channels: usize, sample_rate: f32, phase: &mut f32, active_frequencies: &Arc<Mutex<Vec<f32>>>) {
    let freqs = active_frequencies.lock().unwrap();
    let num_active = freqs.len();

    if channels == 0 { return; }

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;
        if num_active > 0 {
             for &freq in freqs.iter() {
                 sample += (*phase * freq * 2.0 * std::f32::consts::PI).sin();
             }
             // Simple mixing: average or sum?
             // Sum can clip. Average is quieter.
             // Let's do sum but scale by fixed amount to avoid total silence if average.
             // Actually, tanh for soft clipping is nice, but exp expensive.
             // Let's just scale by 0.1 * (1.0 / sqrt(num_active)) to keep energy consistent?
             // Or just 0.1 / num_active

             if num_active > 0 {
                 sample = sample * 0.1;
             }
        }

        for sample_out in frame.iter_mut() {
            *sample_out = sample;
        }

        *phase = (*phase + 1.0 / sample_rate) % 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_init() {
        let engine = AudioEngine::new();
        // Just verify it doesn't crash
        assert!(engine.active_frequencies.lock().unwrap().len() == 0);
    }
}
