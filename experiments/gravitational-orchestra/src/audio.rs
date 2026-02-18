use crate::physics::MAX_BODIES;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, Default)]
pub struct AudioBody {
    pub vel_sq: f32,
    pub mass: f32,
    pub dist_sq_from_center: f32,
}

#[derive(Clone, Copy)]
pub struct SharedState {
    pub bodies: [AudioBody; MAX_BODIES],
    pub count: usize,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            bodies: [AudioBody::default(); MAX_BODIES],
            count: 0,
        }
    }
}

pub struct AudioEngine {
    _stream: cpal::Stream,
    shared_state: Arc<Mutex<SharedState>>,
}

impl AudioEngine {
    pub fn new() -> (Self, Arc<Mutex<SharedState>>) {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config().expect("no default config");

        let shared_state = Arc::new(Mutex::new(SharedState::default()));
        let state_clone = shared_state.clone();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), state_clone),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), state_clone),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), state_clone),
            _ => panic!("Unsupported sample format"),
        }
        .expect("failed to build stream");

        stream.play().expect("failed to play stream");

        (
            AudioEngine {
                _stream: stream,
                shared_state: shared_state.clone(),
            },
            shared_state,
        )
    }
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    shared_state: Arc<Mutex<SharedState>>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut phases = [0.0; MAX_BODIES];

    device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Copy state (cheap for small fixed size struct)
            let state_snapshot = {
                let state = shared_state.lock().unwrap();
                *state
            };

            for frame in data.chunks_mut(channels) {
                let mut sample_sum: f32 = 0.0;

                for i in 0..state_snapshot.count {
                    if i >= MAX_BODIES {
                        break;
                    }
                    let body = state_snapshot.bodies[i];

                    // Frequency mapping
                    let v = body.vel_sq.sqrt();
                    let freq_base = 50.0 + v * 2.0;
                    let freq: f32 = freq_base.clamp(50.0, 2000.0);

                    // Amplitude mapping
                    let dist = body.dist_sq_from_center.sqrt().max(10.0);
                    let amp = (body.mass.sqrt() * 50.0 / dist).clamp(0.0, 0.5);

                    phases[i] = (phases[i] + freq / sample_rate) % 1.0;

                    let wave = (phases[i] * 2.0 * std::f32::consts::PI).sin();

                    sample_sum += wave * amp;
                }

                // Master limiter
                let final_sample: f32 = (sample_sum * 0.5).tanh();

                let value = T::from_sample(final_sample);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None,
    )
}
