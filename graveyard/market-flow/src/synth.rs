//! # Market Synthesizer 🎹
//!
//! This module converts market data (Price and Trade Volume) into audio signals (Frequency and Noise).
//!
//! It implements a simple "Sonification" strategy:
//! *   **Pitch (Frequency):** Driven by the "Center of Mass" of the limit order book (the weighted average price).
//!     *   Higher Price = Higher Pitch.
//!     *   Lower Price = Lower Pitch.
//! *   **Timbre (Noise/Intensity):** Driven by the number of trades executed in the last tick.
//!     *   More Trades = More "Noise" (representing market chaos/liquidity).
//!     *   Fewer Trades = Pure Sine Wave (representing market stability).

use rand::Rng;

/// The state of the audio synthesizer.
///
/// Tracks the current frequency, noise intensity, and phase for generating
/// continuous waveforms.
///
/// # Examples
///
/// ```ignore
/// use market_flow::synth::SynthState;
///
/// let mut synth = SynthState::new();
///
/// // Update with high price (low y-coordinate) and some trades
/// synth.update(10.0, 5);
///
/// // Generate a waveform buffer for visualization
/// let waveform = synth.get_waveform(100);
/// assert_eq!(waveform.len(), 100);
/// ```
pub struct SynthState {
    /// The current target frequency in Hz.
    ///
    /// Mapped inversely from the grid's Y-coordinate center of mass.
    /// (Lower Y = Higher Price = Higher Frequency).
    pub price_frequency: f32,

    /// The "Chaos" factor, ranging from 0.0 to 1.0.
    ///
    /// Increased by trade events, decays over time.
    /// *   0.0: Pure Sine Wave.
    /// *   1.0: Significant White Noise mixed in.
    pub trade_intensity: f32,

    /// The current phase of the oscillator (0.0 to 2*PI).
    /// Used to ensure continuity between frames.
    pub phase: f32,
}

impl SynthState {
    /// Creates a new synthesizer with default settings.
    ///
    /// Starts at A4 (440Hz) with no noise.
    pub fn new() -> Self {
        Self {
            price_frequency: 440.0,
            trade_intensity: 0.0,
            phase: 0.0,
        }
    }

    /// Updates the synthesizer based on the latest market state.
    ///
    /// # Arguments
    ///
    /// * `center_of_mass_y` - The weighted average Y-coordinate of all particles.
    ///   Note: In our grid, Y=0 is Top (High Price), Y=Max is Bottom (Low Price).
    /// * `trades` - The number of trade executions that occurred in this tick.
    pub fn update(&mut self, center_of_mass_y: f32, trades: usize) {
        // Higher y (lower in grid) -> Lower Price -> Lower Freq.
        // Let's assume Price ~ Height - y.
        // If y is small (Top), Price is High -> High Freq.
        //
        // Map:
        // y=0 (Top) -> ~880Hz (High A5)
        // y=Height (Bottom) -> ~110Hz (Low A2)
        //
        // Formula: 880 - (y * 10)
        self.price_frequency = (880.0 - center_of_mass_y * 10.0).max(110.0);

        // Trades add noise/intensity.
        // Each trade adds 0.2 intensity.
        if trades > 0 {
            self.trade_intensity += (trades as f32) * 0.2;
        }

        // Decay existing intensity to return to calm.
        self.trade_intensity *= 0.9;

        // Clamp to max 1.0
        self.trade_intensity = self.trade_intensity.min(1.0);
    }

    /// Generates a waveform buffer for visualization.
    ///
    /// This function advances the oscillator phase and samples the waveform.
    ///
    /// # Arguments
    ///
    /// * `width` - The number of samples to generate (usually the width of the UI widget).
    ///
    /// # Returns
    ///
    /// A vector of `f64` samples, typically in the range [-1.0, 1.0].
    pub fn get_waveform(&mut self, width: usize) -> Vec<f64> {
        let mut buffer = Vec::with_capacity(width);
        // We visualize a small time window (e.g., 20ms) to show the wave shape.
        let time_window = 0.02;
        let dt = time_window / width as f32;

        for _ in 0..width {
            self.phase += self.price_frequency * dt * std::f32::consts::TAU;
            if self.phase > std::f32::consts::TAU {
                self.phase -= std::f32::consts::TAU;
            }

            let signal = self.phase.sin();

            // Add noise based on trade_intensity
            let mut rng = rand::thread_rng();
            let noise = (rng.r#gen::<f32>() * 2.0 - 1.0) * self.trade_intensity;

            // Mix signal (80%) and noise (20% * intensity)
            // Wait, logic in original was: signal * 0.8 + noise * 0.2
            // But noise is already scaled by intensity.
            // So if intensity is 0, noise is 0. Result is 0.8 * sin.
            // If intensity is 1, noise is range [-1, 1]. Result is 0.8*sin + 0.2*noise.
            // This keeps it within [-1, 1].
            buffer.push((signal * 0.8 + noise * 0.2) as f64);
        }

        buffer
    }
}

impl Default for SynthState {
    fn default() -> Self {
        Self::new()
    }
}
