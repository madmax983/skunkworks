use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum NeuronType {
    RegularSpiking,
    FastSpiking,
    Chattering,
    IntrinsicallyBursting,
    Resonator,
}

#[derive(Debug, Clone)]
pub struct IzhikevichNeuron {
    pub v: f32, // Membrane potential (mV)
    pub u: f32, // Recovery variable

    // Parameters
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,

    // For noise
    pub noise_level: f32,
}

impl IzhikevichNeuron {
    pub fn new(n_type: NeuronType) -> Self {
        let v = -65.0;
        let (a, b, c, d) = match n_type {
            NeuronType::RegularSpiking => (0.02, 0.2, -65.0, 8.0),
            NeuronType::FastSpiking => (0.1, 0.2, -65.0, 2.0),
            NeuronType::Chattering => (0.02, 0.2, -50.0, 2.0),
            NeuronType::IntrinsicallyBursting => (0.02, 0.2, -55.0, 4.0),
            NeuronType::Resonator => (0.1, 0.26, -65.0, 2.0),
        };

        let u = b * v;

        Self {
            v,
            u,
            a,
            b,
            c,
            d,
            noise_level: 0.0, // Default no noise, can be enabled
        }
    }

    pub fn update(&mut self, input_current: f32, dt: f32) -> bool {
        let mut spike = false;

        // Add noise if enabled
        let noise = if self.noise_level > 0.0 {
             rand::thread_rng().gen_range(-self.noise_level..self.noise_level)
        } else {
            0.0
        };

        let i = input_current + noise;

        // Izhikevich Equations
        // v' = 0.04v^2 + 5v + 140 - u + I
        // u' = a(bv - u)

        let dv = 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + i;
        let du = self.a * (self.b * self.v - self.u);

        self.v += dv * dt;
        self.u += du * dt;

        if self.v >= 30.0 {
            self.v = self.c;
            self.u += self.d;
            spike = true;
        }

        spike
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spiking() {
        let mut neuron = IzhikevichNeuron::new(NeuronType::RegularSpiking);
        let mut spiked = false;

        // Apply strong current for 100ms
        for _ in 0..100 {
            if neuron.update(10.0, 1.0) {
                spiked = true;
                break;
            }
        }

        assert!(spiked, "Neuron should spike with sufficient input current");
    }

    #[test]
    fn test_neuron_resting() {
        let mut neuron = IzhikevichNeuron::new(NeuronType::RegularSpiking);
        let mut spiked = false;

        // Apply no current
        for _ in 0..100 {
             if neuron.update(0.0, 1.0) {
                spiked = true;
             }
        }

        assert!(!spiked, "Neuron should not spike with zero input current");
    }
}
