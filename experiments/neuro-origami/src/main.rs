use glam::Vec3;
use macroquad::prelude::*;
use neuro_sim::Network;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

#[macroquad::main("Neuro-Origami: Neural Morphogenesis")]
async fn main() {
    let cols = 20;
    let rows = 20;

    // 1. Initialize Origami Mesh
    let params = MiuraParams {
        a: 0.8,
        b: 0.8,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.7);

    // 2. Initialize Physics System and Neural Network
    let mut pbd = PbdSystem::new();
    let mut brain = Network::new();
    let mut p_indices = Vec::with_capacity(points.len());
    let mut n_indices = Vec::with_capacity(points.len());

    // Add particles and neurons for each point
    for pos in &points {
        let p_idx = pbd.add_particle(Vec3::new(pos.x, pos.y, pos.z), 1.0);
        let n_idx = brain.add_neuron();
        p_indices.push(p_idx);
        n_indices.push(n_idx);
    }

    // Pin corners
    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    pbd.add_pin_constraint(
        p_indices[top_left],
        Vec3::new(points[top_left].x, points[top_left].y, points[top_left].z),
    );
    pbd.add_pin_constraint(
        p_indices[top_right],
        Vec3::new(
            points[top_right].x,
            points[top_right].y,
            points[top_right].z,
        ),
    );
    pbd.add_pin_constraint(
        p_indices[bottom_left],
        Vec3::new(
            points[bottom_left].x,
            points[bottom_left].y,
            points[bottom_left].z,
        ),
    );
    pbd.add_pin_constraint(
        p_indices[bottom_right],
        Vec3::new(
            points[bottom_right].x,
            points[bottom_right].y,
            points[bottom_right].z,
        ),
    );

    // 3. Connect Mesh (Actuator Constraints) and Synapses
    let mut constraint_mapping = Vec::new(); // Constraint idx -> (Neuron 1, Neuron 2, Original Rest Length)
    let stiffness = 0.5;

    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;

            // Horizontal connection
            if x < cols {
                let right = y * w + (x + 1);
                let pos1 = points[i];
                let pos2 = points[right];
                let dist = pos1.distance(pos2);

                // Physics constraint
                pbd.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    dist * 0.3,
                    dist * 1.5,
                    stiffness,
                );
                constraint_mapping.push((
                    pbd.constraints.len() - 1,
                    n_indices[i],
                    n_indices[right],
                    dist,
                ));

                // Neural synapses (bidirectional)
                brain.add_synapse(n_indices[i], n_indices[right], 2.0);
                brain.add_synapse(n_indices[right], n_indices[i], 2.0);
            }

            // Vertical connection
            if y < rows {
                let down = (y + 1) * w + x;
                let pos1 = points[i];
                let pos2 = points[down];
                let dist = pos1.distance(pos2);

                // Physics constraint
                pbd.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    dist * 0.3,
                    dist * 1.5,
                    stiffness,
                );
                constraint_mapping.push((
                    pbd.constraints.len() - 1,
                    n_indices[i],
                    n_indices[down],
                    dist,
                ));

                // Neural synapses (bidirectional)
                brain.add_synapse(n_indices[i], n_indices[down], 2.0);
                brain.add_synapse(n_indices[down], n_indices[i], 2.0);
            }

            // Diagonal connections to keep Miura-ori stable
            if x < cols && y < rows {
                let p00 = i;
                let p11 = (y + 1) * w + (x + 1);
                let dist1 = points[p00].distance(points[p11]);
                pbd.add_actuator_constraint(
                    p_indices[p00],
                    p_indices[p11],
                    dist1 * 0.5,
                    dist1 * 1.5,
                    stiffness,
                );
                constraint_mapping.push((
                    pbd.constraints.len() - 1,
                    n_indices[p00],
                    n_indices[p11],
                    dist1,
                ));

                let p10 = y * w + (x + 1);
                let p01 = (y + 1) * w + x;
                let dist2 = points[p10].distance(points[p01]);
                pbd.add_actuator_constraint(
                    p_indices[p10],
                    p_indices[p01],
                    dist2 * 0.5,
                    dist2 * 1.5,
                    stiffness,
                );
                constraint_mapping.push((
                    pbd.constraints.len() - 1,
                    n_indices[p10],
                    n_indices[p01],
                    dist2,
                ));
            }
        }
    }

    // Create random initial activity
    brain.neurons[n_indices[rows / 2 * w + cols / 2]].v = 30.0;

    let mut camera = Camera3D {
        position: vec3(0.0, 15.0, 15.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut rotation = 0.0f32;

    loop {
        let dt = get_frame_time().min(0.05);

        // 4. Bidirectional Feedback Loop

        // A. Read Physics state as Neural Input
        let mut inputs = vec![0.0; brain.neurons.len()];
        for &(c_idx, n1, n2, orig_dist) in &constraint_mapping {
            if let Constraint::Actuator { p1, p2, .. } = pbd.constraints[c_idx] {
                let pos1 = pbd.particles[p1].pos;
                let pos2 = pbd.particles[p2].pos;
                let current_dist = pos1.distance(pos2);
                let stretch = current_dist - orig_dist;

                // Stretch acts as excitatory input
                if stretch.abs() > 0.05 {
                    inputs[n1] += stretch * 2.0;
                    inputs[n2] += stretch * 2.0;
                }
            }
        }

        // B. Add some noise to keep the brain alive
        if rand::gen_range(0, 100) < 5 {
            let rx = rand::gen_range(0, cols);
            let ry = rand::gen_range(0, rows);
            inputs[ry * w + rx] += 20.0;
        }

        // C. Step the Brain
        brain.step(&inputs);

        // D. Write Neural state to Physics Constraints (Actuation)
        for c in &mut pbd.constraints {
            if let Constraint::Actuator {
                ref mut factor,
                p1,
                p2,
                ..
            } = c
            {
                let spike1 = brain.is_spiking(*p1);
                let spike2 = brain.is_spiking(*p2);

                if spike1 || spike2 {
                    // Contraction! Factor goes to 0 (min_length)
                    *factor = (*factor - 0.2).max(0.0);
                } else {
                    // Relaxation! Factor goes to 1 (max_length)
                    *factor = (*factor + 0.02).min(1.0);
                }
            }
        }

        // 5. Update Physics
        pbd.step(dt, 5);

        // 6. Rendering
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        rotation += 0.005;
        camera.position = vec3(rotation.sin() * 25.0, 15.0, rotation.cos() * 25.0);
        set_camera(&camera);

        // Draw Constraints
        for &(c_idx, _, _, _) in &constraint_mapping {
            if let Constraint::Actuator { p1, p2, factor, .. } = pbd.constraints[c_idx] {
                let pos1 = pbd.particles[p1].pos;
                let pos2 = pbd.particles[p2].pos;

                // Color based on factor (contraction state)
                // Red = Contracted (factor ~ 0.0), Blue = Relaxed (factor ~ 1.0)
                let color = Color::new(1.0 - factor, 0.2, factor, 1.0);

                draw_line_3d(
                    vec3(pos1.x, pos1.y, pos1.z),
                    vec3(pos2.x, pos2.y, pos2.z),
                    color,
                );
            }
        }

        // Draw Neurons (spiking = bright yellow)
        for (i, p) in pbd.particles.iter().enumerate() {
            if brain.is_spiking(n_indices[i]) {
                draw_sphere(
                    vec3(p.pos.x, p.pos.y, p.pos.z),
                    0.2,
                    None,
                    Color::new(1.0, 1.0, 0.0, 1.0),
                );
            }
        }

        set_default_camera();

        draw_text(
            "Neuro-Origami: Neural Morphogenesis",
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
