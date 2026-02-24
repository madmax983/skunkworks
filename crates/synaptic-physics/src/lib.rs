//! # Synaptic Physics
//!
//! Shared physics logic for neural simulation experiments, specifically providing the [`Izhikevich`] neuron model.
//!
//! ## Why use this?
//!
//! Simulating biological neural networks involves a trade-off between realism and performance:
//! *   **Hodgkin-Huxley:** Biologically accurate but computationally expensive (4+ ODEs per neuron).
//! *   **Integrate-and-Fire:** Very fast but lacks complex spiking dynamics (bursting, chattering).
//! *   **Izhikevich (This crate):** The "Goldilocks" zone. It uses just 2 ODEs and 4 parameters to reproduce
//!     virtually all known cortical spiking patterns (regular spiking, fast spiking, bursting, etc.)
//!     with the speed of integrate-and-fire models.
//!
//! ## The Physics
//!
//! The model simulates membrane potential using two differential equations:
//!
//! 1.  **Membrane Potential ($v$):**
//!     $v' = 0.04v^2 + 5v + 140 - u + I$
//!
//! 2.  **Recovery Variable ($u$):**
//!     $u' = a(bv - u)$
//!
//! **Units:**
//! *   **$v$ (Voltage):** Measured in millivolts (mV). Resting potential is typically around -65.0 mV.
//! *   **$t$ (Time):** Measured in milliseconds (ms).
//! *   **$I$ (Current):** Input current. In this dimensionless model, $10.0$ is a typical strong DC input.
//!
//! ## Usage
//!
//! ```rust
//! use synaptic_physics::Izhikevich;
//!
//! // 1. Create a neuron (e.g., Regular Spiking)
//! let mut neuron = Izhikevich::new();
//!
//! // 2. Simulate for 100ms with a time step of 0.1ms
//! let dt = 0.1;
//! for t in 0..1000 {
//!     // 3. Update state with 10.0 units of constant input current
//!     let (voltage, spiked) = neuron.update(dt, 10.0);
//!
//!     if spiked {
//!         println!("Neuron fired at {} ms!", t as f32 * dt);
//!     }
//! }
//! ```
//!
//! ## References
//! * Izhikevich, E. M. (2003). [Simple model of spiking neurons](https://www.izhikevich.org/publications/spikes.htm).
//!   IEEE Transactions on Neural Networks, 14(6), 1569-1572.

use rand::Rng;
use std::fmt;

/// Number of internal substeps for numerical integration stability.
const SUBSTEPS: usize = 2;

/// The Izhikevich neuron model.
///
/// This struct holds the state variables (`v`, `u`) and parameters (`a`, `b`, `c`, `d`) defining the neuron's behavior.
/// The model uses a system of two ordinary differential equations to simulate membrane potential dynamics.
///
/// # Examples
///
/// Creating a custom neuron with modified reset parameters:
///
/// ```rust
/// use synaptic_physics::Izhikevich;
///
/// let mut neuron = Izhikevich {
///     v: -65.0,
///     u: -13.0,
///     a: 0.02,
///     b: 0.2,
///     c: -55.0, // Reset to -55mV (instead of standard -65mV)
///     d: 4.0,   // Smaller recovery step
///     current_decay: 0.0,
///     tau: 10.0,
/// };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Izhikevich {
    /// **Membrane potential ($v$).**
    /// Represents the voltage across the neuron membrane in millivolts (mV).
    /// *   Range: Typically -90.0 to +30.0.
    /// *   Resting: ~ -65.0.
    pub v: f32,

    /// **Membrane recovery variable ($u$).**
    /// Accounts for the activation of K+ ionic currents and inactivation of Na+ ionic currents.
    /// It provides negative feedback to $v$.
    pub u: f32,

    /// **Time scale of recovery ($a$).**
    /// Describes how fast the variable $u$ returns to equilibrium.
    /// *   Small values (e.g., 0.02) result in slow recovery.
    pub a: f32,

    /// **Sensitivity of recovery ($b$).**
    /// Describes how strongly $u$ is coupled to subthreshold fluctuations of $v$.
    /// *   Larger values result in stronger coupling.
    pub b: f32,

    /// **After-spike reset value ($c$).**
    /// The value $v$ is reset to after a spike ($v \ge 30$).
    /// *   Typical value: -65.0 mV.
    pub c: f32,

    /// **After-spike recovery reset ($d$).**
    /// The amount added to $u$ after a spike.
    /// *   Typical value: 8.0 or 2.0.
    pub d: f32,

    /// **Transient Synaptic Current.**
    /// Represents decaying input current from synaptic events (spikes).
    /// Values added here (via `inject`) decay exponentially based on `tau`.
    pub current_decay: f32,

    /// **Synaptic Decay Time Constant ($\tau$).**
    /// Controls how fast the injected impulse fades (in milliseconds).
    /// *   Formula: $I(t) = I_0 \cdot e^{-t/\tau}$
    pub tau: f32,
}

