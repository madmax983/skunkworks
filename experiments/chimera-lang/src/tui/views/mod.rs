//! Visual View Providers for the TUI.
//!
//! This module groups all the specialized `ratatui` rendering logic into distinct domain categories.
//! When a user cycles through the interface, different panels are selected and drawn. Each submodule
//! here corresponds to a group of thematic views inside the simulation.
//!
//! * [`audio`] - Renderers for audio and sonic properties (e.g., waveforms, acoustics).
//! * [`bio`] - Renderers for biological processes (e.g., DNA inspection, genetics, taxonomy).
//! * [`core`] - The essential system renderers (e.g., grid views, execution stack, system metrics).
//! * [`magic`] - Renderers tracking esolang interactions and narrative elements (e.g., alchemy, necromancy).
//! * [`misc`] - Assorted renderers that don't fit perfectly into other domains.
//! * [`physics`] - Renderers for physical or continuous phenomena (e.g., fluid dynamics, geology, quantum).
//! * [`tech`] - Renderers for logic gates, structures, or lower-level computing elements (e.g., circuits).

pub(crate) mod audio;
pub(crate) mod bio;
pub(crate) mod core;
pub(crate) mod magic;
pub(crate) mod misc;
pub(crate) mod physics;
pub(crate) mod tech;

pub(crate) use audio::*;
pub(crate) use bio::*;
pub(crate) use core::*;
pub(crate) use magic::*;
pub(crate) use misc::*;
pub(crate) use physics::*;
pub(crate) use tech::*;

// Note: render_prologue_esolang is defined in magic.rs and exported via wildcard.
