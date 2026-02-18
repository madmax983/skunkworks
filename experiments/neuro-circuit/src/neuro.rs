use crate::circuit_gen::Circuit;

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

    pub fn update(&mut self, dt: f32, i_ext: f32) -> bool {
        let v_sq = self.v * self.v;
        let dv = 0.04 * v_sq + 5.0 * self.v + 140.0 - self.u + self.i_syn + i_ext;
        let du = self.a * (self.b * self.v - self.u);

        self.v += dv * dt;
        self.u += du * dt;

        self.i_syn *= 0.90; // Decay

        if self.v >= 30.0 {
            self.v = self.c;
            self.u += self.d;
            return true;
        }
        false
    }
}

pub struct Pulse {
    pub trace_idx: usize,
    pub position_idx: usize, // Index in the trace path
    pub speed: usize,
}

pub struct NeuralCircuit {
    pub neurons: Vec<IzhikevichNeuron>,
    pub pulses: Vec<Pulse>,
    pub time: f32,
}

impl NeuralCircuit {
    pub fn new(num_neurons: usize) -> Self {
        let mut neurons = Vec::new();
        for _ in 0..num_neurons {
            neurons.push(IzhikevichNeuron::new_rs());
        }
        Self {
            neurons,
            pulses: Vec::new(),
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, circuit: &Circuit) {
        self.time += dt;

        // 1. Update Neurons
        let mut spiked_indices = Vec::new();

        // Random background noise to keep it alive
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        for (i, n) in self.neurons.iter_mut().enumerate() {
            // Random thalamic input
            let noise = if rng.gen_bool(0.02) { 15.0 } else { 0.0 };

            if n.update(dt, noise) {
                n.last_spike_time = Some(self.time);
                spiked_indices.push(i);
            }
        }

        // 2. Spawn Pulses
        for &neuron_idx in &spiked_indices {
            // Find all traces starting at this neuron (pad)
            for (trace_idx, trace) in circuit.traces.iter().enumerate() {
                if trace.start_pad == neuron_idx {
                    self.pulses.push(Pulse {
                        trace_idx,
                        position_idx: 0,
                        speed: 3, // Move 3 pixels per frame
                    });
                }
            }
        }

        // 3. Move Pulses & Deliver Spikes
        let mut pulses_to_keep = Vec::new();

        for mut pulse in self.pulses.drain(..) {
            let trace = &circuit.traces[pulse.trace_idx];
            pulse.position_idx += pulse.speed;

            if pulse.position_idx >= trace.path.len() {
                // Pulse arrived!
                // Stimulate target neuron
                // Weight depends on distance? Or constant?
                // Let's use constant weight for now, strong enough to trigger or near trigger.
                self.neurons[trace.end_pad].i_syn += 20.0;
            } else {
                pulses_to_keep.push(pulse);
            }
        }

        self.pulses = pulses_to_keep;
    }
}
