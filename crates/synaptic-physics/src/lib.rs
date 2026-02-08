//! # Synaptic Physics
//!
//! Shared physics logic for neural simulation experiments.
//!
//! This crate provides the `Izhikevich` neuron model, which is used in `synaptic-choir` and `synaptic-pachinko`.

use rand::Rng;

/// The Izhikevich neuron model.
///
/// See: <https://www.izhikevich.org/publications/spikes.htm>
#[derive(Clone, Copy, Debug)]
pub struct Izhikevich {
    /// Membrane potential
    pub v: f32,
    /// Recovery variable
    pub u: f32,
    /// Time scale of recovery variable
    pub a: f32,
    /// Sensitivity of recovery variable
    pub b: f32,
    /// After-spike reset value of v
    pub c: f32,
    /// After-spike reset of u
    pub d: f32,
    /// Decaying injected current (used for impulse injections)
    pub current_decay: f32,
}

impl Izhikevich {
    /// Creates a new neuron with default "Regular Spiking" parameters.
    pub fn new() -> Self {
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            current_decay: 0.0,
        }
    }

    /// Creates a new neuron with random parameters (Regular, Fast Spiking, or Chattering).
    pub fn random(rng: &mut impl Rng) -> Self {
        let r = rng.gen::<f32>();
        if r < 0.6 {
            // Regular Spiking
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.02,
                b: 0.2,
                c: -65.0,
                d: 8.0,
                current_decay: 0.0,
            }
        } else if r < 0.8 {
            // Fast Spiking
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.1,
                b: 0.2,
                c: -65.0,
                d: 2.0,
                current_decay: 0.0,
            }
        } else {
            // Chattering
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.02,
                b: 0.2,
                c: -50.0,
                d: 2.0,
                current_decay: 0.0,
            }
        }
    }

    /// Injects a current impulse that will decay over time.
    pub fn inject(&mut self, current: f32) {
        self.current_decay += current;
    }

    /// Updates the neuron state for a time step `dt`.
    ///
    /// `extra_current` is a continuous current added to the simulation for this step.
    /// It is added to `current_decay`.
    ///
    /// Returns the current membrane potential `v`.
    pub fn update(&mut self, dt: f32, extra_current: f32) -> f32 {
        let substeps = 2;
        let dt_sub = dt / substeps as f32;

        for _ in 0..substeps {
            // Decay the injected current
            self.current_decay *= 0.95; // Exponential decay

            let total_current = extra_current + self.current_decay;

            let dv = 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + total_current;
            self.v += dv * dt_sub;

            let du = self.a * (self.b * self.v - self.u);
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
            }
        }

        self.v
    }
}

impl Default for Izhikevich {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_update() {
        let mut n = Izhikevich::new();
        let v_start = n.v;
        n.update(0.1, 10.0);
        assert!(n.v > v_start);
    }

    #[test]
    fn test_injection() {
        let mut n = Izhikevich::new();
        n.inject(10.0);
        assert_eq!(n.current_decay, 10.0);
        n.update(0.1, 0.0);
        assert!(n.current_decay < 10.0);
    }

    #[test]
    fn test_spike_reset() {
        let mut n = Izhikevich::new();
        // Force a value just below threshold
        n.v = 29.9;
        // Small step should push it over 30, triggering reset to `c` (-65.0)
        // Note: update runs 2 substeps.
        n.update(0.01, 10.0);

        // Should be near resting potential (reset value), not sky high
        assert!(n.v < 0.0);
        assert!(n.v >= -70.0);
    }

    #[test]
    fn test_random_generation() {
        let mut rng = rand::thread_rng();
        // Just verify it doesn't panic and returns valid floats
        for _ in 0..100 {
            let n = Izhikevich::random(&mut rng);
            assert!(n.v.is_finite());
            assert!(n.u.is_finite());
        }
    }

    #[test]
    fn test_nan_resilience() {
        let mut n = Izhikevich::new();
        // Inject NaN current
        n.update(0.1, f32::NAN);
        // v should become NaN, but function should not panic
        assert!(n.v.is_nan());

        // Check if subsequent updates panic
        n.update(0.1, 0.0);
        assert!(n.v.is_nan());
    }

    #[test]
    fn test_current_decay_behavior() {
        let mut n = Izhikevich::new();
        n.inject(100.0);
        // decay is 0.95 per substep (2 substeps per update) => 0.9025 per update
        n.update(1.0, 0.0);

        let expected = 100.0 * 0.95 * 0.95;
        let tolerance = 0.0001;
        assert!((n.current_decay - expected).abs() < tolerance,
            "Decay should match 0.95^2 per update call");
    }
}
