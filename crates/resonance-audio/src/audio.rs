//! # Resonance Audio Model
//!
//! Provides the top-level interface for interacting with the acoustic physics grid.
//!
//! The [`AudioModel`] coordinates the physical simulation, receives asynchronous commands
//! (such as plucks or material changes) from the main thread, and generates the final
//! audio output samples.

/// Shared Audio Model Logic for Resonance Experiments
use crate::physics::{Material, PhysicsGrid};
use crossbeam_channel::{Receiver, Sender};
use std::f32::consts::PI;

/// A snapshot of the simulation state for visualization.
#[derive(Clone, Debug)]
///
/// # Examples
///
/// ```
/// use resonance_audio::AudioSnapshot;
/// let snapshot = AudioSnapshot {
///     pressure: vec![0.0; 100],
///     materials: vec![],
///     energy: vec![0.0; 100],
/// };
/// ```
pub struct AudioSnapshot {
    /// A flat map of acoustic pressure across the grid.
    pub pressure: Vec<f32>,
    /// A flat map of the materials occupying each cell of the grid.
    pub materials: Vec<Material>,
    /// A flat map of acoustic energy values indicating wave intensities.
    pub energy: Vec<f32>,
}

/// Commands to control the audio simulation state.
///
/// These commands are typically sent from the main thread (UI/Input) to the audio thread.
#[derive(Debug, Clone)]
///
/// # Examples
///
/// ```
/// use resonance_audio::AudioCommand;
/// let cmd = AudioCommand::Pluck { x: 5, y: 5, strength: 1.0 };
/// ```
pub enum AudioCommand {
    /// Injects a sudden burst of energy at the specified coordinates.
    ///
    /// This acts like plucking a string or striking a drum at a specific point.
    Pluck {
        /// The X coordinate of the pluck.
        x: usize,
        /// The Y coordinate of the pluck.
        y: usize,
        /// The intensity of the pluck (amplitude).
        strength: f32,
    },
    /// Adds or updates a continuous oscillator at the specified coordinates.
    ///
    /// The oscillator will inject energy into the grid at the given frequency and strength
    /// on every simulation step until it is removed (by setting strength to 0).
    Oscillate {
        /// The X coordinate of the oscillator.
        x: usize,
        /// The Y coordinate of the oscillator.
        y: usize,
        /// The frequency of the oscillation in Hz.
        frequency: f32,
        /// The amplitude of the oscillation. Set to 0.0 to remove the oscillator.
        strength: f32,
    },
    /// Plays a tone for a specific duration.
    ///
    /// This is a "fire and forget" version of `Oscillate` that automatically removes itself
    /// after `duration_ms`.
    Tone {
        /// The X coordinate of the source.
        x: usize,
        /// The Y coordinate of the source.
        y: usize,
        /// The frequency of the tone in Hz.
        frequency: f32,
        /// The amplitude of the tone.
        strength: f32,
        /// The duration of the tone in milliseconds.
        duration_ms: u64,
    },
    /// Places a reflective wall at the specified coordinates.
    ///
    /// Waves will reflect off this cell, and no energy will pass through it.
    AddWall {
        /// The X coordinate of the wall.
        x: usize,
        /// The Y coordinate of the wall.
        y: usize,
    },
    /// Removes a wall from the specified coordinates, allowing waves to pass through again.
    RemoveWall {
        /// The X coordinate to clear.
        x: usize,
        /// The Y coordinate to clear.
        y: usize,
    },
    /// Paints a material at the specified coordinates.
    PaintMaterial {
        /// The X coordinate to paint.
        x: usize,
        /// The Y coordinate to paint.
        y: usize,
        /// The `Material` to assign to the given grid cell.
        material: Material,
    },
    /// Instantly zeroes out all energy in the simulation grid.
    ClearWaves,
    /// Removes all walls from the simulation grid.
    ClearWalls,
    /// Moves the listener to a new position.
    ///
    /// The listener's position determines where the audio samples are read from the grid.
    MoveListener {
        /// The new X coordinate of the listener.
        x: usize,
        /// The new Y coordinate of the listener.
        y: usize,
    },
}

