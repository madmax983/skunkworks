use neuro_sim::Network;

#[test]
fn test_is_spiking_does_not_panic() {
    let mut net = Network::new();
    let n1 = net.add_neuron();

    // Inject enough current to cause a spike
    net.neurons[n1].inject(100.0);
    net.step(&[]);

    assert!(net.is_spiking(n1));
}
