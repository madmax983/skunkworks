#[derive(Debug, Clone, Copy)]
pub struct IzhikevichNeuron {
    pub v: f32,
    u: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl IzhikevichNeuron {
    pub fn new() -> Self {
        Self {
            v: -65.0,
            u: -65.0 * 0.2,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
        }
    }

    pub fn with_params(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            v: -65.0,
            u: -65.0 * b,
            a,
            b,
            c,
            d,
        }
    }

    // Returns true if spiked
    pub fn update(&mut self, input_current: f32) -> bool {
        // Izhikevich simple model (dt = 1ms assumed)
        // v' = 0.04v^2 + 5v + 140 - u + I
        // u' = a(bv - u)

        // Use 2 sub-steps of 0.5ms for numerical stability
        let dt = 0.5;
        let mut spiked = false;

        for _ in 0..2 {
            let v = self.v;
            let u = self.u;

            self.v += dt * (0.04 * v * v + 5.0 * v + 140.0 - u + input_current);
            self.u += dt * (self.a * (self.b * v - u));

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
                spiked = true;
            }
        }

        spiked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spiking() {
        let mut neuron = IzhikevichNeuron::new();
        let mut spiked = false;
        // Inject high current for 100 steps
        for _ in 0..100 {
            if neuron.update(20.0) {
                spiked = true;
                break;
            }
        }
        assert!(spiked, "Neuron should spike with high input current");
    }
}
