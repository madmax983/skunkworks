use neuro_sim::Network;

#[test]
fn havoc_neuro_alloc() {
    let mut n = Network::new();
    // try to make spike in transit explode
    let n1 = n.add_neuron();
    let n2 = n.add_neuron();
    n.add_synapse_with_delay(n1, n2, 10.0, 1_000_000);
    n.step(&[100.0, 0.0]); // Make n1 spike. It pushes the huge delay
}
