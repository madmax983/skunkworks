use crate::population::Population;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Consumer, HeapRb, Producer};
use std::sync::Arc;

pub const POP_SIZE: usize = 100;

#[derive(Clone, Copy, Debug)]
pub struct ControlParams {
    pub current: f32,
    pub coupling: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub voltages: [f32; POP_SIZE],
    pub mean_field: f32,
}

pub struct AudioEngine {
    pub _stream: cpal::Stream,
    pub control_tx: Producer<ControlParams, Arc<HeapRb<ControlParams>>>,
    pub snapshot_rx: Consumer<Snapshot, Arc<HeapRb<Snapshot>>>,
}

impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config()?;

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        // Ring Buffers
        let rb_control = HeapRb::<ControlParams>::new(16);
        let (mut control_tx, mut control_rx) = rb_control.split();

        let rb_snapshot = HeapRb::<Snapshot>::new(16);
        let (mut snapshot_tx, mut snapshot_rx) = rb_snapshot.split();

        // Audio State
        let mut population = Population::new(POP_SIZE);
        let mut current_params = ControlParams {
            current: 0.0,
            coupling: 0.0,
        };

        // Time scaling: Real neurons fire ~10-100Hz.
        // We speed up 20x to make it audible and richer.
        let time_scale = 20.0;
        let dt = (1000.0 / sample_rate) * time_scale;

        let mut snapshot_timer = 0;

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Update params
                while let Some(p) = control_rx.pop() {
                    current_params = p;
                }

                population.coupling_strength = current_params.coupling;

                for frame in data.chunks_mut(channels) {
                    let val = population.update(dt, current_params.current);

                    // Normalize output: -100mV to +30mV -> -1.0 to 1.0 roughly
                    // Range ~ 130mV. Center ~ -35mV?
                    // We compress it a bit to avoid clipping
                    let sample = ((val + 40.0) / 80.0).clamp(-0.8, 0.8);

                    for sample_out in frame.iter_mut() {
                        *sample_out = sample;
                    }

                    // Snapshot logic
                    snapshot_timer += 1;
                    if snapshot_timer > 700 {
                        // Approx 60Hz
                        snapshot_timer = 0;
                        let mut voltages = [0.0; POP_SIZE];
                        for (i, n) in population.neurons.iter().enumerate() {
                            if i < POP_SIZE {
                                voltages[i] = n.v;
                            }
                        }
                        // Only push if space available
                        let _ = snapshot_tx.push(Snapshot {
                            voltages,
                            mean_field: val,
                        });
                    }
                }
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?;

        stream.play()?;

        Ok(Self {
            _stream: stream,
            control_tx,
            snapshot_rx,
        })
    }
}
