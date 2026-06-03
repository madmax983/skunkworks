use neuro_sim::Network;

#[test]
fn test_sentry_out_of_bounds_inputs_in_update() {
    let mut net = Network::new();
    let n1 = net.add_neuron();
    let _n2 = net.add_neuron();

    // We intentionally step with fewer inputs than neurons.
    // The neurons should just default their input_val to 0.0 without panicking.
    net.step(&[10.0]); // Provides input only for n1. n2 should get 0.0 due to unwrap_or(0.0)

    // The fact that this doesn't panic means the `inputs.get(i).copied().unwrap_or(0.0)` logic is safe.
    assert_ne!(net.neurons[n1].v, -65.0); // Should have moved from rest due to 10.0 input
}
