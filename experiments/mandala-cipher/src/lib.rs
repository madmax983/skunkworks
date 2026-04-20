//! # Mandala Cipher ☸️
//!
//! A visual steganography tool that encodes arbitrary binary information into geometric art.
//!
//! This module transforms standard byte arrays into a [`Mandala`], where the data is hidden
//! within the shape and color of individual [`Jewel`]s. The visual arrangement provides a
//! decorative exterior while preserving perfect reconstructability of the original bytes.
//!
//! ## Hero's Journey: Encoding and Decoding
//!
//! ```rust
//! use mandala_cipher::{MandalaConfig, encode, decode};
//!
//! // 1. Set the canvas configuration
//! let config = MandalaConfig::default();
//!
//! // 2. Encode a secret message into a visual mandala
//! let secret_data = b"Hello!";
//! let mandala = encode(secret_data, &config);
//!
//! // 3. Decode the mandala back into the original bytes
//! let recovered = decode(&mandala);
//! assert_eq!(recovered, secret_data);
//! ```

use rand::Rng;

/// The geometric representation of a single nibble of data.
///
/// Shapes are used to distinguish between actual payload and visual "chaff" (noise).
/// Specifically, [`Shape::Circle`] and [`Shape::Square`] are used to store data, while [`Shape::Triangle`] and
/// [`Shape::Diamond`] are used randomly to fill out the remaining space in the visual mandala.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    /// A data-bearing shape representing an even nibble parity.
    Circle,
    /// A data-bearing shape representing an odd nibble parity.
    Square,
    /// A noise shape ignored during decoding.
    Triangle,
    /// A noise shape ignored during decoding.
    Diamond,
}

/// The visual color of a [`Jewel`], used to encode 3 bits of data.
///
/// Because colors map perfectly to `0..8` indices, they allow us to pack the rest
/// of a 4-bit nibble alongside the [`Shape`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    /// Maps to index 0.
    Red,
    /// Maps to index 1.
    Green,
    /// Maps to index 2.
    Blue,
    /// Maps to index 3.
    Yellow,
    /// Maps to index 4.
    Purple,
    /// Maps to index 5.
    Cyan,
    /// Maps to index 6.
    White,
    /// Maps to index 7.
    Black,
}

/// A singular visual element in the mandala that may hold a 4-bit nibble of data.
///
/// A jewel combines a [`Shape`] and a [`Color`] to create a visual token. If the jewel
/// is a [`Shape::Circle`] or [`Shape::Square`], it holds exactly one nibble. Otherwise, it is decorative chaff.
#[derive(Debug, Clone, PartialEq)]
pub struct Jewel {
    /// The geometric form of the jewel.
    pub shape: Shape,
    /// The hue of the jewel.
    pub color: Color,
}

impl Jewel {
    /// Creates a new custom [`Jewel`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mandala_cipher::{Jewel, Shape, Color};
    ///
    /// let jewel = Jewel::new(Shape::Circle, Color::Blue);
    /// assert_eq!(jewel.shape, Shape::Circle);
    /// ```
    pub fn new(shape: Shape, color: Color) -> Self {
        Self { shape, color }
    }

    /// Constructs a [`Jewel`] directly from a 4-bit nibble (0-15).
    ///
    /// The least significant bit determines the [`Shape`] ([`Shape::Circle`] for even, [`Shape::Square`] for odd).
    /// The remaining 3 bits index into the 8 available [`Color`]s.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mandala_cipher::{Jewel, Shape, Color};
    ///
    /// let jewel = Jewel::from_nibble(0);
    /// assert_eq!(jewel.shape, Shape::Circle);
    /// assert_eq!(jewel.color, Color::Red);
    /// ```
    pub fn from_nibble(n: u8) -> Self {
        let n = n & 0xF;
        let shape = if n.is_multiple_of(2) {
            Shape::Circle
        } else {
            Shape::Square
        };
        let color_idx = n / 2;
        let color = match color_idx {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Cyan,
            6 => Color::White,
            _ => Color::Black,
        };
        Jewel { shape, color }
    }

    /// Attempts to extract a 4-bit nibble from the [`Jewel`].
    ///
    /// Returns `None` if the jewel is visual chaff (e.g., a [`Shape::Triangle`] or [`Shape::Diamond`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mandala_cipher::{Jewel, Shape, Color};
    ///
    /// let jewel = Jewel::new(Shape::Square, Color::Blue);
    /// assert_eq!(jewel.to_nibble(), Some(5)); // Blue (2) * 2 + Square (1) = 5
    ///
    /// let chaff = Jewel::new(Shape::Triangle, Color::Red);
    /// assert_eq!(chaff.to_nibble(), None);
    /// ```
    pub fn to_nibble(&self) -> Option<u8> {
        let s = match self.shape {
            Shape::Circle => 0,
            Shape::Square => 1,
            _ => return None,
        };
        let c = match self.color {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
            Color::Yellow => 3,
            Color::Purple => 4,
            Color::Cyan => 5,
            Color::White => 6,
            Color::Black => 7,
        };
        Some(c * 2 + s)
    }

