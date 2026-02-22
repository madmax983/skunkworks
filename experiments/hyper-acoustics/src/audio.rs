use crate::grid::{PhysicsGrid4D, Point4D, GRID_SIZE};
use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub enum AudioCommand {
    Pluck {
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        strength: f32,
    },
    SetParams {
        c2: f32,
        damping: f32,
    },
}

pub struct AudioSnapshot {
    pub u: Vec<f32>,
}

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    #[cfg(not(feature = "audio"))]
    _thread: std::thread::JoinHandle<()>,
}

fn process_commands(
    grid: &mut PhysicsGrid4D,
    cmd_rx: &Receiver<AudioCommand>,
    c2: &mut f32,
    damping: &mut f32,
) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            AudioCommand::Pluck {
                x,
                y,
                z,
                w,
                strength,
            } => {
                grid.pluck(Point4D::new(x, y, z, w), strength);
            }
            AudioCommand::SetParams {
                c2: new_c2,
                damping: new_damping,
            } => {
                *c2 = new_c2;
                *damping = new_damping;
            }
        }
    }
}

pub fn init_audio(
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<AudioSnapshot>,
) -> Result<AudioSystem> {
    let mut grid = PhysicsGrid4D::new();
    let mut c2 = 0.5;
    let mut damping = 0.999;

    // Listener at center
    let center_val = GRID_SIZE / 2;
    let center = Point4D::new(center_val, center_val, center_val, center_val);

    #[cfg(not(feature = "audio"))]
    {
        let _ = center;
    }

    #[cfg(feature = "audio")]
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No audio device"))?;
        let config = device.default_output_config()?;
        let stream_config: cpal::StreamConfig = config.into();

        let mut samples_since_snapshot = 0;
        let snapshot_interval = 735;

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    process_commands(&mut grid, &cmd_rx, &mut c2, &mut damping);

                    grid.step(c2, damping);

                    let val = grid.get(center);
                    *sample = val.clamp(-1.0, 1.0);

                    samples_since_snapshot += 1;
                    if samples_since_snapshot >= snapshot_interval {
                        samples_since_snapshot = 0;
                        let _ = snap_tx.try_send(AudioSnapshot { u: grid.u.clone() });
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        Ok(AudioSystem { _stream: stream })
    }

    #[cfg(not(feature = "audio"))]
    {
        println!("Audio feature disabled. Running in simulation mode.");
        let thread = std::thread::spawn(move || {
            let sample_rate = 44100.0;
            let chunk_size = 1024;
            let snapshot_interval = 735;
            let mut samples_since_snapshot = 0;
            let mut next_tick = std::time::Instant::now();

            loop {
                // Process a chunk of samples
                for _ in 0..chunk_size {
                    process_commands(&mut grid, &cmd_rx, &mut c2, &mut damping);

                    grid.step(c2, damping);

                    samples_since_snapshot += 1;
                    if samples_since_snapshot >= snapshot_interval {
                        samples_since_snapshot = 0;
                        let _ = snap_tx.try_send(AudioSnapshot { u: grid.u.clone() });
                    }
                }

                // Sleep to maintain timing
                let duration = std::time::Duration::from_secs_f64(chunk_size as f64 / sample_rate);
                next_tick += duration;
                if let Some(sleep_time) =
                    next_tick.checked_duration_since(std::time::Instant::now())
                {
                    std::thread::sleep(sleep_time);
                } else {
                    next_tick = std::time::Instant::now(); // We are lagging, reset
                }
            }
        });

        Ok(AudioSystem { _thread: thread })
    }
}
