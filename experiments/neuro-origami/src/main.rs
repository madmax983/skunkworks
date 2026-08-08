//! 🧬 Splice: neuro-origami
//!
//! Lineage:
//! - Parent A (neuro-sim): Provides the biologically plausible Spiking Neural Network (Izhikevich model).
//! - Parent B (origami): Provides the continuous procedural soft-body mesh (Miura-ori crease pattern).
//!
//! Emergent Phenotype: Neural Morphogenesis. The `neuro-sim` network drives the real-time folding and structural properties of the `origami` mesh. When neurons spike, their electrical activity alters the `extension_factor` of the Miura-ori mesh, causing the geometry to physically breathe and fold in response to cognitive loads and action potentials.

use macroquad::prelude::*;
use neuro_sim::Network;
use origami::{generate_miura_mesh, MiuraParams, Orientation};

fn conf() -> Conf {
    Conf {
        window_title: "Neuro Origami".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad initialization.");
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        run_headless();
        return;
    }
    macroquad::Window::from_config(conf(), async_main());
}

async fn async_main() {
    let mut network = Network::new();
    let num_neurons = 100;
    let mut neurons = vec![];

    // Create a network of neurons
    for _ in 0..num_neurons {
        neurons.push(network.add_neuron());
    }

    // Connect randomly
    for i in 0..num_neurons {
        for _ in 0..5 {
            let target = macroquad::rand::gen_range(0, num_neurons);
            if target != i {
                // Random weight between -20 and 20
                let weight = macroquad::rand::gen_range(-20.0, 20.0);
                network.add_synapse(neurons[i], neurons[target], weight);
            }
        }
    }

    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    let grid_size = (10, 10);
    let mut current_extension = 0.5;

    // Setup camera
    let mut camera = Camera3D {
        position: vec3(0.0, 5.0, 15.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        // Rotate camera slowly
        let time = get_time() as f32;
        camera.position.x = time.cos() * 15.0;
        camera.position.z = time.sin() * 15.0;

        // Inject random current to stimulate spikes
        let mut inputs = vec![0.0; num_neurons];
        for input in inputs.iter_mut() {
            if macroquad::rand::gen_range(0, 100) < 5 {
                *input = macroquad::rand::gen_range(10.0, 50.0);
            }
        }

        network.step(&inputs);

        // Count spikes to determine structural stress
        let mut total_spikes = 0;
        for &id in &neurons {
            if network.is_spiking(id) {
                total_spikes += 1;
            }
        }

        // Target extension based on spike activity (more spikes = contracted, fewer = expanded)
        let spike_ratio = (total_spikes as f32 / num_neurons as f32).min(1.0);
        let target_extension = 1.0 - (spike_ratio * 3.0).clamp(0.0, 0.8);

        // Smoothly interpolate to target extension
        current_extension = current_extension * 0.9 + target_extension * 0.1;

        // Generate the folded mesh based on neural activity
        let mesh = generate_miura_mesh(params, grid_size, current_extension);

        set_camera(&camera);

        // Draw the mesh as lines
        for i in (0..mesh.indices.len()).step_by(3) {
            let i1 = mesh.indices[i] as usize;
            let i2 = mesh.indices[i + 1] as usize;
            let i3 = mesh.indices[i + 2] as usize;

            if i1 < mesh.vertices.len() && i2 < mesh.vertices.len() && i3 < mesh.vertices.len() {
                let v1 = mesh.vertices[i1].pos;
                let v2 = mesh.vertices[i2].pos;
                let v3 = mesh.vertices[i3].pos;

                // Color based on activity level
                let color = Color::new(
                    0.2 + spike_ratio * 0.8,
                    0.5 - spike_ratio * 0.5,
                    0.8 - spike_ratio * 0.4,
                    1.0,
                );

                draw_line_3d(v1, v2, color);
                draw_line_3d(v2, v3, color);
                draw_line_3d(v3, v1, color);
            }
        }

        set_default_camera();

        // Draw HUD
        draw_text(format!("Neurons: {}", num_neurons), 20.0, 30.0, 20.0, WHITE);
        draw_text(format!("Spikes: {}", total_spikes), 20.0, 55.0, 20.0, WHITE);
        draw_text(
            format!("Mesh Extension: {:.2}", current_extension),
            20.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
