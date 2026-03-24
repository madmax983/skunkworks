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

#[test]
fn test_sentry_out_of_bounds_is_spiking() {
    let mut net = Network::new();
    let n1 = net.add_neuron();

    // Normal access
    assert_eq!(net.is_spiking(n1), false);

    // Out of bounds access
    assert_eq!(net.is_spiking(99), false);
}

#[test]
fn test_sentry_out_of_bounds_get_synapse_activity() {
    let mut net = Network::new();
    let n1 = net.add_neuron();
    let n2 = net.add_neuron();
    net.add_synapse(n1, n2, 10.0);

    // Normal access
    assert_eq!(net.get_synapse_activity(0), false);

    // Out of bounds access
    assert_eq!(net.get_synapse_activity(99), false);
}
