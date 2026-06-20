use git_associates::GitModel;
use neuro_sim::Network;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Check for headless mode
    if std::env::args().any(|arg| arg == "--headless") {
        return Ok(());
    }

    println!("Starting git-neuro hybrid experiment...");

    // 1. Initialize Git History (Parent A)
    // We open the current repository
    let git = GitModel::open(".")?;
    let commits = git.history(50)?; // Get last 50 commits

    if commits.is_empty() {
        println!("No commits found to process.");
        return Ok(());
    }

    // 2. Initialize Biological Spiking Neural Network (Parent B)
    let mut net = Network::new();

    // Create a network where each commit maps roughly to a sensory neuron,
    // plus a pool of hidden neurons to process the "thoughts".
    let mut sensory_neurons = Vec::new();
    for _ in 0..commits.len() {
        sensory_neurons.push(net.add_neuron());
    }

    // Add some hidden neurons to digest the history
    let mut hidden_neurons = Vec::new();
    for _ in 0..40 {
        hidden_neurons.push(net.add_neuron());
    }

    // Wire sensory neurons to hidden neurons randomly
    for &sn in &sensory_neurons {
        for &hn in &hidden_neurons {
            // Random connection weight. Make it strongly excitatory to guarantee spikes.
            let weight = rand::random::<f32>() * 50.0;
            net.add_synapse(sn, hn, weight);
        }
    }

    // Wire hidden neurons to each other for recurrent dynamics
    for &hn1 in &hidden_neurons {
        for &hn2 in &hidden_neurons {
            if hn1 != hn2 && rand::random::<f32>() < 0.3 {
                let weight = (rand::random::<f32>() * 20.0) - 5.0;
                net.add_synapse(hn1, hn2, weight);
            }
        }
    }

    // Setup inputs buffer
    let mut inputs = vec![0.0; net.neurons.len()];

    // 3. The Novel Trait: Replay history as neural injection current
    for (i, commit) in commits.iter().enumerate().rev() {
        // Clear previous inputs
        inputs.fill(0.0);

        // Let's use the length of the commit message + diff size as a proxy for "complexity/stress"
        // and inject it as current into the corresponding sensory neuron
        let current = (commit.message.len() as f32 * 2.0) + 50.0; // Strong baseline current
        inputs[sensory_neurons[i]] = current;

        // Step the neural simulation multiple times per commit to allow spikes to propagate
        print!("Commit {}: ", &commit.short_hash);
        let mut spikes_this_commit: u32 = 0;

        for step_idx in 0..15 {
            // Only inject external current on the first step of this commit processing window
            if step_idx == 0 {
                net.step(&inputs);
            } else {
                net.step(&[]);
            }

            for &hn in &hidden_neurons {
                if net.is_spiking(hn) {
                    spikes_this_commit += 1;
                }
            }
        }

        let mut display_spikes = spikes_this_commit;
        for _ in 0..20 {
            if display_spikes > 0 {
                print!("⚡");
                display_spikes = display_spikes.saturating_sub(2); // Scale down visually
            } else {
                print!("-");
            }
        }

        if spikes_this_commit > 10 {
            println!(" (Cognitive storm!)");
        } else if spikes_this_commit > 0 {
            println!(" (Thoughts provoked!)");
        } else {
            println!(" (Quiet rumination)");
        }

        // Let the network decay a bit between commits
        for _ in 0..10 {
            net.step(&[]);
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
