use macroquad::prelude::Vec2;
use synaptic_physics::Izhikevich;

pub struct Neuron {
    pub id: usize,
    pub pos: Vec2,
    pub physics: Izhikevich,
    pub frequency: f32,
    pub last_spike_time: Option<f32>,
}

impl Neuron {
    pub fn new(id: usize, pos: Vec2, frequency: f32) -> Self {
        Self {
            id,
            pos,
            physics: Izhikevich::new(), // Default RS neuron
            frequency,
            last_spike_time: None,
        }
    }
}

pub struct Synapse {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
    pub delay: usize,
    pub timer: usize,
    pub active: bool, // For visualization
}

pub struct Network {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self, pos: Vec2, frequency: f32) -> usize {
        let id = self.neurons.len();
        self.neurons.push(Neuron::new(id, pos, frequency));
        id
    }

    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f32, delay: usize) {
        self.synapses.push(Synapse {
            from,
            to,
            weight,
            delay,
            timer: 0,
            active: false,
        });
    }

    pub fn update(&mut self, dt: f32, current_time: f32) -> Vec<usize> {
        let mut spikes = Vec::new();
        let mut input_currents = vec![0.0; self.neurons.len()];

        // 1. Process Synapses
        for synapse in &mut self.synapses {
            synapse.active = false;
            if synapse.timer > 0 {
                synapse.timer -= 1;
                if synapse.timer == 0 {
                    // Spike arrived
                    if synapse.to < input_currents.len() {
                        input_currents[synapse.to] += synapse.weight;
                        synapse.active = true; // Flash for visualization
                    }
                }
            }
        }

        // 2. Update Neurons
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            // physics.update returns (voltage, spiked)
            // Izhikevich update takes dt and input current I
            let (_v, spiked) = neuron.physics.update(dt, input_currents[i]);

            if spiked {
                spikes.push(i);
                neuron.last_spike_time = Some(current_time);
            }
        }

        // 3. Initiate Synaptic Transmission
        for &spike_id in &spikes {
            for synapse in &mut self.synapses {
                if synapse.from == spike_id {
                    synapse.timer = synapse.delay;
                }
            }
        }

        spikes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_spike_propagation() {
        let mut net = Network::new();
        let id1 = net.add_neuron(Vec2::new(0.0, 0.0), 440.0);
        let id2 = net.add_neuron(Vec2::new(10.0, 0.0), 440.0);

        // Strong connection
        net.add_synapse(id1, id2, 100.0, 1);

        // Force spike neuron 1
        net.neurons[id1].physics.v = 30.0; // Threshold

        let spikes = net.update(1.0, 1.0);
        assert!(spikes.contains(&id1));

        // Next step, spike should travel
        // Synapse delay is 1. Timer starts at 1.
        // Update 1: timer -> 0. Input added to id2.

        let spikes2 = net.update(1.0, 2.0);
        // id2 should have received current.
        // 100.0 is enough to spike immediately in Izhikevich model?
        // v' = ... + I.
        // With substeps, it might take a frame.

        // Check if id2 spiked or v increased significantly
        if spikes2.contains(&id2) {
            // Success
        } else {
            assert!(net.neurons[id2].physics.v > -65.0);
        }
    }
}
