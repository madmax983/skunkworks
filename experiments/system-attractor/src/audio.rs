#[cfg(feature = "audio")]
pub mod audio_impl {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use ringbuf::{HeapRb, Producer, SharedRb};
    use std::sync::{Arc, Mutex};

    pub struct Synth {
        #[allow(dead_code)]
        stream: cpal::Stream,
        producer: Producer<f32, Arc<SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>,
        sample_rate: f32,
        phase: f32,
        frequency: f32,
        amplitude: f32,
    }

    impl Synth {
        pub fn new() -> Result<Self, anyhow::Error> {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or(anyhow::anyhow!("No output device"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let ring = HeapRb::<f32>::new(8192);
            let (producer, mut consumer) = ring.split();

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Fill the buffer with samples from the ring buffer
                    for frame in data.chunks_mut(channels) {
                        let sample = consumer.pop().unwrap_or(0.0);
                        for s in frame.iter_mut() {
                            *s = sample;
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )?;
            stream.play()?;

            Ok(Self {
                stream,
                producer,
                sample_rate,
                phase: 0.0,
                frequency: 440.0,
                amplitude: 0.0,
            })
        }

        pub fn update(&mut self, freq: f32, amp: f32) {
            self.frequency = freq;
            self.amplitude = amp;

            // Just try to keep buffer full
            let capacity = self.producer.capacity();
            let len = self.producer.len();
            let available = capacity - len;

            // Generate up to 1024 samples per call to avoid blocking main thread too long if buffer is huge
            let count = available.min(1024);

            for _ in 0..count {
                self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
                let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude;
                if self.producer.push(sample).is_err() {
                    break;
                }
            }
        }
    }
}

#[cfg(not(feature = "audio"))]
pub mod audio_impl {
    pub struct Synth;
    impl Synth {
        pub fn new() -> Result<Self, anyhow::Error> {
            Ok(Self)
        }
        pub fn update(&mut self, _freq: f32, _amp: f32) {}
    }
}

pub use audio_impl::Synth;
