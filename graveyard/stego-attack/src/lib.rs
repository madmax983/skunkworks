//! # Stego Attack
//!
//! A simulation that embeds and extracts hidden data within image files using
//! visual cryptography and steganography. It visualizes the attack as a swarm
//! of agents dismantling the cover image.

/// The configuration parameters for tuning the swarm behavior.
pub mod config;
/// The core physical simulation where pixels become agents.
pub mod simulation;
/// The steganography logic for embedding and extracting LSB data.
pub mod stego;
