//! System Attractor ⚛️⛈️
//!
//! "The weather of the machine."
//!
//! Visualizing the chaotic dynamics of your computer's internal state using the Lorenz Attractor.
//! The Lorenz Attractor is a system of ordinary differential equations originally derived for atmospheric convection. It is a canonical example of deterministic chaos.
//!
//! This crate maps your system's "Vital Signs" to the Lorenz parameters:
//! - **CPU Usage** -> $\sigma$ (Sigma): Represents volatility/turbulence.
//! - **Memory Usage** -> $\rho$ (Rho): Represents the driving force.
//! - **Swap Usage** -> **Jitter**: Introduces entropy/noise into the particle positions.
//! - **Load Average** -> **Color Shift & Instability**: High load causes the visualization to glitch and shift hues.

pub(crate) mod audio;
pub(crate) mod lyapunov;
pub(crate) mod simulation;

pub use audio::*;
pub use lyapunov::*;
pub use simulation::*;
