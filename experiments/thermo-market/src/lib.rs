//! Thermo-Market 📈🌡️
//!
//! "The Invisible Hand is Hot."
//!
//! **Thermo-Market** is a hybrid experiment exploring the thermodynamics of financial markets.
//! It combines a Continuous Double Auction market simulation with a Thermodynamic Construction simulation.
//!
//! In this world, Trading Activity generates Heat.
//! - Bids (Green) and Asks (Red) move through the grid to find prices.
//! - When they collide, a Trade occurs (Yellow Flash).
//! - Each Trade releases Heat (Energy) into the environment.
//! - Heat creates Volatility (Random Jitter) in the market particles, making efficient trading harder.

pub(crate) mod world;
pub use world::*;
