use macroquad::prelude::*;
use neuro_sim::Izhikevich;

#[derive(Clone, Copy, PartialEq)]
pub enum NeuronType {
    Excitatory, // Regular Spiking (RS)
    Inhibitory, // Fast Spiking (FS)
    Pacemaker,  // Chattering (CH) - used for rhythmic drive
}

pub struct Neuron {
    pub physics: Izhikevich,
    pub pos: Vec2,
    pub neuron_type: NeuronType,
    pub voltage: f32,
    pub spiked: bool,
    pub incoming_synapses: Vec<(usize, f32)>, // (source_idx, weight)
}

impl Neuron {
    pub fn new(pos: Vec2, neuron_type: NeuronType) -> Self {
        let mut physics = Izhikevich::new();

        // Configure parameters based on type
        match neuron_type {
            NeuronType::Excitatory => {
                // RS: a=0.02, b=0.2, c=-65, d=8
                physics.a = 0.02;
                physics.b = 0.2;
                physics.c = -65.0;
                physics.d = 8.0;
            }
            NeuronType::Inhibitory => {
                // FS: a=0.1, b=0.2, c=-65, d=2
                physics.a = 0.1;
                physics.b = 0.2;
                physics.c = -65.0;
                physics.d = 2.0;
            }
            NeuronType::Pacemaker => {
                // CH: a=0.02, b=0.2, c=-50, d=2
                physics.a = 0.02;
                physics.b = 0.2;
                physics.c = -50.0;
                physics.d = 2.0;
            }
        }

        Self {
            physics,
            pos,
            neuron_type,
            voltage: physics.v,
            spiked: false,
            incoming_synapses: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_update() {
        let mut brain = Brain::new();
        brain.add_neuron(vec2(0.0, 0.0), NeuronType::Excitatory);
        brain.add_neuron(vec2(10.0, 10.0), NeuronType::Inhibitory);

        let external_currents = vec![0.0, 0.0];
        // Just ensure it doesn't panic
        brain.update(0.1, &external_currents);

        assert_eq!(brain.neurons.len(), 2);
    }
}

pub struct Brain {
    pub neurons: Vec<Neuron>,
    pub spikes: Vec<usize>, // Indices of neurons that spiked this frame
}

impl Brain {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            spikes: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self, pos: Vec2, neuron_type: NeuronType) -> usize {
        let idx = self.neurons.len();
        self.neurons.push(Neuron::new(pos, neuron_type));
        idx
    }

    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f32) {
        if to < self.neurons.len() && from < self.neurons.len() {
            self.neurons[to].incoming_synapses.push((from, weight));
        }
    }

    pub fn update(&mut self, total_dt: f32, external_currents: &[f32]) {
        self.spikes.clear();

        // 1. Process Synapses (Input aggregation)
        // Check previous frame spikes to inject current.
        let mut injections = vec![0.0; self.neurons.len()];
        for (target_idx, target_neuron) in self.neurons.iter().enumerate() {
            let mut input_kick = 0.0;
            for (source_idx, weight) in &target_neuron.incoming_synapses {
                if self.neurons[*source_idx].spiked {
                    input_kick += *weight;
                }
            }
            injections[target_idx] = input_kick;
        }

        // Apply injections once per frame (instantaneous kick)
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            if injections[i] != 0.0 {
                neuron.physics.inject(injections[i]);
            }
        }

        // 2. Sub-stepping Physics
        // Use a fixed step size for consistency
        let step_size = 1.0; // 1.0 ms
        let mut accumulated_time = 0.0;

        // Reset spiked flag for this frame accumulation
        for neuron in &mut self.neurons {
            neuron.spiked = false;
        }

        while accumulated_time < total_dt {
            let dt = if total_dt - accumulated_time < step_size {
                total_dt - accumulated_time
            } else {
                step_size
            };

            for (i, neuron) in self.neurons.iter_mut().enumerate() {
                let external = if i < external_currents.len() {
                    external_currents[i]
                } else {
                    0.0
                };
                let (v, spiked) = neuron.physics.update(dt, external);

                neuron.voltage = v;
                if spiked {
                    neuron.spiked = true;
                }
            }
            accumulated_time += dt;
        }

        // Collect spikes after all substeps
        for (i, neuron) in self.neurons.iter().enumerate() {
            if neuron.spiked {
                self.spikes.push(i);
            }
        }
    }
}
