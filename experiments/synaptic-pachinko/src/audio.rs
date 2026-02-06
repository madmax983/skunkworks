use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, Consumer};
use std::sync::Arc;
use crate::neuron::Izhikevich;

#[derive(Clone, Copy, Debug)]
pub struct NeuronHit {
    pub index: usize,
    pub strength: f32,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub voltages: Vec<f32>,
    pub mean_field: f32,
}

pub struct AudioEngine {
    pub _stream: cpal::Stream,
    pub hit_tx: crossbeam_channel::Sender<NeuronHit>,
    pub snapshot_rx: Consumer<Snapshot, Arc<HeapRb<Snapshot>>>,
}

impl AudioEngine {
    pub fn new(neuron_count: usize) -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let (hit_tx, hit_rx) = crossbeam_channel::unbounded::<NeuronHit>();

        let rb_snapshot = HeapRb::<Snapshot>::new(16);
        let (mut snapshot_tx, mut snapshot_rx) = rb_snapshot.split();

        let mut neurons: Vec<Izhikevich> = Vec::with_capacity(neuron_count);
        let mut rng = rand::thread_rng();
        for _ in 0..neuron_count {
            neurons.push(Izhikevich::random(&mut rng));
        }

        let time_scale = 10.0;
        let dt = (1000.0 / sample_rate) * time_scale;
        let mut snapshot_timer = 0;

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process inputs
                while let Ok(hit) = hit_rx.try_recv() {
                    if hit.index < neurons.len() {
                        neurons[hit.index].inject(hit.strength);
                    }
                }

                for frame in data.chunks_mut(channels) {
                    let mut sum_v = 0.0;
                    for neuron in &mut neurons {
                        sum_v += neuron.update(dt);
                    }
                    let mean_field = sum_v / neurons.len() as f32;

                    // Output sound
                    let sample = ((mean_field + 40.0) / 80.0).clamp(-0.8, 0.8);

                    for sample_out in frame.iter_mut() {
                        *sample_out = sample;
                    }

                    // Snapshot logic
                    snapshot_timer += 1;
                    if snapshot_timer > 700 { // ~60Hz
                        snapshot_timer = 0;
                        let voltages: Vec<f32> = neurons.iter().map(|n| n.v).collect();
                        let _ = snapshot_tx.push(Snapshot {
                            voltages,
                            mean_field,
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
            hit_tx,
            snapshot_rx,
        })
    }
}
