use macroquad::prelude::*;
use neuro_sim::Network;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

#[macroquad::main("Neuro-Origami: Spike-Driven Morphogenesis")]
async fn main() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Neural Network
    let mut net = Network::new();

    // Add neurons (one per grid cell)
    let num_neurons = cols * rows;
    for _ in 0..num_neurons {
        net.add_neuron();
    }

    // Connect neurons to their neighbors
    for y in 0..rows {
        for x in 0..cols {
            let i = y * cols + x;

            // Connect to right neighbor
            if x < cols - 1 {
                let right = y * cols + (x + 1);
                net.add_synapse(i, right, 15.0); // Excitatory
                net.add_synapse(right, i, 15.0);
            }

            // Connect to bottom neighbor
            if y < rows - 1 {
                let down = (y + 1) * cols + x;
                net.add_synapse(i, down, 15.0); // Excitatory
                net.add_synapse(down, i, 15.0);
            }
        }
    }

    // 2. Initialize Origami Mesh and Physics
    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.5);

    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    // Add particles
    for pos in &points {
        p_indices.push(system.add_particle(*pos, 1.0));
    }

    // Pin corners
    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    system.add_pin_constraint(p_indices[top_left], points[top_left]);
    system.add_pin_constraint(p_indices[top_right], points[top_right]);
    system.add_pin_constraint(p_indices[bottom_left], points[bottom_left]);
    system.add_pin_constraint(p_indices[bottom_right], points[bottom_right]);

    // Map constraints to neuron index
    let mut constraint_mapping = Vec::new();

    let stiffness = 0.8;
    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;

            if x < cols {
                let right = y * w + (x + 1);
                let dist = points[i].distance(points[right]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    dist * 0.2,
                    dist * 1.5,
                    stiffness,
                );
                let neuron_idx = y.min(rows - 1) * cols + x.min(cols - 1);
                constraint_mapping.push((system.constraints.len() - 1, neuron_idx));
            }

            if y < rows {
                let down = (y + 1) * w + x;
                let dist = points[i].distance(points[down]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    dist * 0.2,
                    dist * 1.5,
                    stiffness,
                );
                let neuron_idx = y.min(rows - 1) * cols + x.min(cols - 1);
                constraint_mapping.push((system.constraints.len() - 1, neuron_idx));
            }
        }
    }

    let mut camera = Camera3D {
        position: vec3(0.0, 15.0, 15.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut rotation = 0.0f32;
    let mut external_inputs = vec![0.0f32; num_neurons];
    let mut heat = vec![0.0f32; num_neurons];

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Inject random noise/current
        external_inputs.fill(0.0);
        if rand::gen_range(0.0, 1.0) < 0.1 {
            let i = rand::gen_range(0, num_neurons);
            external_inputs[i] = 100.0;
        }

        // Update SNN
        net.step(&external_inputs);

        // Track neuron firing heat
        for i in 0..num_neurons {
            if net.is_spiking(i) {
                heat[i] = 1.0;
            } else {
                heat[i] *= 0.95;
            }
        }

        // Apply heat/spikes to physical actuator constraints
        for &(c_idx, neuron_idx) in &constraint_mapping {
            let spike_heat = heat[neuron_idx];

            if let Constraint::Actuator { ref mut factor, .. } = system.constraints[c_idx] {
                // High activity -> factor approaches 0.0 (contraction)
                // Low activity -> factor relaxes to 1.0 (expansion)
                let target = 1.0 - spike_heat * 0.8;
                *factor = *factor * 0.9 + target * 0.1;
            }
        }

        // Update Physics
        system.step(0.016, 5);

        // Update Camera
        rotation += 0.005;
        camera.position = vec3(rotation.sin() * 20.0, 15.0, rotation.cos() * 20.0);
        set_camera(&camera);

        // Render Wireframe Mesh
        for y in 0..rows {
            for x in 0..cols {
                let i = y * w + x;
                let neuron_idx = y * cols + x;

                let p00 = system.particles[p_indices[i]].pos;
                let p10 = system.particles[p_indices[i + 1]].pos;
                let p01 = system.particles[p_indices[i + w]].pos;

                let h = heat[neuron_idx];

                // Color maps to neural heat (Red when firing, fading to cool blue)
                let r = h;
                let g = h * 0.2 + 0.1;
                let b = (1.0 - h) * 0.8;

                let color = Color::new(r, g, b, 1.0);

                draw_line_3d(p00, p10, color);
                draw_line_3d(p00, p01, color);
            }
        }

        for x in 0..cols {
            let i = rows * w + x;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + 1]].pos;
            draw_line_3d(p0, p1, Color::new(0.2, 0.2, 0.4, 1.0));
        }
        for y in 0..rows {
            let i = y * w + cols;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + w]].pos;
            draw_line_3d(p0, p1, Color::new(0.2, 0.2, 0.4, 1.0));
        }

        set_default_camera();

        // Check for headless mode
        if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
            return;
        }

        draw_text("Neuro-Origami", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Spiking Neural Network actuates Miura-ori mesh constraints.",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
