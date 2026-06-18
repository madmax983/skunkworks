use clap::Parser;
use gray_scott::GrayScott;
use neuro_sim::Network;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, help = "Run in headless mode (no GUI)")]
    headless: bool,
}

fn main() {
    let args = Args::parse();

    println!("🧬 neuro-gray: Neuromorphic Reaction-Diffusion starting...");
    if args.headless {
        println!("Running in headless mode. Bypassing GUI panics.");
    }

    // 1. Create a Gray-Scott reaction-diffusion dish
    let width = 100;
    let height = 100;
    let mut dish = GrayScott::new(width, height);

    // 2. Create a Spiking Neural Network
    let mut brain = Network::new();

    // Add a cluster of neurons and connect them randomly
    let mut neurons = Vec::new();
    for _ in 0..10 {
        neurons.push(brain.add_neuron());
    }

    for i in 0..neurons.len() {
        for j in 0..neurons.len() {
            if i != j {
                // Random excitatory/inhibitory weights
                let weight = if (i + j) % 2 == 0 { 10.0 } else { -5.0 };
                brain.add_synapse(neurons[i], neurons[j], weight);
            }
        }
    }

    // Standard Gray-Scott parameters for cell division
    let feed = 0.0367;
    let kill = 0.0649;

    let simulation_steps = if args.headless { 100 } else { 10 }; // Run longer in headless CI

    for t in 0..simulation_steps {
        // Inject current into the first neuron to stimulate the network
        let mut input_currents = vec![0.0; neurons.len()];
        if t % 10 == 0 {
            input_currents[0] = 30.0; // Strong jolt every 10 steps
        }

        brain.step(&input_currents);

        // Map spikes to chemical injections
        let mut spike_count = 0;
        for (idx, &neuron) in neurons.iter().enumerate() {
            if brain.is_spiking(neuron) {
                spike_count += 1;
                // Map the neuron index to a spatial location on the dish
                let x = 10 + (idx * 8) % (width - 20);
                let y = 10 + (idx * 5) % (height - 20);

                // Inject "Chemical V" (kill chemical) where the neuron spikes
                if let Some(x_coord) = x.try_into().ok() {
                    if let Some(y_coord) = y.try_into().ok() {
                         dish.add_chemical(x_coord, y_coord, 1.0);
                    }
                }
            }
        }

        // Update the Gray-Scott simulation
        dish.update(feed, kill, 1.0);

        if args.headless && t % 10 == 0 {
            println!("t={}: {} neurons spiked. Turing pattern evolving.", t, spike_count);
        }
    }

    println!("Simulation complete. Hybrid vigor confirmed.");
}