impl fmt::Display for Izhikevich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Izhikevich(v={:.1} mV, u={:.1}, I={:.1})",
            self.v, self.u, self.current_decay
        )
    }
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
        Self::new_regular_spiking()
    }

    /// Creates a "Regular Spiking" (RS) neuron.
    ///
    /// Typical of cortical excitatory neurons.
    ///
    /// Parameters: $a=0.02, b=0.2, c=-65.0, d=8.0, \tau=10.0$.
    pub fn new_regular_spiking() -> Self {
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

    /// Creates a "Fast Spiking" (FS) neuron.
    ///
    /// Typical of inhibitory interneurons.
    ///
    /// Parameters: $a=0.1, b=0.2, c=-65.0, d=2.0, \tau=5.0$.
    pub fn new_fast_spiking() -> Self {
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.1,
            b: 0.2,
            c: -65.0,
            d: 2.0,
            current_decay: 0.0,
            tau: 5.0,
        }
    }

    /// Creates a "Chattering" (CH) neuron.
    ///
    /// Typical of bursting cortical neurons.
    ///
    /// Parameters: $a=0.02, b=0.2, c=-50.0, d=2.0, \tau=10.0$.
    pub fn new_chattering() -> Self {
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

    /// Creates a new neuron with random parameters selected from presets.
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
    /// use rand::thread_rng;
    ///
    /// let mut rng = thread_rng();
    /// let neuron = Izhikevich::random(&mut rng);
    /// ```
    pub fn random(rng: &mut impl Rng) -> Self {
        let r = rng.gen::<f32>();
        if r < 0.6 {
            Self::new_regular_spiking()
        } else if r < 0.8 {
            Self::new_fast_spiking()
        } else {
            Self::new_chattering()
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
    ///
    /// // Inject a spike event (e.g., EPSP)
    /// neuron.inject(50.0);
    ///
    /// // The injection fades over time
    /// assert!(neuron.current_decay == 50.0);
    /// neuron.update(1.0, 0.0);
    /// assert!(neuron.current_decay < 50.0);
    /// ```
    pub fn inject(&mut self, current: f32) {
        self.current_decay += current;
    }

    /// Updates the neuron state for a time step `dt`.
    ///
    /// Performs numerical integration (Euler method) to advance the simulation.
    /// This method uses internal substeps (default: 2) to maintain stability even with larger `dt`.
    ///
    /// # Parameters
    ///
    /// * `dt`: The time step size (e.g., 0.1 or 1.0). Small steps improve accuracy. Must be non-negative.
    /// * `extra_current`: Continuous input current ($I$) applied during this step (e.g., from sensory input).
    ///
    /// # Returns
    ///
    /// Returns a tuple `(voltage, spiked)`:
    /// * `voltage`: The membrane potential after the update.
    ///   **Note:** If a spike occurred, this value is the reset potential ($c$), not the peak (30mV).
    /// * `spiked`: Boolean indicating if the neuron fired an action potential (reached threshold 30mV) during any substep.
    ///
    /// # Panics
    ///
    /// This function does not panic in release mode, but passing `NaN` or `Inf` for `dt` or `extra_current` will propagate those values to the neuron state.
    /// In debug mode, it may panic if `dt < 0.0` or `tau <= 0.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use synaptic_physics::Izhikevich;
    /// let mut neuron = Izhikevich::new();
    ///
    /// // Advance by 1.0 unit of time with 5.0 units of input current
    /// let (v, spiked) = neuron.update(1.0, 5.0);
    ///
    /// if spiked {
    ///     println!("Bang!");
    /// }
    /// ```
    pub fn update(&mut self, dt: f32, extra_current: f32) -> (f32, bool) {
        debug_assert!(dt >= 0.0, "Time step must be non-negative");
        debug_assert!(self.tau > 0.0, "Tau must be positive");

        // Internal substeps for numerical stability
        let dt_sub = dt / SUBSTEPS as f32;
        let mut spiked = false;

        // Decay the injected current
        // e^(-dt / tau)
        let decay = (-dt_sub / self.tau).exp();

        for _ in 0..SUBSTEPS {
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
    use rand::Rng;

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

        // Expected decay factor: exp(-dt / tau)
        // This holds regardless of substeps because (exp(-dt/N))^N = exp(-dt)
        let expected = 100.0 * (-dt / n.tau).exp();

        n.update(dt, 0.0);

        let tolerance = 0.0001;
        assert!(
            (n.current_decay - expected).abs() < tolerance,
            "Decay should match exp(-dt/tau) behavior. Got {}, expected {}",
            n.current_decay,
            expected
        );
    }

    #[test]
    fn test_named_constructors() {
        let rs = Izhikevich::new_regular_spiking();
        assert_eq!(rs.a, 0.02);
        assert_eq!(rs.d, 8.0);

        let fs = Izhikevich::new_fast_spiking();
        assert_eq!(fs.a, 0.1);
        assert_eq!(fs.d, 2.0);

        let ch = Izhikevich::new_chattering();
        assert_eq!(ch.c, -50.0);
    }

    #[test]
    fn test_display() {
        let n = Izhikevich::new();
        let s = format!("{}", n);
        assert!(s.contains("Izhikevich(v="));
        assert!(s.contains("mV"));
    }

    #[test]
    fn test_stability_random_walk() {
        let mut rng = rand::thread_rng();
        let mut n = Izhikevich::new();
        let dt = 0.5;

        // Run for 1000 steps with random noise
        for _ in 0..1000 {
            let noise = rng.gen_range(-5.0..5.0);
            let (v, _) = n.update(dt, noise);

            // Bounds check for numerical explosion
            // Izhikevich model can spike to +30, but u can drift.
            // Extreme divergence would result in +/- Inf or very large numbers.
            // We set generous bounds to catch "explosion".
            assert!(v < 200.0, "Voltage exploded positively: {}", v);
            assert!(v > -200.0, "Voltage exploded negatively: {}", v);
            assert!(n.u < 200.0, "Recovery variable u exploded positively: {}", n.u);
            assert!(n.u > -200.0, "Recovery variable u exploded negatively: {}", n.u);
        }
    }

    #[test]
    fn test_zero_tau_decay() {
        let mut n = Izhikevich::new();
        n.tau = 0.0000001; // Effectively zero
        n.inject(100.0);

        // With tau -> 0, decay should be instant.
        // exp(-dt / 0) -> exp(-inf) -> 0.0
        n.update(1.0, 0.0);

        assert!(n.current_decay < 0.0001, "Current should have decayed instantly");
    }

    #[test]
    #[should_panic(expected = "Time step must be non-negative")]
    #[cfg(debug_assertions)]
    fn test_negative_dt_panic() {
        let mut n = Izhikevich::new();
        n.update(-0.1, 0.0);
    }
}
