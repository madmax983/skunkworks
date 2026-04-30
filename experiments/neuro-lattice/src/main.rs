use ::rand::Rng;
use macroquad::prelude::*;
use miller_lattice::Crystal;
use neuro_sim::Network;
use std::path::Path;

fn window_conf() -> Conf {
    Conf {
        window_title: "Neuro Lattice".to_owned(),
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root_path = Path::new(manifest_dir).join("../../crates");

    // Build the Crystal from the file system
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());
    let num_atoms = crystal.atoms.len();
    println!("Built Crystal Lattice with {} atoms.", num_atoms);

    // Build the Neural Network
    let mut network = Network::new();
    let mut rng = ::rand::thread_rng();

    // Map each Atom to a Neuron
    // We add a neuron for each atom. They'll be inserted in order, so atom index == neuron index.
    for _ in 0..num_atoms {
        network.add_neuron();
    }

    // Connect the neurons based on the Crystal's bonds
    // A bond is (parent_idx, child_idx)
    for &(parent_idx, child_idx) in &crystal.bonds {
        // Excitatory signals propagating down the tree
        let weight_down = rng.gen_range(8.0..15.0);
        let delay_down = rng.gen_range(1..5);
        network.add_synapse_with_delay(parent_idx, child_idx, weight_down, delay_down);

        // Weak inhibitory or excitatory signals going back up
        if rng.gen_bool(0.5) {
            let weight_up = rng.gen_range(-5.0..2.0);
            let delay_up = rng.gen_range(1..3);
            network.add_synapse_with_delay(child_idx, parent_idx, weight_up, delay_up);
        }
    }

    // Add some random cross-connections to make it a more complex network (simulating complex coupling)
    for _ in 0..(num_atoms / 5) {
        let n1 = rng.gen_range(0..num_atoms);
        let n2 = rng.gen_range(0..num_atoms);
        if n1 != n2 {
            let weight = rng.gen_range(-10.0..10.0);
            let delay = rng.gen_range(1..10);
            network.add_synapse_with_delay(n1, n2, weight, delay);
        }
    }

    let mut inputs = vec![0.0; num_atoms];

    let camera = Camera3D {
        position: vec3(0.0, 15.0, 30.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // 1. User Interaction (Stimulate random nodes on spacebar)
        if is_key_pressed(KeyCode::Space) {
            for _ in 0..(num_atoms / 10).max(1) {
                let idx = rng.gen_range(0..num_atoms);
                inputs[idx] += 100.0;
            }
        }

        // Apply constant background noise to keep network alive
        for i in 0..num_atoms {
            if rng.gen_bool(0.01) {
                inputs[i] += rng.gen_range(5.0..15.0);
            }
        }

        // 2. Step the physics and neural network
        network.step(&inputs);
        inputs.fill(0.0); // Reset inputs for next frame

        // 3. Render
        set_camera(&camera);

        // Draw bonds (Synapses)
        for &(parent_idx, child_idx) in &crystal.bonds {
            let p_atom = &crystal.atoms[parent_idx];
            let c_atom = &crystal.atoms[child_idx];

            let p1 = vec3(
                p_atom.position.x as f32,
                p_atom.position.y as f32,
                p_atom.position.z as f32,
            );
            let p2 = vec3(
                c_atom.position.x as f32,
                c_atom.position.y as f32,
                c_atom.position.z as f32,
            );

            let is_spiking_c = network.is_spiking(child_idx);

            let color = if is_spiking_c {
                Color::new(0.0, 1.0, 0.8, 0.8)
            } else {
                Color::new(0.3, 0.3, 0.4, 0.3)
            };

            draw_line_3d(p1, p2, color);
        }

        // Draw Atoms (Neurons)
        for (i, atom) in crystal.atoms.iter().enumerate() {
            let pos = vec3(
                atom.position.x as f32,
                atom.position.y as f32,
                atom.position.z as f32,
            );

            let is_spiking = network.is_spiking(i);

            let color = if is_spiking {
                Color::new(1.0, 0.8, 0.0, 1.0) // Bright yellow if spiking
            } else if atom.is_dir {
                Color::new(0.2, 0.6, 0.9, 0.8) // Blueish for directories
            } else {
                Color::new(0.6, 0.2, 0.8, 0.6) // Purple for files
            };

            let radius = if is_spiking { 0.4 } else { 0.2 };
            draw_sphere(pos, radius, None, color);
        }

        set_default_camera();

        // 2D UI
        draw_text(
            "Neural Codebase Morphogenesis",
            20.0,
            30.0,
            30.0,
            Color::new(0.8, 0.8, 0.9, 1.0),
        );
        draw_text(
            &format!(
                "Neurons: {} | Synapses: {}",
                num_atoms,
                network.synapses.len()
            ),
            20.0,
            60.0,
            20.0,
            Color::new(0.6, 0.6, 0.7, 1.0),
        );
        draw_text(
            "Press [SPACE] to stimulate the network",
            20.0,
            90.0,
            20.0,
            Color::new(1.0, 0.8, 0.0, 1.0),
        );

        next_frame().await;
    }
}
