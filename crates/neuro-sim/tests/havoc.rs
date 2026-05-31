use neuro_sim::Network;

// 👺 Havoc: Prove that `update_neurons` panics when `inputs[i]` is accessed because the
// `inputs` vector is exposed publicly and can be truncated or manipulated, causing an out-of-bounds error
// during the internal step phase. Or `spikes[i]` can also be out of bounds!
#[test]

fn havoc_neuro_oob_panic() {
    let mut net = Network::new();
    // Add a neuron normally
    net.add_neuron();

    // 🧨 The Trigger: Mutating public fields that internal step relies upon length guarantees for.
    // `update_neurons` uses `neurons.iter().enumerate()` and does `inputs[i]` and `spikes[i]`.
    // If we truncate either of these, we trigger a panic.
    net.inputs.clear();

    // This will call update_neurons, which panics when accessing inputs[0]
    net.step(&[]);
}
