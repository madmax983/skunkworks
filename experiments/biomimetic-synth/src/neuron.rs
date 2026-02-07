use rand::Rng;

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
    /// Last spike time (for visualization/debugging)
    pub last_spike: Option<u64>,
}

impl Izhikevich {
    /// Creates a new neuron with default "Regular Spiking" parameters.
    pub fn new() -> Self {
        Self {
            v: -65.0,
            u: -13.0, // b * v = 0.2 * -65 = -13
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            last_spike: None,
        }
    }

    /// Creates a neuron with "Chattering" parameters (bursting).
    pub fn chattering() -> Self {
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -50.0,
            d: 2.0,
            last_spike: None,
        }
    }

    /// Creates a neuron with random parameters to create diversity.
    pub fn random(rng: &mut impl Rng) -> Self {
        let r = rng.gen::<f32>();
        if r < 0.5 {
            Self::new() // Regular Spiking
        } else if r < 0.8 {
            // Fast Spiking
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.1,
                b: 0.2,
                c: -65.0,
                d: 2.0,
                last_spike: None,
            }
        } else {
            Self::chattering()
        }
    }

    /// Updates the neuron state. Returns true if the neuron spiked.
    /// Uses Euler integration with 2 substeps for stability.
    pub fn update(&mut self, dt: f32, input_current: f32, tick: u64) -> bool {
        let substeps = 2;
        let dt_sub = dt / substeps as f32;
        let mut spiked = false;

        for _ in 0..substeps {
            let v = self.v;
            let u = self.u;

            // v' = 0.04v^2 + 5v + 140 - u + I
            let dv = 0.04 * v * v + 5.0 * v + 140.0 - u + input_current;

            // u' = a(bv - u)
            let du = self.a * (self.b * v - u);

            self.v += dv * dt_sub;
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                // Spike!
                self.v = self.c;
                self.u += self.d;
                spiked = true;
                self.last_spike = Some(tick);
            }
        }
        spiked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_spiking() {
        let mut n = Izhikevich::new();
        // Inject sufficient current to cause spiking
        let input = 10.0;
        let mut spike_count = 0;

        // Simulate for 100ms (assuming dt=1.0ms is standard for Izhikevich, but here we pass dt explicitly)
        // Izhikevich model is usually tuned for dt=1ms steps if coefficients are as above.
        // If we use dt=1.0, update logic holds.
        for t in 0..100 {
            if n.update(1.0, input, t) {
                spike_count += 1;
            }
        }

        // Should spike at least once with input=10
        assert!(spike_count > 0, "Neuron did not spike with input current");
    }

    #[test]
    fn test_no_input_silence() {
        let mut n = Izhikevich::new();
        let mut spike_count = 0;
        for t in 0..100 {
            if n.update(1.0, 0.0, t) {
                spike_count += 1;
            }
        }
        assert_eq!(spike_count, 0, "Neuron spiked without input");
    }
}
