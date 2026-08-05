//! # neuro-physics
//!
//! **Parents**: `crates/neuro-sim` + `crates/physics-pbd`
//! **Concept**: Neural Muscle Contraction.
//!
//! ## Traits
//! * **Lineage from `neuro-sim`**: Biological Spiking Neural Network evaluating Izhikevich neurons.
//! * **Lineage from `physics-pbd`**: Soft-body continuous position-based dynamics via constraints and springs.
//! * **Phenotype**: This hybrid visually maps neural firing spikes directly into physical muscle contractions. A grid of neurons is simulated, and their states actively drive the constraints of a hanging soft-body tissue mesh.
//!
//! ## Quick Start
//! ```bash
//! cargo run -p neuro-physics --headless
//! ```
//!
use std::env;

use ::glam::Vec3;
use neuro_sim::Network;
use physics_pbd::{Constraint, PbdSystem};

// To avoid conflicts with macroquad's exported types:
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Neuro Physics".to_owned(),
        ..Default::default()
    }
}

async fn run_sim() {
    let mut brain = Network::new();
    let mut body = PbdSystem::new();
    let mut node_to_neuron = Vec::new();
    let mut rng = thread_rng();

    let width = 5;
    let height = 5;
    let mut p_indices = vec![];
    let spacing = 20.0;
    let start_x = -(width as f32) * spacing / 2.0;
    let start_y = 100.0;

    // Create particles and neurons
    for y in 0..height {
        for x in 0..width {
            let is_pinned = y == 0;
            let mass = if is_pinned { 0.0 } else { 1.0 };
            let pos = Vec3::new(
                start_x + (x as f32) * spacing,
                start_y - (y as f32) * spacing,
                0.0,
            );
            let p_idx = body.add_particle(pos, mass);
            if is_pinned {
                let _ = body.add_pin_constraint(p_idx, pos);
            }
            p_indices.push(p_idx);

            let n_idx = brain.add_neuron();
            brain.neurons[n_idx].a = 0.02 + rng.gen_range(0.0..0.02);
            brain.neurons[n_idx].b = 0.2;
            brain.neurons[n_idx].c = -65.0;
            brain.neurons[n_idx].d = 8.0;
            node_to_neuron.push(n_idx);
        }
    }

    // Connect particles and neurons
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;

            if x < width - 1 {
                let right_idx = y * width + (x + 1);
                let _ = body.add_actuator_constraint(
                    p_indices[idx],
                    p_indices[right_idx],
                    spacing * 0.5,
                    spacing,
                    1.0,
                );
                brain.add_synapse(
                    node_to_neuron[idx],
                    node_to_neuron[right_idx],
                    rng.gen_range(5.0..10.0),
                );
                brain.add_synapse(
                    node_to_neuron[right_idx],
                    node_to_neuron[idx],
                    rng.gen_range(5.0..10.0),
                );
            }
            if y < height - 1 {
                let down_idx = (y + 1) * width + x;
                let _ = body.add_actuator_constraint(
                    p_indices[idx],
                    p_indices[down_idx],
                    spacing * 0.5,
                    spacing,
                    1.0,
                );
                brain.add_synapse(
                    node_to_neuron[idx],
                    node_to_neuron[down_idx],
                    rng.gen_range(5.0..10.0),
                );
                brain.add_synapse(
                    node_to_neuron[down_idx],
                    node_to_neuron[idx],
                    rng.gen_range(5.0..10.0),
                );
            }
        }
    }

    let camera = Camera3D {
        position: vec3(0.0, 50.0, 150.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 50.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        let mut inputs = vec![0.0; brain.neurons.len()];

        for (i, input) in inputs.iter_mut().enumerate().take(brain.neurons.len()) {
            if rng.gen_bool(0.02) {
                *input = 20.0;
            } else {
                let p_idx = i;
                let vel = body.particles[p_idx].vel;
                *input = vel.length() * 0.5;
            }
        }

        brain.step(&inputs);

        for c in body.constraints.iter_mut() {
            if let Constraint::Actuator { p1, p2, factor, .. } = c {
                let n1 = node_to_neuron[*p1];
                let n2 = node_to_neuron[*p2];

                if brain.spikes[n1] || brain.spikes[n2] {
                    *factor = 0.0; // Contract
                } else {
                    *factor = (*factor + 0.1).min(1.0); // Relax
                }
            }
        }

        for p in &mut body.particles {
            if p.inv_mass > 0.0 {
                p.vel.y -= 0.5; // Gravity
            }
        }
        body.step(0.016, 5);

        set_camera(&camera);

        for c in body.constraints.iter() {
            if let Constraint::Actuator { p1, p2, factor, .. } = c {
                let pos1 = body.particles[*p1].pos;
                let pos2 = body.particles[*p2].pos;

                let color = if *factor < 0.5 { RED } else { BLUE };

                draw_line_3d(
                    vec3(pos1.x, pos1.y, pos1.z),
                    vec3(pos2.x, pos2.y, pos2.z),
                    color,
                );
            }
        }

        for (i, p) in body.particles.iter().enumerate() {
            let is_spiking = brain.spikes[node_to_neuron[i]];
            let color = if is_spiking { YELLOW } else { WHITE };

            draw_sphere(vec3(p.pos.x, p.pos.y, p.pos.z), 1.0, None, color);
        }

        set_default_camera();

        draw_text(
            "Neural Muscle Contraction (neuro-physics)",
            10.0,
            20.0,
            30.0,
            WHITE,
        );

        let mut active_spikes = 0;
        for &s in &brain.spikes {
            if s {
                active_spikes += 1;
            }
        }

        draw_text(
            format!("Neurons: {}", brain.neurons.len()),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Spiking: {}", active_spikes),
            10.0,
            70.0,
            20.0,
            YELLOW,
        );
        draw_text(
            format!("Muscles: {}", body.constraints.len()),
            10.0,
            90.0,
            20.0,
            BLUE,
        );

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--headless")) {
        println!("Headless mode: exiting early to prevent CI timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
