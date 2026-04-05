use glam::Vec3;
use macroquad::prelude::*;
use neuro_sim::Network;
use physics_pbd::{Constraint, PbdSystem};

const GRID_COLS: usize = 10;
const GRID_ROWS: usize = 10;
const SPACING: f32 = 30.0;
const REST_LENGTH: f32 = SPACING;
const CONTRACTION_LENGTH: f32 = SPACING * 0.5;

struct Node {
    pbd_idx: usize,
    neuron_idx: usize,
    _x: usize,
    _y: usize,
}

#[macroquad::main("Neuro Tissue")]
async fn main() {
    let mut pbd = PbdSystem::new();
    let mut brain = Network::new();
    let mut nodes = Vec::new();

    let start_x = screen_width() / 2.0 - (GRID_COLS as f32 * SPACING) / 2.0;
    let start_y = screen_height() / 2.0 - (GRID_ROWS as f32 * SPACING) / 2.0;

    // 1. Initialize Grid of Particles and Neurons
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let pos = Vec3::new(
                start_x + col as f32 * SPACING,
                start_y + row as f32 * SPACING,
                0.0,
            );

            // Pin the top row to hang the tissue
            let mass = if row == 0 { 0.0 } else { 1.0 };
            let pbd_idx = pbd.add_particle(pos, mass);
            let neuron_idx = brain.add_neuron();

            nodes.push(Node {
                pbd_idx,
                neuron_idx,
                _x: col,
                _y: row,
            });

            // Connect neurons arbitrarily to create some internal dynamics
            if row > 0 {
                let up_neuron = nodes[(row - 1) * GRID_COLS + col].neuron_idx;
                brain.add_synapse(neuron_idx, up_neuron, 2.5); // Feedback up
            }
            if col > 0 {
                let left_neuron = nodes[row * GRID_COLS + col - 1].neuron_idx;
                brain.add_synapse(neuron_idx, left_neuron, 2.5); // Feedback left
            }
        }
    }

    // 2. Initialize PBD Constraints (Structural Mesh)
    // Connect adjacent nodes horizontally and vertically
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let idx = row * GRID_COLS + col;
            let p1 = nodes[idx].pbd_idx;

            if col < GRID_COLS - 1 {
                let right_idx = row * GRID_COLS + col + 1;
                let p2 = nodes[right_idx].pbd_idx;
                pbd.add_distance_constraint(p1, p2, REST_LENGTH);
            }
            if row < GRID_ROWS - 1 {
                let down_idx = (row + 1) * GRID_COLS + col;
                let p2 = nodes[down_idx].pbd_idx;
                pbd.add_distance_constraint(p1, p2, REST_LENGTH);
            }
        }
    }

    // Gravity
    let gravity = Vec3::new(0.0, 98.1, 0.0);

    // Track original rest lengths to restore them after contraction
    let original_rest_lengths: Vec<f32> = pbd
        .constraints
        .iter()
        .map(|c| match c {
            Constraint::Distance { rest_length, .. } => *rest_length,
            _ => REST_LENGTH,
        })
        .collect();

    loop {
        let dt = get_frame_time().min(0.05); // Cap delta time

        // --- Physics Update ---
        // Apply gravity
        for p in &mut pbd.particles {
            if p.inv_mass > 0.0 {
                p.vel += gravity * dt;
            }
        }

        // Apply mouse interaction (drag closest particle)
        let (mx, my) = mouse_position();
        let mouse_pos = Vec3::new(mx, my, 0.0);
        if is_mouse_button_down(MouseButton::Left) {
            let mut closest_idx = 0;
            let mut min_dist = f32::MAX;
            for (i, p) in pbd.particles.iter().enumerate() {
                if p.inv_mass > 0.0 {
                    let dist = p.pos.distance_squared(mouse_pos);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_idx = i;
                    }
                }
            }
            if min_dist < 10000.0 {
                pbd.particles[closest_idx].pos = mouse_pos;
                pbd.particles[closest_idx].vel = Vec3::ZERO;
            }
        }

        pbd.step(dt, 10);

        // --- Bidirectional Feedback: Brain <-> Physics ---

        // 1. Read Physics state as Neural Input
        let mut inputs = vec![0.0; brain.neurons.len()];
        for (i, c) in pbd.constraints.iter().enumerate() {
            if let Constraint::Distance { p1, p2, .. } = c {
                let pos1 = pbd.particles[*p1].pos;
                let pos2 = pbd.particles[*p2].pos;
                let current_dist = pos1.distance(pos2);
                let stretch = current_dist - original_rest_lengths[i];

                // Map stretch to input current.
                // Find which neurons correspond to these particles
                // Optimization: just use pbd_idx == neuron_idx since we created them 1:1
                if stretch.abs() > 2.0 {
                    inputs[*p1] += stretch * 0.5;
                    inputs[*p2] += stretch * 0.5;
                }
            }
        }

        // 2. Step the Brain
        brain.step(&inputs);

        // 3. Write Neural state to Physics Constraints (Contraction)
        for c in &mut pbd.constraints {
            if let Constraint::Distance {
                p1,
                p2,
                rest_length,
                ..
            } = c
            {
                let spike1 = brain.is_spiking(*p1);
                let spike2 = brain.is_spiking(*p2);

                if spike1 || spike2 {
                    // Contract
                    *rest_length = CONTRACTION_LENGTH;
                } else {
                    // Relax back to original
                    *rest_length += (REST_LENGTH - *rest_length) * 0.1;
                }
            }
        }

        // --- Rendering ---
        clear_background(BLACK);

        // Draw Constraints
        for c in &pbd.constraints {
            if let Constraint::Distance {
                p1,
                p2,
                rest_length,
                ..
            } = c
            {
                let pos1 = pbd.particles[*p1].pos;
                let pos2 = pbd.particles[*p2].pos;

                // Color based on contraction
                let t = (*rest_length - CONTRACTION_LENGTH) / (REST_LENGTH - CONTRACTION_LENGTH);
                let t = t.clamp(0.0, 1.0);

                // Red = Contracted, Blue = Relaxed
                let color = Color::new(1.0 - t, 0.0, t, 1.0);

                draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, color);
            }
        }

        // Draw Particles / Neurons
        for node in &nodes {
            let pos = pbd.particles[node.pbd_idx].pos;
            let is_spiking = brain.is_spiking(node.neuron_idx);

            let color = if is_spiking { YELLOW } else { DARKGRAY };
            let radius = if is_spiking { 5.0 } else { 3.0 };

            draw_circle(pos.x, pos.y, radius, color);
        }

        // Draw UI
        draw_text("Neuro Tissue", 10.0, 30.0, 30.0, WHITE);
        draw_text("Neurons Spike -> Tissue Contracts", 10.0, 60.0, 20.0, GRAY);
        draw_text("Tissue Stretch -> Neural Input", 10.0, 80.0, 20.0, GRAY);
        draw_text("Drag with Mouse", 10.0, 100.0, 20.0, YELLOW);

        next_frame().await;
    }
}
