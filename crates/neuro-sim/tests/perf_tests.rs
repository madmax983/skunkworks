use neuro_sim::Network;

#[test]
fn test_zero_delay_no_allocation() {
    let mut network = Network::new();
    let a = network.add_neuron();
    let b = network.add_neuron();
    network.add_synapse(a, b, 10.0); // Synapse 0 (default zero delay)

    // Force spike
    network.neurons[a].v = 35.0;

    // First step: neuron spikes
    network.step(&[]);

    // Second step: synapse sees the spike and processes it immediately
    network.step(&[]);

    // Ensure that the spike didn't cause an allocation to be kept in transit
    assert_eq!(
        network.synapses[0].spikes_in_transit.capacity(),
        0,
        "Zero-delay synapse allocated spikes_in_transit!"
    );
}
