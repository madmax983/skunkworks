//! # Resonance Audio
//!
//! A real-time 2D wave physics simulation engine for audio synthesis.
//!
//! This crate implements a **Finite Difference Time Domain (FDTD)** solver for the 2D wave equation.
//! It simulates acoustic propagation in a grid, allowing for dynamic interactions between sound waves,
//! boundaries (walls), and listeners.
//!
//! ## Core Concepts
//!
//! - **Physics Grid**: The simulation space where waves propagate. It uses a discrete grid where each cell
//!   represents the pressure at a specific point.
//! - **Audio Model**: The high-level controller that manages the grid, processes commands, and generates
//!   audio samples for the output buffer.
//! - **Audio Commands**: Instructions to modify the simulation state, such as plucking a string,
//!   adding a wall, or moving the listener.
//!
//! ## Usage
//!
//! The main entry point is the [`AudioModel`](audio::AudioModel). It runs on the audio thread, processing
//! commands from a channel and filling an output buffer.
//!
//! ```rust
//! use resonance_audio::audio::{AudioModel, AudioCommand};
//! use crossbeam_channel::bounded;
//!
//! // 1. Create channels for communication
//! let (cmd_tx, cmd_rx) = bounded(128);
//! let (snap_tx, snap_rx) = bounded(1);
//!
//! // 2. Initialize the audio model
//! // Width: 100, Height: 100
//! let mut model = AudioModel::new(100, 100, cmd_rx, snap_tx);
//!
//! // 3. Send a command to pluck the grid at (50, 50)
//! cmd_tx.send(AudioCommand::Pluck { x: 50, y: 50, strength: 0.5 }).unwrap();
//!
//! // 4. Process audio (typically done in an audio callback)
//! let mut buffer = vec![0.0; 512];
//! model.process(&mut buffer);
//! ```

pub mod audio;
pub mod physics;