/// A continuous oscillator that injects energy into the grid.
#[derive(Debug, Clone)]
///
/// # Examples
///
/// ```
/// use resonance_audio::Oscillator;
/// let osc = Oscillator {
///     x: 5,
///     y: 5,
///     idx: 0,
///     frequency: 440.0,
///     strength: 1.0,
///     phase: 0.0,
/// };
/// ```
pub struct Oscillator {
    /// The X coordinate.
    pub x: usize,
    /// The Y coordinate.
    pub y: usize,
    /// The precomputed grid index.
    pub idx: usize,
    /// The current phase of the oscillator.
    pub phase: f32,
    /// The frequency in Hz.
    pub frequency: f32,
    /// The amplitude.
    pub strength: f32,
}

/// The main audio simulation engine.
///
/// This struct runs on the audio thread and manages the physics grid, processes commands,
/// and generates audio samples.
///
/// # Examples
///
/// ```
/// use resonance_audio::AudioModel;
/// use crossbeam_channel::bounded;
/// let (cmd_tx, cmd_rx) = bounded(128);
/// let (snap_tx, snap_rx) = bounded(1);
/// let model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);
/// ```
pub struct AudioModel {
    /// The underlying physics simulation grid.
    pub grid: PhysicsGrid,
    /// The X coordinate of the listener (microphone).
    pub listener_x: usize,
    /// The Y coordinate of the listener (microphone).
    pub listener_y: usize,
    /// Receiver for incoming commands from the main thread.
    pub command_rx: Receiver<AudioCommand>,
    /// Sender for simulation snapshots (for visualization).
    pub snapshot_tx: Sender<AudioSnapshot>,
    /// Optional sender for raw audio samples (for analysis/recording).
    pub recording_tx: Option<Sender<Vec<f32>>>,
    /// Counter for generated samples, used for snapshot timing.
    pub sample_counter: usize,
    /// Active continuous oscillators.
    pub oscillators: Vec<Oscillator>,
    /// Active transient tones: List of (x, y, freq, strength, remaining_samples, phase).
    pub active_tones: Vec<(usize, usize, f32, f32, usize, f32)>,
}

