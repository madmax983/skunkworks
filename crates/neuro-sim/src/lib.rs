//! # Neuro Sim
//!
//! A high-level Spiking Neural Network (SNN) simulation crate powered by [`synaptic_physics`].
//!
//! This crate provides a [`Network`] abstraction that manages a collection of [`Izhikevich`] neurons
//! connected by [`Synapse`]s. It handles spike propagation, synaptic delays, and weights.
//!
//! ## The Model
//!
//! - **Neurons**: Uses the Izhikevich model (via `synaptic_physics`) which balances biological plausibility with performance.
//! - **Synapses**: Directed connections with:
//!     - **Weight**: Strength of the connection (positive = excitatory, negative = inhibitory).
//!     - **Delay**: Discrete time steps before a spike reaches the target.
//! - **Time**: Discrete steps. By convention, 1 step $\approx$ 1ms (though this is adjustable via interpretation).
//!
//! ## Hero's Journey: Building a Brain
//!
//! ```rust
//! use neuro_sim::Network;
//!
//! // 1. Create a blank network
//! let mut brain = Network::new();
//!
//! // 2. Add two neurons
//! let sensory = brain.add_neuron(); // Neuron 0
//! let motor = brain.add_neuron();   // Neuron 1
//!
//! // 3. Connect them (Sensory -> Motor)
//! //    Weight: 15.0 (Strong excitation)
//! //    Delay: 0 (Immediate effect in next step)
//! brain.add_synapse(sensory, motor, 15.0);
//!
//! // 4. Simulate!
//! // We'll inject current into the sensory neuron to make it fire.
//! for t in 0..10 {
//!     // Input: 20.0 units to Neuron 0, 0.0 to Neuron 1
//!     brain.step(&[20.0, 0.0]);
//!
//!     if brain.is_spiking(sensory) {
//!         println!("t={}: Sensory Neuron Spiked! ⚡", t);
//!     }
//!     if brain.is_spiking(motor) {
//!         println!("t={}: Motor Neuron Responded! 🦾", t);
//!     }
//! }
//! ```

use synaptic_physics::Izhikevich;

/// A connection between two neurons.
///
/// When the `from` neuron spikes, a signal travels along this synapse.
/// After `delay` steps, the `weight` is added to the `to` neuron's input current.
#[derive(Clone, Debug)]
pub struct Synapse {
    /// Index of the source neuron.
    pub from: usize,
    /// Index of the target neuron.
    pub to: usize,
    /// Strength of the connection.
    /// * Positive > 0: Excitatory (EPSP).
    /// * Negative < 0: Inhibitory (IPSP).
    pub weight: f32,
    /// Transmission delay in simulation steps.
    /// * 0: The spike affects the target in the *very next* step.
    /// * N: The spike affects the target in N+1 steps.
    pub delay: usize,
    /// Queue of spikes currently traveling along this synapse.
    /// Values represent "remaining steps until arrival".
    pub spikes_in_transit: Vec<usize>,
    /// Visualization state: did this synapse deliver a spike in the most recent step?
    pub active: bool,
}

/// A network of neurons and synapses.
///
/// This is the main container for the simulation. It owns all neurons and synapses
/// and orchestrates the update loop.
#[derive(Clone, Debug)]
pub struct Network {
    /// The collection of neurons in the network.
    pub neurons: Vec<Izhikevich>,
    /// The collection of connections between neurons.
    pub synapses: Vec<Synapse>,
    /// A cache of which neurons spiked in the *previous* step.
    ///
    /// This is used to decouple the update order: synapses read from this
    /// frozen state to determine if they should initiate a new signal.
    pub spikes: Vec<bool>,

    /// Internal buffer for accumulating inputs to avoid re-allocation every step.
    input_buffer: Vec<f32>,
}

