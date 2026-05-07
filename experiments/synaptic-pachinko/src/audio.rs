use crate::neuron::Izhikevich;
use ringbuf::{Consumer, HeapRb};
use std::sync::Arc;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Clone, Copy, Debug)]
/// A physical collision event translated into neural energy.
///
/// When a falling [`crate::physics::PacketKind`] strikes a [`crate::physics::NeuronPin`] in the physics simulation,
/// it generates a `NeuronHit`. The audio engine consumes these events, converting
/// the physical kinetic energy into a sudden spike of voltage directed at the corresponding Izhikevich neuron.
///
/// ## Examples
/// ```
/// use synaptic_pachinko::audio::NeuronHit;
///
/// let impact = NeuronHit {
///     index: 42,
///     strength: 15.0,
/// };
/// assert_eq!(impact.strength, 15.0);
/// ```
pub struct NeuronHit {
    /// The index of the struck neuron.
    pub index: usize,
    /// The charge/impact strength deposited to the neuron.
    pub strength: f32,
}

#[derive(Clone, Debug)]
/// A frozen moment in time capturing the electrical potential of the entire network.
///
/// Because the audio synthesis engine runs at high frequency in a background thread,
/// the main UI thread periodically consumes a `Snapshot` to synchronize the visual
/// representations of the [`crate::physics::NeuronPin`]s without blocking the audio simulation.
///
/// ## Examples
/// ```
/// use synaptic_pachinko::audio::Snapshot;
///
/// let state = Snapshot {
///     voltages: vec![-65.0, -40.0, 30.0], // The third neuron is spiking!
///     mean_field: -25.0,
/// };
/// assert_eq!(state.voltages.len(), 3);
/// ```
pub struct Snapshot {
    /// Voltages for all tracked neurons.
    pub voltages: Vec<f32>,
    /// The mean field voltage (average potential across the network).
    pub mean_field: f32,
}

#[allow(dead_code)] // The backend is held to keep the stream/thread alive
enum AudioBackend {
    #[cfg(feature = "audio")]
    Stream(cpal::Stream),
    #[cfg(not(feature = "audio"))]
    Thread(std::thread::JoinHandle<()>),
}

/// The maestro orchestrating the symphony of spiking neurons.
///
/// `AudioEngine` runs in a dedicated background thread (or CPAL audio callback),
/// continuously stepping the differential equations of a network of Izhikevich neurons.
/// It listens for incoming [`NeuronHit`] events from the physics thread, injects charge,
/// and synthesizes an audio waveform from the resulting action potentials.
///
/// It also periodically publishes a [`Snapshot`] of the network's internal voltages
/// back to the main thread for rendering.
///
/// ## Examples
/// ```no_run
/// use synaptic_pachinko::audio::AudioEngine;
///
/// // Create an engine simulating a network of 100 pins
/// let engine = AudioEngine::new(100).expect("Failed to open audio device");
///
/// // Send a hit event to the 5th neuron
/// engine.hit_tx.send(synaptic_pachinko::audio::NeuronHit {
///     index: 5,
///     strength: 20.0,
/// }).unwrap();
/// ```
pub struct AudioEngine {
    _backend: AudioBackend,
    /// Channel sender to send collision impacts to the audio thread.
    pub hit_tx: crossbeam_channel::Sender<NeuronHit>,
    /// Buffer consumer containing the latest network voltage state.
    pub snapshot_rx: Consumer<Snapshot, Arc<HeapRb<Snapshot>>>,
}

impl AudioEngine {
    /// Initializes the audio engine, starting background synthesis threads.
    ///
    /// ## Arguments
    ///
    /// * `neuron_count` - The number of neurons (pins) to simulate.
    pub fn new(neuron_count: usize) -> anyhow::Result<Self> {
        let (hit_tx, hit_rx) = crossbeam_channel::unbounded::<NeuronHit>();

        let rb_snapshot = HeapRb::<Snapshot>::new(16);
        let (mut snapshot_tx, snapshot_rx) = rb_snapshot.split();

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
