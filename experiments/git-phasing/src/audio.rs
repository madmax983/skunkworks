use anyhow::Result;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::engine::Engine;

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

#[cfg(not(feature = "audio"))]
pub fn init_audio(engine: Arc<Mutex<Engine>>) -> Result<AudioSystem> {
    eprintln!("Audio disabled. Simulation mode.");

    std::thread::spawn(move || {
        let sample_rate = 44100.0;
        let dt = 1.0 / sample_rate;
        let chunk_size = 441; // 10ms
        let chunk_duration = std::time::Duration::from_millis(10);

        loop {
            let start = std::time::Instant::now();

            {
                let mut engine = engine.lock();
                let bpm = engine.bpm; // Copy BPM before borrow
                for _ in 0..chunk_size {
                     engine.track1.tick(dt, bpm);
                     engine.track2.tick(dt, bpm);
                }
            }

            let elapsed = start.elapsed();
            if elapsed < chunk_duration {
                std::thread::sleep(chunk_duration - elapsed);
            }
        }
    });

    Ok(AudioSystem {})
}

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "audio")]
struct Voice {
    phase: f32,
    freq: f32,
    amp: f32,
    decay: f32,
}

#[cfg(feature = "audio")]
impl Voice {
    fn new() -> Self {
        Self {
            phase: 0.0,
            freq: 440.0,
            amp: 0.0,
            decay: 0.9995,
        }
    }

    fn process(&mut self, sample_rate: f32) -> f32 {
        self.phase += self.freq / sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

        let signal = if self.phase < 0.5 {
            4.0 * self.phase - 1.0
        } else {
            1.0 - 4.0 * (self.phase - 0.5)
        };

        self.amp *= self.decay;

        signal * self.amp
    }

    fn trigger(&mut self, freq: f32, vel: f32) {
        self.freq = freq;
        self.amp = vel * 0.5;
    }
}

#[cfg(feature = "audio")]
pub fn init_audio(engine: Arc<Mutex<Engine>>) -> Result<AudioSystem> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No output device"))?;
    let config = device.default_output_config()?;
    let config: cpal::StreamConfig = config.into();
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f32;

    let mut voice1 = Voice::new();
    let mut voice2 = Voice::new();

    voice1.decay = 0.9992;
    voice2.decay = 0.9992;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut engine = engine.lock();
            let dt = 1.0 / sample_rate;
            let bpm = engine.bpm; // Copy BPM before borrow

            for frame in data.chunks_mut(channels) {
                if let Some((freq, vel)) = engine.track1.tick(dt, bpm) {
                    voice1.trigger(freq, vel);
                }

                if let Some((freq, vel)) = engine.track2.tick(dt, bpm) {
                    voice2.trigger(freq, vel);
                }

                let out1 = voice1.process(sample_rate);
                let out2 = voice2.process(sample_rate);

                if channels >= 2 {
                    frame[0] = out1;
                    frame[1] = out2;
                } else if channels == 1 {
                    frame[0] = (out1 + out2) * 0.5;
                }
            }
        },
        |err| eprintln!("Audio error: {}", err),
        None,
    )?;

    stream.play()?;

    Ok(AudioSystem { _stream: stream })
}