impl Network {
    /// Creates a new, empty network.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_sim::Network;
    /// let net = Network::new();
    /// assert_eq!(net.neurons.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
            spikes: Vec::new(),
            input_buffer: Vec::new(),
        }
    }

    /// Adds a default Izhikevich neuron (Regular Spiking) to the network.
    ///
    /// # Returns
    ///
    /// The index (`usize`) of the newly created neuron. Use this index to connect synapses.
    pub fn add_neuron(&mut self) -> usize {
        self.neurons.push(Izhikevich::new());
        self.spikes.push(false);
        self.neurons.len() - 1
    }

    /// Adds a synapse with zero delay (immediate effect).
    ///
    /// # Arguments
    ///
    /// * `from` - Index of source neuron.
    /// * `to` - Index of target neuron.
    /// * `weight` - Connection strength.
    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f32) {
        self.add_synapse_with_delay(from, to, weight, 0);
    }

    /// Adds a synapse with a specified delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - The number of steps the signal takes to travel.
    ///   * 0: Arrives in step T+1 (if spike at T).
    ///   * 1: Arrives in step T+2.
    ///
    /// # Examples
    ///
    /// ```
    /// use neuro_sim::Network;
    /// let mut net = Network::new();
    /// let n1 = net.add_neuron();
    /// let n2 = net.add_neuron();
    ///
    /// // Spike at T=0 reaches n2 at T=11
    /// net.add_synapse_with_delay(n1, n2, 10.0, 10);
    /// ```
    pub fn add_synapse_with_delay(&mut self, from: usize, to: usize, weight: f32, delay: usize) {
        self.synapses.push(Synapse {
            from,
            to,
            weight,
            delay,
            spikes_in_transit: Vec::new(),
            active: false,
        });
    }

    /// Advances the simulation by one time step.
    ///
    /// The update cycle is:
    /// 1. **Inputs**: Apply external current and clear buffers.
    /// 2. **Synapses**:
    ///    - Check if source neurons spiked *last* step.
    ///    - Advance spikes in transit.
    ///    - Deliver spikes that arrived (t=0) to target neurons.
    /// 3. **Neurons**: Update membrane potential and check for new spikes.
    ///
    /// # Arguments
    ///
    /// * `external_inputs` - A slice of currents to inject into neurons matching the index.
    ///   If the slice is shorter than the neuron count, remaining neurons receive 0.0.
    pub fn step(&mut self, external_inputs: &[f32]) {
        // 1. Collect inputs for this step
        if self.input_buffer.len() != self.neurons.len() {
            self.input_buffer.resize(self.neurons.len(), 0.0);
        }
        self.input_buffer.fill(0.0);

        // Add external inputs
        for (i, val) in external_inputs.iter().enumerate() {
            if i < self.input_buffer.len() {
                self.input_buffer[i] += val;
            }
        }

        // 2. Process Synapses (Propagate spikes)
        for syn in &mut self.synapses {
            syn.active = false;

            // Check if source neuron spiked in previous step
            if let Some(&spiked) = self.spikes.get(syn.from) {
                if spiked {
                    syn.spikes_in_transit.push(syn.delay);
                }
            }

            // Advance spikes in transit
            let mut weight_to_add = 0.0;
            syn.spikes_in_transit.retain_mut(|t| {
                if *t == 0 {
                    weight_to_add += syn.weight;
                    false // Remove from queue
                } else {
                    *t -= 1;
                    true // Keep in queue
                }
            });

            if weight_to_add != 0.0 {
                if let Some(input) = self.input_buffer.get_mut(syn.to) {
                    *input += weight_to_add;
                    syn.active = true;
                }
            }
        }

        // 3. Update Neurons
        // We assume 1.0ms time step to match previous local implementation behavior
        let dt = 1.0;

        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let (_, spiked) = neuron.update(dt, self.input_buffer[i]);
            self.spikes[i] = spiked;
        }
    }

    /// Checks if a specific neuron spiked in the most recent step.
    pub fn is_spiking(&self, index: usize) -> bool {
        self.spikes.get(index).cloned().unwrap_or(false)
    }

    /// Checks if a specific synapse was active (delivered a spike) in the most recent step.
    pub fn get_synapse_activity(&self, index: usize) -> bool {
        self.synapses.get(index).map(|s| s.active).unwrap_or(false)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_synapse_is_ignored() {
        let mut network = Network::new();
        let a = network.add_neuron();
        // Add synapse pointing to invalid neuron index 99
        network.add_synapse(a, 99, 1.0);

        // Force a spike
        network.neurons[a].v = 40.0;

        // First step: neuron spikes, `spikes[a]` becomes true
        network.step(&[]);

        // Second step: synapse sees spike (from prev step), pushes to transit (delay 0).
        // Since delay is 0, it processes immediately in THIS step (step 2).
        // It tries to add weight to neuron 99, but ignores it safely.
        network.step(&[]);

        // If we reached here without panic, the test passes.
    }

    #[test]
    fn test_inhibition() {
        let mut network = Network::new();
        let a = network.add_neuron();
        let b = network.add_neuron();

        // Add inhibitory synapse A -> B
        network.add_synapse(a, b, -10.0);

        // Set A to spike threshold
        network.neurons[a].v = 35.0;

        // Step 1: A spikes. Spike is recorded in `self.spikes`.
        // B receives 0 input yet because synapse reads `self.spikes` from *start* of step 1 (which was empty/false).
        network.step(&[]);
        assert!(network.spikes[a], "A should have spiked");

        let v_before_impact = network.neurons[b].v;

        // Step 2: Synapse reads `self.spikes` (true). Pushes delay 0.
        // Processes delay 0 -> adds weight to B's input.
        // B updates with negative input.
        network.step(&[]);

        let v_after = network.neurons[b].v;

        // B should be inhibited
        assert!(
            v_after < v_before_impact - 0.1,
            "Post-synaptic neuron should be inhibited by spike (Before: {}, After: {})",
            v_before_impact,
            v_after
        );
    }

    #[test]
    fn test_delay() {
        let mut network = Network::new();
        let a = network.add_neuron();
        let b = network.add_neuron();

        // Add synapse with delay 2
        network.add_synapse_with_delay(a, b, 10.0, 2);

        // Force spike
        network.neurons[a].v = 35.0;

        // Step 1: A spikes.
        network.step(&[]);
        assert!(network.spikes[a]);

        // Step 2: Synapse sees spike. Pushes delay 2.
        // Transit: [2]. Decrement? No, wait.
        // Logic: Push 2. Then retain_mut:
        // If *t == 0 -> fire. Else *t -= 1.
        // So 2 becomes 1.
        network.step(&[]);

        // Step 3: Transit: [1]. Decrement -> 0.
        network.step(&[]);

        // Step 4: Transit: [0]. Fire! Remove.
        let _v_start = network.neurons[b].v;
        network.step(&[]);
        let _v_end = network.neurons[b].v;

        // Wait, let's trace carefully.
        // Step 1: A spikes (recorded in `spikes`).
        // Step 2: Synapse reads `spikes[a]`. Pushes 2.
        //         Loop over transit: 2 -> 1.
        //         No fire.
        // Step 3: Synapse reads `spikes[a]` (false).
        //         Loop over transit: 1 -> 0.
        //         No fire.
        // Step 4: Synapse reads `spikes[a]` (false).
        //         Loop over transit: 0 -> fire!
        //         Add weight to B. B updates.

        // So delay 2 means:
        // T=1 (Spike)
        // T=2 (Transit 1)
        // T=3 (Transit 0)
        // T=4 (Impact)

        // Delay 0:
        // T=1 (Spike)
        // T=2 (Transit 0 -> Impact immediately)
        // This matches "next step" behavior (since T=1 spike affects T=2 input).

        // So delay 2 adds 2 EXTRA steps of delay beyond the implicit 1-step delay.
        // Effectively 3 steps total from spike to impact.

        // Let's verify Step 4 had impact.
        // In Step 4, `v_end` should be higher than `v_start` (if no decay/drift masking it).
        // Actually, let's just check `active`.
        assert!(network.get_synapse_activity(0));
    }
}