impl AudioModel {
    /// Creates a new `AudioModel`.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the simulation grid.
    /// * `height` - The height of the simulation grid.
    /// * `command_rx` - The channel to receive commands like plucking or placing walls.
    /// * `snapshot_tx` - The channel to send telemetry back to the main thread (for rendering ripples).
    /// * `recording_tx` - Optional: If provided, raw audio buffers are duplicated here (e.g., for saving to `.wav`).
    ///
    /// # Examples
    ///
    /// ```
    /// use resonance_audio::{AudioModel, AudioCommand};
    /// use crossbeam_channel::bounded;
    ///
    /// // Set up the communication "nerve fibers" between threads.
    /// let (brain_to_ear, command_rx) = bounded(10);
    /// let (snapshot_tx, ear_to_eye) = bounded(10);
    ///
    /// // Construct the acoustic chamber.
    /// let mut model = AudioModel::new(100, 100, command_rx, snapshot_tx, None);
    ///
    /// // Ask the model to pluck a string at (50, 50).
    /// brain_to_ear.send(AudioCommand::Pluck { x: 50, y: 50, strength: 1.0 }).unwrap();
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        command_rx: Receiver<AudioCommand>,
        snapshot_tx: Sender<AudioSnapshot>,
        recording_tx: Option<Sender<Vec<f32>>>,
    ) -> Self {
        Self {
            grid: PhysicsGrid::new(width, height),
            listener_x: width / 2,
            listener_y: height / 2,
            command_rx,
            snapshot_tx,
            recording_tx,
            sample_counter: 0,
            // ⚡ Bolt: Pre-allocate vectors with capacity to avoid dynamic heap reallocations
            // when multiple oscillators or tones are spawned.
            oscillators: Vec::with_capacity(32),
            active_tones: Vec::with_capacity(32),
        }
    }

    /// Processes pending commands from the main thread at the start of the audio block.
    fn process_commands(&mut self) {
        // Process all pending commands at the start of the block
        while let Ok(cmd) = self.command_rx.try_recv() {
            match cmd {
                AudioCommand::Pluck { x, y, strength } => self.grid.pluck(x, y, strength),
                AudioCommand::Oscillate {
                    x,
                    y,
                    frequency,
                    strength,
                } => self.handle_oscillate_command(x, y, frequency, strength),
                AudioCommand::Tone {
                    x,
                    y,
                    frequency,
                    strength,
                    duration_ms,
                } => self.handle_tone_command(x, y, frequency, strength, duration_ms),
                AudioCommand::AddWall { x, y } => self.grid.add_wall(x, y),
                AudioCommand::RemoveWall { x, y } => self.grid.remove_wall(x, y),
                AudioCommand::PaintMaterial { x, y, material } => {
                    self.grid.set_material(x, y, material)
                }
                AudioCommand::ClearWaves => self.grid.clear_waves(),
                AudioCommand::ClearWalls => self.grid.clear_walls(),
                AudioCommand::MoveListener { x, y } => {
                    if x < self.grid.width && y < self.grid.height {
                        self.listener_x = x;
                        self.listener_y = y;
                    }
                }
            }
        }
    }

    /// Processes audio and fills the output buffer.
    ///
    /// This method performs the following steps for each sample in the buffer:
    /// 1. Processes any pending `AudioCommand`s (once per block).
    /// 2. Updates the state of all active oscillators and tones, injecting energy into the grid.
    /// 3. Advances the physics simulation by one step (`grid.step()`).
    /// 4. Samples the grid at the listener's position.
    /// 5. Clamps the sample and writes it to the output buffer.
    /// 6. Periodically sends a snapshot of the grid to the visualization thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use resonance_audio::{AudioModel, AudioCommand};
    /// use crossbeam_channel::bounded;
    ///
    /// let (cmd_tx, cmd_rx) = bounded(10);
    /// let (snap_tx, snap_rx) = bounded(10);
    /// let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);
    ///
    /// // Fill a 10-sample buffer
    /// let mut buffer = vec![0.0; 10];
    /// model.process(&mut buffer);
    /// ```
    fn handle_oscillate_command(&mut self, x: usize, y: usize, frequency: f32, strength: f32) {
        if let Some(pos) = self.oscillators.iter().position(|o| o.x == x && o.y == y) {
            if strength.abs() < 0.001 {
                self.oscillators.swap_remove(pos);
            } else {
                let osc = &mut self.oscillators[pos];
                osc.frequency = frequency;
                osc.strength = strength;
            }
        } else if strength.abs() >= 0.001 && x < self.grid.width && y < self.grid.height {
            let idx = y * self.grid.width + x;
            self.oscillators.push(Oscillator {
                x,
                y,
                idx,
                phase: 0.0,
                frequency,
                strength,
            });
        }
    }

    fn handle_tone_command(
        &mut self,
        x: usize,
        y: usize,
        frequency: f32,
        strength: f32,
        duration_ms: u64,
    ) {
        let duration_samples = (duration_ms as f64 * 44100.0 / 1000.0) as usize;
        self.active_tones
            .push((x, y, frequency, strength, duration_samples, 0.0));
    }

    fn apply_oscillators(&mut self) {
        for osc in self.oscillators.iter_mut() {
            osc.phase += osc.frequency * 2.0 * PI / 44100.0;
            if osc.phase > 2.0 * PI {
                osc.phase -= 2.0 * PI;
            }
            let val = osc.phase.sin() * osc.strength;

            if self.grid.materials[osc.idx] != Material::Wall {
                self.grid.u[osc.idx] += val;
            }
        }
    }

    fn apply_active_tones(&mut self) {
        self.active_tones
            .retain_mut(|(x, y, freq, strength, remaining, phase)| {
                if *remaining == 0 {
                    return false;
                }
                *remaining -= 1;

                *phase += *freq * 2.0 * PI / 44100.0;
                if *phase > 2.0 * PI {
                    *phase -= 2.0 * PI;
                }
                let val = phase.sin() * *strength;

                if *x >= self.grid.width || *y >= self.grid.height {
                    return true;
                }

                let idx = *y * self.grid.width + *x;
                if self.grid.materials[idx] != Material::Wall {
                    self.grid.u[idx] += val;
                }

                true
            });
    }
    pub fn process(&mut self, output: &mut [f32]) {
        self.process_commands();

        for sample in output.iter_mut() {
            self.apply_oscillators();

            self.apply_active_tones();

            self.grid.step();

            // Sample at listener position
            let val = self.grid.get(self.listener_x, self.listener_y);

            // Soft clip / Tanh to prevent explosion
            let clamped = val.clamp(-1.0, 1.0);
            *sample = clamped;

            self.sample_counter += 1;

            // Send snapshot at ~60Hz (Assuming 44100Hz, 735 samples)
            // Use rem_euclid or just % if we are sure positive. usize is positive.
            #[allow(clippy::manual_is_multiple_of)]
            if self.sample_counter % 735 == 0 {
                // Ignore error if channel is full
                // Only clone grid if channel can receive it to avoid needless allocations
                if !self.snapshot_tx.is_full() {
                    let _ = self.snapshot_tx.try_send(AudioSnapshot {
                        pressure: self.grid.u.to_vec(),
                        materials: self.grid.materials.to_vec(),
                        energy: self.grid.energy_map.to_vec(),
                    });
                }
            }
        }

        // If recording is enabled, send a copy of the output buffer
        if let Some(tx) = &self.recording_tx {
            if !tx.is_full() {
                let buffer_copy = output.to_vec();
                // Ignore error if receiver is dropped or full (if bounded)
                let _ = tx.try_send(buffer_copy);
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

        let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);

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

    #[test]
    fn test_oscillator_lifecycle() {
        let (cmd_tx, cmd_rx) = bounded(10);
        let (snap_tx, _snap_rx) = bounded(10);

        let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);
        let mut buffer = vec![0.0; 10];

        // 1. Add Oscillator
        cmd_tx
            .send(AudioCommand::Oscillate {
                x: 2,
                y: 2,
                frequency: 440.0,
                strength: 0.5,
            })
            .unwrap();

        model.process(&mut buffer);

        assert_eq!(model.oscillators.len(), 1);
        assert_eq!(model.oscillators[0].x, 2);
        assert_eq!(model.oscillators[0].y, 2);
        assert!((model.oscillators[0].strength - 0.5).abs() < 0.001);

        // 2. Update Oscillator
        cmd_tx
            .send(AudioCommand::Oscillate {
                x: 2,
                y: 2,
                frequency: 880.0,
                strength: 0.8,
            })
            .unwrap();

        model.process(&mut buffer);

        assert_eq!(model.oscillators.len(), 1);
        assert!((model.oscillators[0].frequency - 880.0).abs() < 0.001);
        assert!((model.oscillators[0].strength - 0.8).abs() < 0.001);

        // 3. Remove Oscillator
        cmd_tx
            .send(AudioCommand::Oscillate {
                x: 2,
                y: 2,
                frequency: 880.0,
                strength: 0.0,
            })
            .unwrap();

        model.process(&mut buffer);

        assert_eq!(model.oscillators.len(), 0);

        // 4. Out of bounds Oscillator
        cmd_tx
            .send(AudioCommand::Oscillate {
                x: 20,
                y: 20,
                frequency: 440.0,
                strength: 1.0,
            })
            .unwrap();

        model.process(&mut buffer);

        assert_eq!(model.oscillators.len(), 0);
    }

    #[test]
    fn test_audio_commands() {
        let (cmd_tx, cmd_rx) = bounded(100);
        let (snap_tx, _snap_rx) = bounded(100);
        let (rec_tx, rec_rx) = bounded(100);

        let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, Some(rec_tx));

        // Issue various commands
        cmd_tx.send(AudioCommand::AddWall { x: 1, y: 1 }).unwrap();
        cmd_tx
            .send(AudioCommand::PaintMaterial {
                x: 2,
                y: 2,
                material: Material::Slow,
            })
            .unwrap();
        cmd_tx
            .send(AudioCommand::RemoveWall { x: 1, y: 1 })
            .unwrap();
        cmd_tx
            .send(AudioCommand::Tone {
                x: 5,
                y: 5,
                frequency: 440.0,
                strength: 1.0,
                duration_ms: 1,
            })
            .unwrap();
        cmd_tx
            .send(AudioCommand::MoveListener { x: 3, y: 3 })
            .unwrap();
        cmd_tx
            .send(AudioCommand::MoveListener { x: 100, y: 100 })
            .unwrap(); // out of bounds

        let mut buf = vec![0.0; 100]; // Duration 1ms is approx 44 samples
        model.process(&mut buf);

        assert_eq!(model.listener_x, 3);
        assert_eq!(model.listener_y, 3);
        assert_eq!(model.grid.materials[2 * 10 + 2], Material::Slow);
        assert_eq!(model.grid.materials[10 + 1], Material::Air); // Removed wall

        // Validate Tone added and decaying
        assert!(!model.active_tones.is_empty() || buf.iter().any(|&x| x.abs() > 0.0));

        // Ensure recording TX got a buffer
        assert!(rec_rx.try_recv().is_ok());

        cmd_tx.send(AudioCommand::ClearWalls).unwrap();
        cmd_tx.send(AudioCommand::ClearWaves).unwrap();
        let mut buf2 = vec![0.0; 10];
        model.process(&mut buf2);
    }
}