    /// Generates a randomized noise [`Jewel`] that carries no data.
    ///
    /// Chaff jewels exclusively use [`Shape::Triangle`] or [`Shape::Diamond`] shapes to ensure they are
    /// skipped by the [`decode`] function. They are used to fill the mandala for visual
    /// completeness without corrupting the payload.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mandala_cipher::{Jewel, Shape};
    ///
    /// let chaff = Jewel::random_chaff();
    /// assert!(chaff.shape == Shape::Triangle || chaff.shape == Shape::Diamond);
    /// assert_eq!(chaff.to_nibble(), None); // Confirms it holds no data
    /// ```
    pub fn random_chaff() -> Self {
        let mut rng = rand::thread_rng();
        let shape = if rng.gen_bool(0.5) {
            Shape::Triangle
        } else {
            Shape::Diamond
        };
        let color = match rng.gen_range(0..8) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Cyan,
            6 => Color::White,
            _ => Color::Black,
        };
        Jewel { shape, color }
    }
}

/// Configuration determining the visual layout and data capacity of the mandala.
///
/// The total byte capacity of the mandala is `(rings * segments_per_ring) / 2`.
#[derive(Debug, Clone)]
pub struct MandalaConfig {
    /// The number of concentric rings extending outward from the center.
    pub rings: usize,
    /// The number of slots available for jewels along a single ring within a slice.
    pub segments_per_ring: usize,
    /// How many times the canonical data slice is mirrored/rotated visually.
    pub symmetry_order: usize,
}

impl Default for MandalaConfig {
    fn default() -> Self {
        Self {
            rings: 16,
            segments_per_ring: 8,
            symmetry_order: 12,
        }
    }
}

/// The entire encoded visual structure containing the steganographic payload.
#[derive(Debug, Clone)]
pub struct Mandala {
    /// The flattened 1D array of jewels filling the canonical sector of the mandala.
    pub jewels: Vec<Option<Jewel>>,
    /// The configuration rules used to generate this specific pattern.
    pub config: MandalaConfig,
}

impl Mandala {
    /// Initializes an empty `Mandala` layout based on the provided configuration.
    ///
    /// The capacity of the `jewels` array is pre-allocated based on the `config.rings`
    /// and `config.segments_per_ring`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mandala_cipher::{Mandala, MandalaConfig};
    ///
    /// let config = MandalaConfig::default();
    /// let mandala = Mandala::new(config.clone());
    /// assert_eq!(mandala.jewels.len(), config.rings * config.segments_per_ring);
    /// ```
    pub fn new(config: MandalaConfig) -> Self {
        let capacity = config.rings * config.segments_per_ring;
        Self {
            jewels: vec![None; capacity],
            config,
        }
    }
}

/// Encodes an arbitrary byte slice into a visual [`Mandala`].
///
/// The algorithm splits each `data` byte into two 4-bit nibbles and converts them into
/// data-bearing [`Jewel`]s based on the provided `config`. Any remaining slots in the mandala are filled with randomized
/// noise ([`Jewel::random_chaff`]) to obfuscate the data length and create a continuous pattern.
///
/// # Edge Cases
///
/// If the `data` slice is larger than the capacity of the mandala (determined by
/// `config.rings * config.segments_per_ring / 2` bytes), the payload will be **silently truncated**
/// to fit within the visual geometry.
///
/// # Examples
///
/// ```rust
/// use mandala_cipher::{MandalaConfig, encode};
///
/// let config = MandalaConfig::default();
/// let payload = vec![0xAB, 0xCD]; // 2 bytes = 4 nibbles
/// let mandala = encode(&payload, &config);
///
/// // The first jewel should represent the lower nibble of 0xAB (which is 0xB = 11)
/// assert_eq!(mandala.jewels[0].as_ref().unwrap().to_nibble(), Some(11));
/// ```
pub fn encode(data: &[u8], config: &MandalaConfig) -> Mandala {
    let mut mandala = Mandala::new(config.clone());
    let capacity = mandala.jewels.len();

    let mut jewel_idx = 0;
    for &byte in data {
        if jewel_idx + 1 >= capacity {
            break;
        }
        let low = byte & 0xF;
        let high = (byte >> 4) & 0xF;

        mandala.jewels[jewel_idx] = Some(Jewel::from_nibble(low));
        jewel_idx += 1;
        mandala.jewels[jewel_idx] = Some(Jewel::from_nibble(high));
        jewel_idx += 1;
    }

    for i in jewel_idx..capacity {
        mandala.jewels[i] = Some(Jewel::random_chaff());
    }

    mandala
}

/// Extracts the original byte payload hidden within a [`Mandala`].
///
/// The decoder scans through the `mandala.jewels` array, ignoring any visual chaff
/// ([`Shape::Triangle`] or [`Shape::Diamond`] shapes) and reassembling valid data-bearing jewels
/// back into continuous bytes.
///
/// # Examples
///
/// ```rust
/// use mandala_cipher::{MandalaConfig, encode, decode};
///
/// let config = MandalaConfig::default();
/// let original = vec![42, 255, 7];
/// let mandala = encode(&original, &config);
///
/// let recovered = decode(&mandala);
/// assert_eq!(recovered, original);
/// ```
pub fn decode(mandala: &Mandala) -> Vec<u8> {
    let mut data = Vec::new();
    let mut current_byte: u8 = 0;
    let mut is_high_nibble = false;

    for jewel in mandala.jewels.iter().flatten() {
        if let Some(nibble) = jewel.to_nibble() {
            if !is_high_nibble {
                current_byte = nibble;
                is_high_nibble = true;
            } else {
                current_byte |= nibble << 4;
                data.push(current_byte);
                current_byte = 0;
                is_high_nibble = false;
            }
        }
    }

    data
}
