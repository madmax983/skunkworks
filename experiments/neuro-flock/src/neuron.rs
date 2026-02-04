use rand::Rng;

#[derive(Clone, Debug)]
pub struct Neuron {
    pub v: f64,
    pub u: f64,
    // Parameters
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Neuron {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f64>();

        let (a, b, c, d) = if r < 0.8 {
            // Regular Spiking (Excitatory)
            let noise = rng.gen::<f64>();
            (
                0.02,
                0.2,
                -65.0 + 15.0 * noise.powi(2),
                8.0 - 6.0 * noise.powi(2)
            )
        } else {
            // Fast Spiking (Inhibitory)
            let noise = rng.gen::<f64>();
            (
                0.02 + 0.08 * noise,
                0.25 - 0.05 * noise,
                -65.0,
                2.0
            )
        };

        Self {
            v: -65.0,
            u: 0.2 * -65.0,
            a, b, c, d
        }
    }

    /// Update state, return true if spiked
    pub fn update(&mut self, dt: f64, input_current: f64) -> bool {
        // v' = 0.04v^2 + 5v + 140 - u + I
        // u' = a(bv - u)

        let v = self.v;
        let u = self.u;

        let dv = 0.04 * v * v + 5.0 * v + 140.0 - u + input_current;
        let du = self.a * (self.b * v - u);

        self.v += dt * dv;
        self.u += dt * du;

        if self.v >= 30.0 {
            self.v = self.c;
            self.u += self.d;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spike() {
        let mut n = Neuron::new();
        // Force a spike by setting v high
        n.v = 29.0;
        // Large input current
        let spiked = n.update(1.0, 100.0);
        assert!(spiked || n.v >= 30.0);
    }
}
