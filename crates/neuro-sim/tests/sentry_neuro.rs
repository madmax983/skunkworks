use neuro_sim::Network;

#[test]
fn test_default_impls_and_getters() {
    let mut net = Network::default();
    let n = net.add_neuron();
    net.add_synapse(n, n, 1.0);

    // Inject inputs that out-of-bounds or within
    net.step(&[100.0, 10.0]); // larger than num neurons

    assert!(!net.is_spiking(100)); // out of bounds
    assert!(!net.get_synapse_activity(100)); // out of bounds
}
