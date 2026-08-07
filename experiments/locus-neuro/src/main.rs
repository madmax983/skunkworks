//! 🧬 Splice: locus-neuro
//!
//! Lineage:
//! - Parent A (locus): Provides topological boundaries (`Topology::Torus`).
//! - Parent B (neuro-sim): Provides the biological Spiking Neural Network (`Network`).
//!
//! Emergent Phenotype: Topological Brain Simulation. The physical neurons of the SNN are mapped onto a 2D grid wrapped in a continuous Torus topology. Action potentials (spikes) propagating out of one side of the 2D grid wrap around and stimulate neurons on the opposite side.

use locus::{Topology, Vec2};
use macroquad::prelude::*;
use neuro_sim::Network;

fn conf() -> Conf {
    Conf {
        window_title: "Locus Neuro".to_owned(),
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
    let width = 20;
    let height = 15;
    let topology = Topology::Torus;
    let mut neurons = vec![];

    // Create a grid of neurons
    for y in 0..height {
        for x in 0..width {
            let id = network.add_neuron();
            neurons.push((id, Vec2::new(x as f64, y as f64)));
        }
    }

    // Connect neighbors
    for y in 0..height {
        for x in 0..width {
            let current_idx = y * width + x;
            let current_id = neurons[current_idx].0;

            // Connect to right neighbor (wrapping)
            if let Some((ny, nx)) = topology.normalize(y as i64, (x + 1) as i64, width, height) {
                let neighbor_idx = ny * width + nx;
                let neighbor_id = neurons[neighbor_idx].0;
                network.add_synapse(current_id, neighbor_id, 10.0);
            }

            // Connect to bottom neighbor (wrapping)
            if let Some((ny, nx)) = topology.normalize((y + 1) as i64, x as i64, width, height) {
                let neighbor_idx = ny * width + nx;
                let neighbor_id = neurons[neighbor_idx].0;
                network.add_synapse(current_id, neighbor_id, 10.0);
            }
        }
    }

    loop {
        clear_background(BLACK);

        // Inject random current into a random neuron to keep it alive
        let mut inputs = vec![0.0; neurons.len()];
        if macroquad::rand::gen_range(0, 10) == 0 {
            let target = macroquad::rand::gen_range(0, neurons.len());
            inputs[target] = 30.0;
        }

        network.step(&inputs);

        let cell_width = 800.0 / width as f32;
        let cell_height = 600.0 / height as f32;

        for (id, pos) in &neurons {
            let x = pos.x as f32 * cell_width;
            let y = pos.y as f32 * cell_height;

            let color = if network.is_spiking(*id) {
                WHITE
            } else {
                DARKGRAY
            };

            draw_rectangle(x, y, cell_width - 2.0, cell_height - 2.0, color);
        }

        next_frame().await;
    }
}
