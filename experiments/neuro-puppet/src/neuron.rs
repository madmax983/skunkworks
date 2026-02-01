#[derive(Debug, Clone, Copy)]
pub struct IzhikevichNeuron {
    pub v: f64,
    pub u: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
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

    pub fn update(&mut self, input: f64) -> bool {
        // Standard Izhikevich model with dt=1.0ms
        let v_curr = self.v;
        let u_curr = self.u;

        self.v = v_curr + (0.04 * v_curr * v_curr + 5.0 * v_curr + 140.0 - u_curr + input);
        self.u = u_curr + self.a * (self.b * v_curr - u_curr);

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
    fn test_neuron_spiking() {
        let mut neuron = IzhikevichNeuron::new();
        let mut spiked = false;
        // Inject current
        // With I=10, it should spike eventually.
        for _ in 0..100 {
            if neuron.update(10.0) {
                spiked = true;
                break;
            }
        }
        assert!(spiked, "Neuron should spike with input current");
    }
}
