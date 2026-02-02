use std::f32;

/// Izhikevich Neuron Model
/// v' = 0.04v^2 + 5v + 140 - u + I
/// u' = a(bv - u)
/// if v >= 30, then v <- c, u <- u + d
#[derive(Debug, Clone)]
pub struct Neuron {
    pub v: f32,
    pub u: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub spiked: bool,
}

impl Neuron {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            v: -65.0,
            u: b * -65.0,
            a,
            b,
            c,
            d,
            spiked: false,
        }
    }

    pub fn regular_spiking() -> Self {
        Self::new(0.02, 0.2, -65.0, 8.0)
    }

    pub fn fast_spiking() -> Self {
        Self::new(0.1, 0.2, -65.0, 2.0)
    }

    pub fn chattering() -> Self {
        Self::new(0.02, 0.2, -50.0, 2.0)
    }

    pub fn update(&mut self, input_current: f32, dt: f32) {
        // We typically assume dt=1ms for the standard constants,
        // but if we want variable dt we need to scale.
        // The standard equations are for 1ms steps.
        // Let's assume input_current includes the synaptic input.

        // Use 0.5ms or smaller sub-steps for stability if dt=1.0 is too coarse?
        // Izhikevich usually suggests 0.5ms steps for numerical stability.
        // Let's do two 0.5ms steps per update call if dt is 1.0.

        let steps = 2;
        let h = dt / steps as f32;

        self.spiked = false;

        for _ in 0..steps {
            let v = self.v;
            let u = self.u;

            // v' = 0.04v^2 + 5v + 140 - u + I
            let dv = 0.04 * v * v + 5.0 * v + 140.0 - u + input_current;
            // u' = a(bv - u)
            let du = self.a * (self.b * v - u);

            self.v += dv * h;
            self.u += du * h;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
                self.spiked = true; // Spike event happened in this frame
                                    // In a strictly discrete system we might clamp to 30 for visualization
                                    // but resetting to c is the rule.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spiking() {
        let mut n = Neuron::regular_spiking();
        let mut spike_count = 0;

        // Stimulate for 100ms with enough current to spike
        for _ in 0..100 {
            n.update(10.0, 1.0); // 10 input current
            if n.spiked {
                spike_count += 1;
            }
        }

        assert!(spike_count > 0, "Neuron should spike with input current");
    }

    #[test]
    fn test_neuron_silence() {
        let mut n = Neuron::regular_spiking();
        let mut spike_count = 0;

        // No input
        for _ in 0..100 {
            n.update(0.0, 1.0);
            if n.spiked {
                spike_count += 1;
            }
        }

        assert_eq!(spike_count, 0, "Neuron should not spike without input");
    }
}
