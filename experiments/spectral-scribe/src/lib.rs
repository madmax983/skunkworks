//! Spectral Scribe: Audio Steganography Engine.
//!
//! Encodes text strings into audio files by translating characters into
//! spectrogram footprints, and decoding them back into text.

pub mod decoder;
pub use decoder::*;
pub mod encoder;
pub use encoder::*;
/// Bitmapped font rendering mapping module.
pub(crate) mod font;
pub use font::*;
