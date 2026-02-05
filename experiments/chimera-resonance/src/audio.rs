use crate::physics::PhysicsGrid;
use crossbeam_channel::{Receiver, Sender};

pub enum AudioCommand {
    Pluck { x: usize, y: usize, strength: f32 },
    AddWall { x: usize, y: usize },
    MoveListener { x: usize, y: usize },
}

pub struct AudioModel {
    pub grid: PhysicsGrid,
    pub listener_x: usize,
    pub listener_y: usize,
    pub command_rx: Receiver<AudioCommand>,
    pub snapshot_tx: Sender<Vec<f32>>,
    pub sample_counter: usize,
}

impl AudioModel {
    pub fn new(
        width: usize,
        height: usize,
        command_rx: Receiver<AudioCommand>,
        snapshot_tx: Sender<Vec<f32>>,
    ) -> Self {
        Self {
            grid: PhysicsGrid::new(width, height),
            listener_x: width / 2,
            listener_y: height / 2,
            command_rx,
            snapshot_tx,
            sample_counter: 0,
        }
    }

    pub fn process(&mut self, output: &mut [f32]) {
        // Iterate through requested sample buffer
        for sample in output.iter_mut() {
            // Check commands
            while let Ok(cmd) = self.command_rx.try_recv() {
                match cmd {
                    AudioCommand::Pluck { x, y, strength } => self.grid.pluck(x, y, strength),
                    AudioCommand::AddWall { x, y } => self.grid.add_wall(x, y),
                    AudioCommand::MoveListener { x, y } => {
                        if x < self.grid.width && y < self.grid.height {
                            self.listener_x = x;
                            self.listener_y = y;
                        }
                    }
                }
            }

            self.grid.step();

            // Sample at listener position
            let val = self.grid.get(self.listener_x, self.listener_y);

            // Soft clip / Tanh to prevent explosion?
            // Or just raw. Raw is "physics".
            // But let's clamp slightly to save ears.
            let clamped = val.clamp(-1.0, 1.0);
            *sample = clamped;

            self.sample_counter += 1;

            // Send snapshot at ~60Hz (Assuming 44100Hz, 735 samples)
            // Use rem_euclid or just % if we are sure positive. usize is positive.
            if self.sample_counter % 735 == 0 {
                // Ignore error if channel is full
                let _ = self.snapshot_tx.try_send(self.grid.u.clone());
            }
        }
    }
}
