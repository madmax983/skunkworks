use crate::neuron::Izhikevich;
use ringbuf::{Consumer, HeapRb};
use std::sync::Arc;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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

#[allow(dead_code)] // The backend is held to keep the stream/thread alive
enum AudioBackend {
    #[cfg(feature = "audio")]
    Stream(cpal::Stream),
    #[cfg(not(feature = "audio"))]
    Thread(std::thread::JoinHandle<()>),
}

pub struct AudioEngine {
    _backend: AudioBackend,
    pub hit_tx: crossbeam_channel::Sender<NeuronHit>,
    pub snapshot_rx: Consumer<Snapshot, Arc<HeapRb<Snapshot>>>,
}

impl AudioEngine {
    pub fn new(neuron_count: usize) -> anyhow::Result<Self> {
        let (hit_tx, hit_rx) = crossbeam_channel::unbounded::<NeuronHit>();

        let rb_snapshot = HeapRb::<Snapshot>::new(16);
        let (mut snapshot_tx, mut snapshot_rx) = rb_snapshot.split();

        let mut neurons: Vec<Izhikevich> = Vec::with_capacity(neuron_count);
        let mut rng = rand::thread_rng();
        for _ in 0..neuron_count {
            neurons.push(Izhikevich::random(&mut rng));
        }

        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow::anyhow!("no output device available"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

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
                            let (v, _) = neuron.update(dt, 0.0);
                            sum_v += v;
                        }
                        let mean_field = sum_v / neurons.len() as f32;

                        // Output sound
                        let sample = ((mean_field + 40.0) / 80.0).clamp(-0.8, 0.8);

                        for sample_out in frame.iter_mut() {
                            *sample_out = sample;
                        }

                        // Snapshot logic
                        snapshot_timer += 1;
                        if snapshot_timer > 700 {
                            // ~60Hz
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
                _backend: AudioBackend::Stream(stream),
                hit_tx,
                snapshot_rx,
            })
        }

        #[cfg(not(feature = "audio"))]
        {
            let handle = std::thread::spawn(move || {
                let sample_rate = 44100.0;
                let time_scale = 10.0;
                let dt = (1000.0 / sample_rate) * time_scale;
                let mut snapshot_timer = 0;

                // Simulate roughly 44100Hz
                let frame_duration =
                    std::time::Duration::from_micros((1_000_000.0 / sample_rate * 256.0) as u64); // processing in chunks of 256

                loop {
                    let start = std::time::Instant::now();

                    // Simulate a chunk of 256 samples
                    for _ in 0..256 {
                        while let Ok(hit) = hit_rx.try_recv() {
                            if hit.index < neurons.len() {
                                neurons[hit.index].inject(hit.strength);
                            }
                        }

                        let mut sum_v = 0.0;
                        for neuron in &mut neurons {
                            let (v, _) = neuron.update(dt, 0.0);
                            sum_v += v;
                        }
                        let mean_field = sum_v / neurons.len() as f32;

                        snapshot_timer += 1;
                        if snapshot_timer > 700 {
                            snapshot_timer = 0;
                            let voltages: Vec<f32> = neurons.iter().map(|n| n.v).collect();
                            let _ = snapshot_tx.push(Snapshot {
                                voltages,
                                mean_field,
                            });
                        }
                    }

                    // Sleep to maintain timing roughly
                    let elapsed = start.elapsed();
                    if frame_duration > elapsed {
                        std::thread::sleep(frame_duration - elapsed);
                    }
                }
            });

            Ok(Self {
                _backend: AudioBackend::Thread(handle),
                hit_tx,
                snapshot_rx,
            })
        }
    }
}
