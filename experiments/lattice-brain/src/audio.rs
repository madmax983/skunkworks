use crossbeam_channel::{unbounded, Receiver, Sender};
use synaptic_physics::Izhikevich;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use ringbuf::{Consumer, HeapRb, SharedRb};
#[cfg(feature = "audio")]
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub voltages: Vec<f32>,
    pub mean_field: f32,
}

pub enum AudioCommand {
    UpdateNetwork {
        neurons: Vec<Izhikevich>,
        connections: Vec<Vec<(usize, f32)>>,
    },
    Inject {
        index: usize,
        current: f32,
    },
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<cpal::Stream>,
    pub cmd_tx: Sender<AudioCommand>,
    #[cfg(feature = "audio")]
    pub snapshot_rx: Consumer<Snapshot, Arc<SharedRb<Snapshot, Vec<std::mem::MaybeUninit<Snapshot>>>>>,
    #[cfg(not(feature = "audio"))]
    pub snapshot_rx: Receiver<Snapshot>, // Dummy receiver for non-audio mode
}

impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let (cmd_tx, cmd_rx) = unbounded::<AudioCommand>();

        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = match host.default_output_device() {
                Some(d) => d,
                None => {
                    return Ok(Self::dummy(cmd_tx));
                }
            };

            let config = match device.default_output_config() {
                Ok(c) => c,
                Err(_) => return Ok(Self::dummy(cmd_tx)),
            };

            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let rb = HeapRb::<Snapshot>::new(16);
            let (mut snapshot_tx, snapshot_rx) = rb.split();

            let mut neurons: Vec<Izhikevich> = Vec::new();
            let mut connections: Vec<Vec<(usize, f32)>> = Vec::new();

            let dt = 1000.0 / sample_rate;
            let mut snapshot_timer = 0;

            let stream_res = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Process commands
                    while let Ok(cmd) = cmd_rx.try_recv() {
                        match cmd {
                            AudioCommand::UpdateNetwork { neurons: n, connections: c } => {
                                neurons = n;
                                connections = c;
                            }
                            AudioCommand::Inject { index, current } => {
                                if index < neurons.len() {
                                    neurons[index].inject(current);
                                }
                            }
                        }
                    }

                    if neurons.is_empty() {
                        for sample in data.iter_mut() { *sample = 0.0; }
                        return;
                    }

                    for frame in data.chunks_mut(channels) {
                        let mut mean_field = 0.0;

                        // Physics Step
                        for neuron in neurons.iter_mut() {
                            let _fired = neuron.update(dt, 0.0);
                            mean_field += neuron.v;
                        }

                        // Simple Mean Field
                        if !neurons.is_empty() {
                             mean_field /= neurons.len() as f32;
                        }

                        let sample = ((mean_field + 65.0) / 100.0).clamp(-0.8, 0.8);

                        for sample_out in frame.iter_mut() {
                            *sample_out = sample;
                        }

                        // Snapshot
                        snapshot_timer += 1;
                        if snapshot_timer > 735 { // ~60Hz at 44100
                            snapshot_timer = 0;
                            let voltages: Vec<f32> = neurons.iter().map(|n| n.v).collect();
                            let _ = snapshot_tx.push(Snapshot { voltages, mean_field });
                        }
                    }
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            );

            match stream_res {
                Ok(stream) => {
                     stream.play().ok();
                     Ok(Self {
                        _stream: Some(stream),
                        cmd_tx,
                        snapshot_rx,
                    })
                }
                Err(_) => Ok(Self::dummy(cmd_tx)),
            }
        }

        #[cfg(not(feature = "audio"))]
        {
            Ok(Self::dummy(cmd_tx))
        }
    }

    #[cfg(feature = "audio")]
    fn dummy(cmd_tx: Sender<AudioCommand>) -> Self {
         let rb = HeapRb::<Snapshot>::new(1);
         let (_, snapshot_rx) = rb.split();
         Self {
             _stream: None,
             cmd_tx,
             snapshot_rx,
         }
    }

    #[cfg(not(feature = "audio"))]
    fn dummy(cmd_tx: Sender<AudioCommand>) -> Self {
         let (_, snapshot_rx) = unbounded();
         Self {
             cmd_tx,
             snapshot_rx,
         }
    }
}
