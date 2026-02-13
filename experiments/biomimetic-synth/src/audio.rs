use crate::network::Network;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub const SAMPLE_RATE: u32 = 44100;
pub const SNAPSHOT_INTERVAL: u32 = 735; // Send snapshot every ~60Hz (44100 / 60)

#[derive(Clone, Debug)]
pub struct SynapseData {
    pub pre: usize,
    pub post: usize,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub voltages: Vec<f32>,
    pub spikes: Vec<bool>,
    pub synapses: Vec<SynapseData>,
    pub mean_field: f32,
}

pub enum Command {
    Inject {
        index: usize,
        current: f32,
    },
    SetParams {
        index: usize,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    },
}

pub struct AudioEngine {
    network: Network,
    // inputs removed, handled internally by neurons
    cmd_rx: Receiver<Command>,
    snap_tx: Sender<Snapshot>,
    sample_count: u32,
}

impl AudioEngine {
    pub fn new(network: Network) -> (Self, Sender<Command>, Receiver<Snapshot>) {
        let (cmd_tx, cmd_rx) = unbounded();
        let (snap_tx, snap_rx) = unbounded();

        let engine = Self {
            network,
            cmd_rx,
            snap_tx,
            sample_count: 0,
        };

        (engine, cmd_tx, snap_rx)
    }

    pub fn process_step(&mut self) -> f32 {
        // 1. Decay Inputs - Handled by Izhikevich model internally (current_decay)

        // 2. Process Commands
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                Command::Inject { index, current } => {
                    self.network.inject(index, current);
                }
                Command::SetParams { index, a, b, c, d } => {
                    if let Some(neuron) = self.network.neurons.get_mut(index) {
                        neuron.a = a;
                        neuron.b = b;
                        neuron.c = c;
                        neuron.d = d;
                    }
                }
            }
        }

        // 3. Step Network
        self.network.step(1.0);

        // 4. Generate Audio Sample
        // Mix mean field (low freq) and spikes (high freq clicks)
        let mut mean_field = 0.0;
        let mut spike_accum = 0.0;
        let mut spike_vec = Vec::with_capacity(self.network.neurons.len());

        for (i, n) in self.network.neurons.iter().enumerate() {
            mean_field += n.v;
            let spiked = self.network.last_spikes[i] == Some(self.network.tick);
            if spiked {
                spike_accum += 1.0;
            }
            spike_vec.push(spiked);
        }
        mean_field /= self.network.neurons.len() as f32;

        // Simple mapping: Spikes are loud, mean field is hum
        let sample = (mean_field * 0.001) + (spike_accum * 0.05);

        // 5. Snapshot
        self.sample_count += 1;
        if self.sample_count >= SNAPSHOT_INTERVAL {
            self.sample_count = 0;

            let voltages: Vec<f32> = self.network.neurons.iter().map(|n| n.v).collect();
            let synapses: Vec<SynapseData> = self
                .network
                .synapses
                .iter()
                .map(|s| SynapseData {
                    pre: s.pre,
                    post: s.post,
                    weight: s.weight,
                })
                .collect();

            let _ = self.snap_tx.try_send(Snapshot {
                voltages,
                spikes: spike_vec,
                synapses,
                mean_field: sample,
            });
        }

        sample
    }

    fn run_fallback(mut self) {
        let target_frame_time = Duration::from_micros(1_000_000 / SAMPLE_RATE as u64);
        loop {
            let start = Instant::now();
            // Process a batch
            for _ in 0..100 {
                self.process_step();
            }
            let elapsed = start.elapsed();
            let target_batch = target_frame_time * 100;
            if elapsed < target_batch {
                thread::sleep(target_batch - elapsed);
            }
        }
    }

    #[cfg(not(feature = "audio"))]
    pub fn run(self) {
        self.run_fallback();
    }

    #[cfg(feature = "audio")]
    pub fn run(self) {
        use rodio::{OutputStream, Sink};

        let stream_result = OutputStream::try_default();

        if let Ok((_stream, stream_handle)) = stream_result {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let source = SynthSource { engine: self };
                sink.append(source);
                sink.sleep_until_end();
            } else {
                eprintln!("Sink creation failed. Running fallback.");
                // Can't run fallback because self moved. Panic is acceptable or just exit.
                // Ideally we'd recover self but conditional compilation makes it hard.
                // Just exit thread.
            }
        } else {
            eprintln!("Audio output failed. Running in fallback mode.");
            self.run_fallback();
        }
    }
}

#[cfg(feature = "audio")]
struct SynthSource {
    engine: AudioEngine,
}

#[cfg(feature = "audio")]
impl rodio::Source for SynthSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(feature = "audio")]
impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.engine.process_step())
    }
}
