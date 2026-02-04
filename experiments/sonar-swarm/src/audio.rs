use crate::physics::AcousticGrid;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{Receiver, Sender};

pub enum AudioCommand {
    Pluck(usize, usize, f32),
    AddWall(usize, usize),
    RemoveWall(usize, usize),
    ClearWalls,
}

pub struct AudioModel {
    grid: AcousticGrid,
    command_rx: Receiver<AudioCommand>,
    snapshot_tx: Sender<Vec<f32>>,
    samples_since_snapshot: usize,
    snapshot_interval: usize,
}

impl AudioModel {
    pub fn new(
        width: usize,
        height: usize,
        command_rx: Receiver<AudioCommand>,
        snapshot_tx: Sender<Vec<f32>>,
    ) -> Self {
        Self {
            grid: AcousticGrid::new(width, height),
            command_rx,
            snapshot_tx,
            samples_since_snapshot: 0,
            snapshot_interval: 735, // ~60Hz at 44.1kHz
        }
    }

    pub fn process(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            // Process commands
            while let Ok(cmd) = self.command_rx.try_recv() {
                match cmd {
                    AudioCommand::Pluck(x, y, s) => self.grid.pluck(x, y, s),
                    AudioCommand::AddWall(x, y) => self.grid.add_wall(x, y),
                    AudioCommand::RemoveWall(x, y) => self.grid.remove_wall(x, y),
                    AudioCommand::ClearWalls => self.grid.walls.fill(false),
                }
            }

            // Step Physics
            self.grid.step();

            // Sample Output - just listen to center for now?
            // Or maybe listen to average pressure?
            // Let's listen to center.
            let p = self.grid.get(self.grid.width / 2, self.grid.height / 2);
            *sample = p.clamp(-1.0, 1.0);

            // Handle Snapshot
            self.samples_since_snapshot += 1;
            if self.samples_since_snapshot >= self.snapshot_interval {
                self.samples_since_snapshot = 0;
                // Send a copy of the pressure field
                // Use try_send to drop if full (UI is slow)
                let _ = self.snapshot_tx.try_send(self.grid.pressure.clone());
            }
        }
    }
}

pub fn run_audio(
    width: usize,
    height: usize,
    command_rx: Receiver<AudioCommand>,
    snapshot_tx: Sender<Vec<f32>>,
) -> Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;
    let sample_format = config.sample_format();
    let config: cpal::StreamConfig = config.into();

    let mut model = AudioModel::new(width, height, command_rx, snapshot_tx);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;
    Ok(stream)
}
