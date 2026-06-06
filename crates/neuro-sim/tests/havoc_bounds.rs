use neuro_sim::Network;

#[test]
fn havoc_neuro_bounds() {
    let mut n = Network::new();
    n.is_spiking(100);
}
