use std::time::Instant;
use synaptic_physics::Izhikevich;

fn main() {
    let mut neuron = Izhikevich::new();
    let iterations = 10_000_000;
    let dt = 0.1;
    let input = 10.0;

    // Warmup
    for _ in 0..1000 {
        neuron.update(dt, input);
    }

    let start = Instant::now();
    let mut spikes = 0;

    // Use a black box to prevent optimization (simple version)
    for _ in 0..iterations {
        let (_, spiked) = neuron.update(dt, input);
        if spiked {
            spikes += 1;
        }
    }

    let duration = start.elapsed();
    println!("Time: {:.2?}", duration);
    println!("Spikes: {}", spikes);
    println!("Time per update: {:.2} ns", duration.as_nanos() as f64 / iterations as f64);
}
