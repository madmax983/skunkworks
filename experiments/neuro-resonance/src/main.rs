use crossbeam_channel::bounded;
use neuro_sim::Network;
use resonance_audio::audio::{AudioCommand, AudioModel};

/// Bio-Acoustic Rhythm. Spiking Neural Network (SNN) where neurons pluck an acoustic wave tank.
///
/// The discrete neural spikes translate into continuous acoustic vibrations.
fn main() {
    println!("🧬 Splice: neuro-resonance");
    println!("Initializing Spiking Neural Network (neuro-sim)...");
    let mut brain = Network::new();

    // Create a simple rhythmic pacemaker network
    let n1 = brain.add_neuron();
    let n2 = brain.add_neuron();
    let n3 = brain.add_neuron();

    // Connect them in a ring with varying delays to create polyrhythms
    brain.add_synapse_with_delay(n1, n2, 20.0, 5);
    brain.add_synapse_with_delay(n2, n3, 20.0, 10);
    brain.add_synapse_with_delay(n3, n1, 20.0, 15);

    // Give initial kick
    brain.neurons[n1].inject(100.0);

    println!("Initializing Acoustic Wave Tank (resonance-audio)...");
    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, _snap_rx) = bounded(1);

    let mut model = AudioModel::new(100, 100, cmd_rx, snap_tx, None);

    // Run a short simulation to show the concept (phenotype)
    println!("Beginning bio-acoustic simulation loop...");
    let mut buffer = vec![0.0; 512];
    for _ in 0..10 {
        brain.step(&[]);

        // Translate spikes into physical plucks in the wave tank
        if brain.is_spiking(n1) {
            println!("Neuron {} spiked! Plucking tank at (30, 30)...", n1);
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: 30,
                y: 30,
                strength: 1.0,
            });
        }
        if brain.is_spiking(n2) {
            println!("Neuron {} spiked! Plucking tank at (70, 30)...", n2);
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: 70,
                y: 30,
                strength: 1.0,
            });
        }
        if brain.is_spiking(n3) {
            println!("Neuron {} spiked! Plucking tank at (50, 70)...", n3);
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: 50,
                y: 70,
                strength: 1.0,
            });
        }

        // Let the acoustic tank process
        model.process(&mut buffer);
    }

    println!("Bio-acoustic simulation complete. Emergent rhythmic waves successfully produced.");
}
