//! # Synaptic Physics
//!
//! Shared physics logic for neural simulation experiments.
//!
//! This crate provides the [`Izhikevich`] neuron model, a computationally efficient model that reproduces
//! spiking and bursting behavior of cortical neurons. It combines the biological plausibility of
//! Hodgkin-Huxley-type dynamics with the computational efficiency of integrate-and-fire models.
//!
//! ## Usage
//!
//! ```rust
//! use synaptic_physics::Izhikevich;
//!
//! // Create a default "Regular Spiking" neuron
//! let mut neuron = Izhikevich::new();
//!
//! // Simulate for 100ms
//! let dt = 0.1; // time step
//! for _ in 0..1000 {
//!     // Update with 10.0 units of input current
//!     let (voltage, spiked) = neuron.update(dt, 10.0);
//!
//!     if spiked {
//!         println!("Spike!");
//!     }
//! }
//! ```
//!
//! ## References
//! * Izhikevich, E. M. (2003). [Simple model of spiking neurons](https://www.izhikevich.org/publications/spikes.htm).
//!   IEEE Transactions on Neural Networks, 14(6), 1569-1572.

use rand::Rng;

/// The Izhikevich neuron model.
///
/// This struct holds the state variables (`v`, `u`) and parameters (`a`, `b`, `c`, `d`) defining the neuron's behavior.
/// The model uses a system of two ordinary differential equations to simulate membrane potential dynamics.
///
/// # Examples
///
/// Creating a custom neuron:
///
/// ```rust
/// use synaptic_physics::Izhikevich;
///
/// let mut neuron = Izhikevich {
///     v: -65.0,
///     u: -13.0,
///     a: 0.02,
///     b: 0.2,
///     c: -55.0, // Higher reset potential
///     d: 4.0,   // Lower reset recovery
///     current_decay: 0.0,
///     tau: 10.0, // Decay time constant
/// };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Izhikevich {
    /// Membrane potential ($v$). Represents the voltage across the neuron membrane in millivolts (mV).
    /// Typically rests around -65.0.
    pub v: f32,

    /// Membrane recovery variable ($u$). Accounts for the activation of K+ ionic currents
    /// and inactivation of Na+ ionic currents. It provides negative feedback to $v$.
    pub u: f32,

    /// Time scale of the recovery variable $u$.
    /// Smaller values result in slower recovery.
    pub a: f32,

    /// Sensitivity of the recovery variable $u$ to the subthreshold fluctuations of the membrane potential $v$.
    /// Greater values couple $v$ and $u$ more strongly.
    pub b: f32,

    /// After-spike reset value of the membrane potential $v$.
    /// When $v \ge 30$, $v$ is reset to $c$.
    pub c: f32,

    /// After-spike reset of the recovery variable $u$.
    /// When $v \ge 30$, $u$ is reset to $u + d$.
    pub d: f32,

    /// Decaying injected current (used for impulse injections).
    /// This value is added to the input current during updates and decays exponentially based on `tau`.
    pub current_decay: f32,

    /// Time constant for current decay (in ms).
    /// Controls how fast the injected impulse fades.
    pub tau: f32,
}

impl Izhikevich {
    /// Creates a new neuron with default "Regular Spiking" (RS) parameters.
    ///
    /// Parameters: $a=0.02, b=0.2, c=-65.0, d=8.0, \tau=10.0$.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synaptic_physics::Izhikevich;
    /// let neuron = Izhikevich::new();
    /// assert_eq!(neuron.a, 0.02);
    /// ```
    pub fn new() -> Self {
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            current_decay: 0.0,
            tau: 10.0,
        }
    }

    /// Creates a new neuron with random parameters.
    ///
    /// Selects between three common firing patterns based on probabilities:
    /// - **Regular Spiking (60%):** Standard cortical neuron behavior.
    /// - **Fast Spiking (20%):** Interneuron behavior.
    /// - **Chattering (20%):** Bursting behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synaptic_physics::Izhikevich;
    /// let mut rng = rand::thread_rng();
    /// let neuron = Izhikevich::random(&mut rng);
    /// ```
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
                tau: 10.0,
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
                tau: 5.0, // Faster decay for fast spiking
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
                tau: 10.0,
            }
        }
    }

    /// Injects a current impulse that will decay over time.
    ///
    /// This is useful for simulating a sudden synaptic event (spike arrival) rather than a continuous current.
    /// The injected current is added to `current_decay`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synaptic_physics::Izhikevich;
    /// let mut neuron = Izhikevich::new();
    /// neuron.inject(50.0); // Simulates a strong kick
    /// ```
    pub fn inject(&mut self, current: f32) {
        self.current_decay += current;
    }

    /// Updates the neuron state for a time step `dt`.
    ///
    /// Performs numerical integration (Euler method) to advance the simulation.
    ///
    /// # Parameters
    ///
    /// * `dt`: The time step size (e.g., 0.1 or 1.0). Small steps improve accuracy.
    /// * `extra_current`: Continuous input current ($I$) applied during this step (e.g., from sensory input).
    ///
    /// # Returns
    ///
    /// Returns a tuple `(voltage, spiked)`:
    /// * `voltage`: The membrane potential after the update.
    /// * `spiked`: Boolean indicating if the neuron fired an action potential during this step.
    ///
    /// # Panics
    ///
    /// This function does not panic, but passing `NaN` or `Inf` for `dt` or `extra_current` will propagate those values to the neuron state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synaptic_physics::Izhikevich;
    /// let mut neuron = Izhikevich::new();
    /// // Advance by 1.0 unit of time with 5.0 units of input current
    /// let (v, spiked) = neuron.update(1.0, 5.0);
    /// ```
    pub fn update(&mut self, dt: f32, extra_current: f32) -> (f32, bool) {
        // Internal substeps for numerical stability
        let substeps = 2;
        let dt_sub = dt / substeps as f32;
        let mut spiked = false;

        // Decay the injected current
        // e^(-dt / tau)
        let decay = (-dt_sub / self.tau).exp();

        for _ in 0..substeps {
            self.current_decay *= decay;

            let total_current = extra_current + self.current_decay;

            let dv = 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + total_current;
            self.v += dv * dt_sub;

            let du = self.a * (self.b * self.v - self.u);
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
                spiked = true;
            }
        }

        (self.v, spiked)
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
        let (_, spiked) = n.update(0.01, 100.0); // Strong current to ensure spike

        assert!(spiked, "Should have spiked");
        // Should be near resting potential (reset value)
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
        n.tau = 10.0;
        n.inject(100.0);

        let dt = 1.0;
        let substeps = 2;
        let dt_sub = dt / substeps as f32;

        // Expected decay factor per substep
        let decay_factor = (-dt_sub / n.tau).exp();
        let expected = 100.0 * decay_factor * decay_factor; // 2 substeps

        n.update(dt, 0.0);

        let tolerance = 0.0001;
        assert!(
            (n.current_decay - expected).abs() < tolerance,
            "Decay should match exp(-dt/tau) behavior. Got {}, expected {}",
            n.current_decay,
            expected
        );
    }
}
