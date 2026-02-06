/// Shared Audio Model Logic for Resonance Experiments
use crate::physics::PhysicsGrid;
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::f32::consts::PI;

pub enum AudioCommand {
    Pluck { x: usize, y: usize, strength: f32 },
    Oscillate { x: usize, y: usize, frequency: f32, strength: f32 },
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
    /// Map of (x, y) -> (phase, frequency, strength)
    pub oscillators: HashMap<(usize, usize), (f32, f32, f32)>,
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
            oscillators: HashMap::new(),
        }
    }

    pub fn process(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            // Check commands
            while let Ok(cmd) = self.command_rx.try_recv() {
                match cmd {
                    AudioCommand::Pluck { x, y, strength } => self.grid.pluck(x, y, strength),
                    AudioCommand::Oscillate { x, y, frequency, strength } => {
                        if strength.abs() < 0.001 {
                            self.oscillators.remove(&(x, y));
                        } else {
                            // Reset phase if new? Or keep phase to avoid clicking?
                            // Let's keep phase if exists, else 0.0.
                            let entry = self.oscillators.entry((x, y)).or_insert((0.0, frequency, strength));
                            entry.1 = frequency;
                            entry.2 = strength;
                        }
                    }
                    AudioCommand::AddWall { x, y } => self.grid.add_wall(x, y),
                    AudioCommand::MoveListener { x, y } => {
                        if x < self.grid.width && y < self.grid.height {
                            self.listener_x = x;
                            self.listener_y = y;
                        }
                    }
                }
            }

            // Apply oscillators
            for ((x, y), (phase, freq, strength)) in self.oscillators.iter_mut() {
                // frequency is Hz. Sample rate assumed 44100.
                *phase += *freq * 2.0 * PI / 44100.0;
                if *phase > 2.0 * PI {
                    *phase -= 2.0 * PI;
                }
                let val = phase.sin() * *strength;

                // Inject into grid
                if *x < self.grid.width && *y < self.grid.height {
                    let idx = *y * self.grid.width + *x;
                    if !self.grid.walls[idx] {
                         self.grid.u[idx] += val;
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
            #[allow(clippy::manual_is_multiple_of)]
            if self.sample_counter % 735 == 0 {
                // Ignore error if channel is full
                let _ = self.snapshot_tx.try_send(self.grid.u.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::bounded;

    #[test]
    fn test_audio_process() {
        let (cmd_tx, cmd_rx) = bounded(10);
        let (snap_tx, snap_rx) = bounded(10);

        let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx);

        // Pluck via command
        cmd_tx
            .send(AudioCommand::Pluck {
                x: 5,
                y: 5,
                strength: 1.0,
            })
            .unwrap();

        let mut buffer = vec![0.0; 100];
        model.process(&mut buffer);

        // Check that something happened in buffer (wave propagated)
        // At start, listener is at 5,5 (width/2, height/2)
        // Pluck at 5,5.
        // First sample should be close to 1.0 (or whatever step produces).

        assert!(buffer.iter().any(|&x| x.abs() > 0.0));

        // Check snapshot was sent (maybe not, 100 < 735)
        // Let's process enough to trigger snapshot
        let mut big_buffer = vec![0.0; 1000];
        model.process(&mut big_buffer);

        assert!(snap_rx.try_recv().is_ok());
    }
}
