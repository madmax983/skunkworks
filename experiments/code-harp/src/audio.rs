use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Debug)]
pub struct PluckEvent {
    pub frequency: f32,
    pub damping: f32, // 0.0 - 1.0 (1.0 = sustain forever)
    pub gain: f32,
}

struct KarplusStrong {
    buffer: Vec<f32>,
    index: usize,
    damping: f32,
    val_prev: f32,
}

impl KarplusStrong {
    fn new(sample_rate: f32, frequency: f32, damping: f32) -> Self {
        let period = sample_rate / frequency;
        let len = period.round().max(2.0) as usize;
        let mut buffer = vec![0.0; len];

        // Initial burst (white noise)
        // We do this once on creation (pluck)
        for i in 0..len {
            buffer[i] = (rand::random::<f32>() * 2.0 - 1.0);
        }

        Self {
            buffer,
            index: 0,
            damping,
            val_prev: 0.0,
        }
    }

    fn tick(&mut self) -> f32 {
        let current_val = self.buffer[self.index];
        let next_index = (self.index + 1) % self.buffer.len();

        // Karplus-Strong algorithm:
        // y[n] = 0.5 * (y[n-L] + y[n-L-1])
        // Here, buffer holds y[n-L] .. y[n-1].
        // We overwrite the oldest sample with the new one.

        // Simple averaging lowpass
        let avg = (current_val + self.val_prev) * 0.5;
        let new_val = avg * self.damping;

        self.val_prev = current_val;
        self.buffer[self.index] = new_val;
        self.index = next_index;

        current_val
    }

    fn is_silent(&self) -> bool {
        // Approximate silence check
        // If the energy is very low, we can kill it.
        // But checking whole buffer is slow.
        // Just checking current sample is unreliable (zero crossing).
        // Let's rely on a max lifetime or external management.
        false
    }
}

pub fn init() -> anyhow::Result<Sender<PluckEvent>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device"))?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let (sender, receiver) = unbounded::<PluckEvent>();

    // We send the stream to a background thread or just let it live?
    // cpal stream is Send.
    // But we need to keep it alive.
    // For this experiment, we'll box it and leak it, or return a handle.
    // Returning a handle is cleaner but I need to define a struct.
    // Let's use a static or just thread::spawn approach?
    // No, cpal handles threads. We just need to keep the `Stream` object alive.

    std::thread::spawn(move || {
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                run::<f32>(&device, &config.into(), receiver.clone(), sample_rate)
            }
            cpal::SampleFormat::I16 => {
                run::<i16>(&device, &config.into(), receiver.clone(), sample_rate)
            }
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), receiver, sample_rate),
            _ => panic!("Unsupported sample format"),
        };

        if let Ok(_s) = stream {
            // Keep the stream alive forever
            std::thread::park();
        } else {
            eprintln!("Failed to create audio stream");
        }
    });

    Ok(sender)
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    receiver: Receiver<PluckEvent>,
    sample_rate: f32,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: cpal::Sample + cpal::FromSample<f32> + SizedSample,
{
    let mut voices: Vec<KarplusStrong> = Vec::with_capacity(32);
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Check for new plucks
            while let Ok(event) = receiver.try_recv() {
                // Limit polyphony
                if voices.len() >= 32 {
                    voices.remove(0); // Remove oldest
                }
                voices.push(KarplusStrong::new(
                    sample_rate,
                    event.frequency,
                    event.damping,
                ));
            }

            // Fill buffer
            for frame in data.chunks_mut(channels) {
                let mut mix = 0.0;

                for voice in voices.iter_mut() {
                    mix += voice.tick();
                }

                // Soft clip
                mix = mix.tanh();

                // Convert to output type
                let sample: T = T::from_sample(mix * 0.5); // 0.5 master volume

                for s in frame.iter_mut() {
                    *s = sample;
                }
            }
        },
        |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    stream.play()?;
    Ok(stream)
}
