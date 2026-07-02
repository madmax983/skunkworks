//! # Stego Attack
//!
//! A simulation that embeds and extracts hidden data within image files using
//! visual cryptography and steganography. It visualizes the attack as a swarm
//! of agents dismantling the cover image.

/// The configuration parameters for tuning the swarm behavior.
pub(crate) mod config;
/// The core physical simulation where pixels become agents.
pub(crate) mod simulation;
/// The steganography logic for embedding and extracting LSB data.
pub(crate) mod stego;

// Facade API
pub use config::*;
pub use simulation::*;
pub use stego::*;
