//! # Neuro-Terminal 🧠
//!
//! A real-time neural network visualization library.
//!
//! This crate provides the foundational math and logic to simulate and train
//! simple neural networks from scratch. It is designed to be paired with a
//! TUI (Terminal User Interface) to visualize the learning process of a network
//! attempting to solve 2D classification problems.
//!
//! ## Core Modules
//!
//! * **[`nn`]**: The core Neural Network implementation, including custom Matrix math
//!   and backpropagation algorithms.
//! * **[`evo`]**: An evolutionary algorithm (Genetic Algorithm) implementation for
//!   training networks without backpropagation.

pub mod evo;
pub use evo::*;
pub mod nn;
pub use nn::*;
