#[derive(Clone, Debug)]
pub struct IzhikevichNeuron {
    pub v: f32,
    pub u: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub i_syn: f32,
    pub last_spike_time: Option<f32>,
}

impl IzhikevichNeuron {
    pub fn new_rs() -> Self {
        Self {
            v: -65.0,
            u: 0.2 * -65.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            i_syn: 0.0,
            last_spike_time: None,
        }
    }

    // Fast Spiking (Inhibitory)
    pub fn new_fs() -> Self {
        Self {
            v: -65.0,
            u: 0.2 * -65.0,
            a: 0.1,
            b: 0.2,
            c: -65.0,
            d: 2.0,
            i_syn: 0.0,
            last_spike_time: None,
        }
    }

    pub fn update(&mut self, dt: f32, i_ext: f32) -> bool {
        // v' = 0.04v^2 + 5v + 140 - u + I
        // u' = a(bv - u)

        let v_sq = self.v * self.v;
        let dv = 0.04 * v_sq + 5.0 * self.v + 140.0 - self.u + self.i_syn + i_ext;
        let du = self.a * (self.b * self.v - self.u);

        self.v += dv * dt;
        self.u += du * dt;

        // Decay synaptic current
        self.i_syn *= 0.90;

        if self.v >= 30.0 {
            self.v = self.c;
            self.u += self.d;
            return true;
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct Synapse {
    pub pre: usize,
    pub post: usize,
    pub weight: f32,
    pub delay: usize,
    pub spikes_in_transit: Vec<usize>,
}

impl Synapse {
    pub fn new(pre: usize, post: usize, weight: f32, delay: usize) -> Self {
        Self {
            pre,
            post,
            weight,
            delay,
            spikes_in_transit: Vec::new(),
        }
    }
}

pub struct CPGNetwork {
    pub neurons: Vec<IzhikevichNeuron>,
    pub synapses: Vec<Synapse>,
    pub time: f32,
}

impl CPGNetwork {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
            time: 0.0,
        }
    }

    pub fn add_neuron(&mut self, neuron: IzhikevichNeuron) -> usize {
        self.neurons.push(neuron);
        self.neurons.len() - 1
    }

    pub fn add_synapse(&mut self, pre: usize, post: usize, weight: f32, delay: usize) {
        self.synapses.push(Synapse::new(pre, post, weight, delay));
    }

    pub fn mutate(&mut self) {
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        for syn in &mut self.synapses {
            // Jitter weights
            let jitter = rng.gen_range(-5.0..5.0);
            syn.weight += jitter;

            // Clamp weights to reasonable range?
            // Excitatory should stay positive? Inhibitory negative?
            // Or allow sign flip?
            // Let's keep sign but change magnitude.
            // Or just allow drift.
        }
    }

    pub fn step(&mut self, dt: f32, external_inputs: &[f32]) {
        self.time += dt;

        let neurons = &mut self.neurons;

        // 1. Update Neurons & Collect Spikes
        let mut spiked_indices = Vec::new();
        for (i, n) in neurons.iter_mut().enumerate() {
            let i_ext = if i < external_inputs.len() {
                external_inputs[i]
            } else {
                0.0
            };
            if n.update(dt, i_ext) {
                n.last_spike_time = Some(self.time);
                spiked_indices.push(i);
            }
        }

        // 2. Process Synapses
        let synapses = &mut self.synapses;

        for syn in synapses {
            // If pre spiked, add to transit
            if spiked_indices.contains(&syn.pre) {
                syn.spikes_in_transit.push(syn.delay);
            }

            let mut weight_to_add = 0.0;
            syn.spikes_in_transit.retain_mut(|t| {
                if *t == 0 {
                    weight_to_add += syn.weight;
                    false
                } else {
                    *t -= 1;
                    true
                }
            });

            if weight_to_add != 0.0 {
                neurons[syn.post].i_syn += weight_to_add;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_spiking() {
        let mut n = IzhikevichNeuron::new_rs();
        let mut spiked = false;

        // Inject current
        for _ in 0..1000 {
            if n.update(0.5, 10.0) {
                spiked = true;
                break;
            }
        }

        assert!(spiked, "Neuron should spike with high input current");
    }
}
