use crate::neuron::{HodgkinHuxley, Command};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, Consumer, Producer, SharedRb};
use std::sync::Arc;
use anyhow::Result;

pub struct AudioSystem {
    // We hold the stream to keep it alive
    #[allow(dead_code)]
    pub stream: cpal::Stream,

    // Send commands to audio thread
    pub command_tx: Producer<Command, Arc<SharedRb<Command, Vec<std::mem::MaybeUninit<Command>>>>>,

    // Receive state snapshots for visualization
    pub waveform_rx: Consumer<f32, Arc<SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>,

    // Slower update for m/h/n bars
    pub state_rx: Consumer<HodgkinHuxley, Arc<SharedRb<HodgkinHuxley, Vec<std::mem::MaybeUninit<HodgkinHuxley>>>>>,
}

struct AudioState {
    neuron: HodgkinHuxley,
    prev_x: f32, // For DC blocker
    prev_y: f32,
    sample_counter: usize,
}

impl AudioState {
    fn new() -> Self {
        Self {
            neuron: HodgkinHuxley::new(),
            prev_x: 0.0,
            prev_y: 0.0,
            sample_counter: 0,
        }
    }
}

pub fn init_audio() -> Result<AudioSystem> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No output device"))?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // Ring buffers
    let cmd_rb = HeapRb::<Command>::new(1024);
    let (cmd_prod, mut cmd_cons) = cmd_rb.split();

    let wave_rb = HeapRb::<f32>::new(4096);
    let (mut wave_prod, wave_cons) = wave_rb.split();

    let state_rb = HeapRb::<HodgkinHuxley>::new(128);
    let (mut state_prod, state_cons) = state_rb.split();

    let dt = 1000.0 / sample_rate; // ms

    let mut state = AudioState::new();

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                // 1. Process Commands
                while let Some(cmd) = cmd_cons.pop() {
                    state.neuron.apply_command(cmd);
                }

                // 2. Step Physics
                state.neuron.step(dt);

                // 3. Signal Processing (DC Blocker + Gain)
                let raw_v = state.neuron.v;
                let x = raw_v;
                let y = x - state.prev_x + 0.995 * state.prev_y;
                state.prev_x = x;
                state.prev_y = y;

                // Soft clip for safety
                let output_sample = (y * 0.02).tanh();

                // 4. Output to Buffer
                for sample in frame.iter_mut() {
                    *sample = output_sample;
                }

                // 5. Output to Visualization
                // Push waveform
                let _ = wave_prod.push(raw_v); // Raw V is better for oscilloscope

                // Push state occasionally (every ~500 samples ~ 10ms)
                state.sample_counter += 1;
                if state.sample_counter >= 500 {
                    let _ = state_prod.push(state.neuron.clone());
                    state.sample_counter = 0;
                }
            }
        },
        |err| eprintln!("Audio error: {}", err),
        None // Timeout
    )?;

    stream.play()?;

    Ok(AudioSystem {
        stream,
        command_tx: cmd_prod,
        waveform_rx: wave_cons,
        state_rx: state_cons,
    })
}
