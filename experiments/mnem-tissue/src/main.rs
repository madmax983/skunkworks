use glam::Vec3;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};

mod graph;
use graph::Graph;
mod glitch;
use glitch::TextGlitcher;

// Constants for physics
const SUBSTEPS: usize = 10;
const GRAVITY: Vec3 = Vec3::new(0.0, 98.1, 0.0); // Droop downwards due to gravity

#[macroquad::main("Mnemonic Tissue")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Scan
    graph.scan_directory("experiments/mnem-rot/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut system = PbdSystem::new();
    let mut particle_map = vec![];

    // Center screen projection anchor
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    // Lineage integration: Create soft-body tissue out of the codebase graph
    for _node in &graph.nodes {
        // Place particles randomly in a blob around the center, z=0
        let pos = Vec3::new(
            center_x + rand::gen_range(-100.0, 100.0),
            center_y + rand::gen_range(-100.0, 100.0),
            0.0,
        );
        let mass = 1.0;
        let p_idx = system.add_particle(pos, mass);
        particle_map.push(p_idx);
    }

    // Map edges to distance constraints (tissue bonds)
    for edge in &graph.edges {
        if edge.from < particle_map.len() && edge.to < particle_map.len() {
            let p1 = particle_map[edge.from];
            let p2 = particle_map[edge.to];
            let dist = 80.0; // Desired rest length between related files
            system.add_distance_constraint(p1, p2, dist);
        }
    }

    // Pin the root node (assuming node 0 is something central or we just pin the first one)
    if !particle_map.is_empty() {
        system.add_pin_constraint(particle_map[0], Vec3::new(center_x, center_y - 200.0, 0.0));
    }

    let mut entropy = 0.0;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    loop {
        let dt = get_frame_time().min(0.05); // Cap dt
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        // Input
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_vec;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_vec - last_mouse;
            offset += delta;
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
        }

        let world_mouse = (mouse_vec - offset) / zoom;
        let mut hovered_node = None;

        // Decay logic
        entropy += 0.0001;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            // Decay
            node.health -= entropy * dt * 0.05;
            if node.health < 0.0 {
                node.health = 0.0;
            }

            // Sync graph pos to PBD
            let p_idx = particle_map[i];
            let p_pos = system.particles[p_idx].pos;
            node.pos = vec2(p_pos.x, p_pos.y);

            // Interaction
            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                // Heal
                node.health += dt * 5.0;
                if node.health > 1.0 {
                    node.health = 1.0;
                }

                // Mouse pull: user can drag tissue around if holding left click
                if is_mouse_button_down(MouseButton::Left) {
                    system.particles[p_idx].pos = Vec3::new(world_mouse.x, world_mouse.y, 0.0);
                    system.particles[p_idx].vel = Vec3::ZERO;
                }
            }
        }

        // --- Lineage: Entropy -> Tissue Integrity ---
        // As nodes decay, their stiffness weakens. Once a node is completely dead (health < 0.1),
        // its constraints to other nodes effectively vanish (stiffness = 0.0), causing the organ to tear.

        for (edge_idx, edge) in graph.edges.iter().enumerate() {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let h1 = graph.nodes[edge.from].health;
                let h2 = graph.nodes[edge.to].health;
                let avg_health = (h1 + h2) / 2.0;

                // We assume constraints correspond 1:1 with edges (plus the pin at the end of constraints, so we need to map carefully)
                // Actually, PbdSystem allows dynamic constraints, but `add_distance_constraint` pushes to the end.
                // Let's just modify the constraints in place.
                if edge_idx < system.constraints.len() {
                    // The first N constraints are the edges
                    if let Constraint::Distance {
                        p1: _,
                        p2: _,
                        rest_length: _,
                        stiffness,
                    } = &mut system.constraints[edge_idx]
                    {
                        if avg_health < 0.1 {
                            *stiffness = 0.0; // Torn
                        } else {
                            *stiffness = avg_health * edge.strength; // Weakening
                        }
                    }
                }
            }
        }

        // Apply external forces (Gravity & slight random Brownian motion for organic feel)
        for i in 0..system.particles.len() {
            // Only apply gravity if they aren't pinned
            if system.particles[i].inv_mass > 0.0 {
                system.particles[i].vel += GRAVITY * dt;

                // Brownian motion / squirm
                system.particles[i].vel += Vec3::new(
                    rand::gen_range(-10.0, 10.0),
                    rand::gen_range(-10.0, 10.0),
                    0.0,
                );
            }
        }

        // Step physics
        system.step(dt, SUBSTEPS);

        // Rendering
        clear_background(Color::new(0.02, 0.02, 0.05, 1.0)); // Dark bio fluid

        // Draw edges (Tissue bonds)
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                if avg_health >= 0.1 {
                    // If not torn
                    let jitter = if avg_health < 0.5 {
                        vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
                    } else {
                        vec2(0.0, 0.0)
                    };

                    draw_line(
                        n1.x + jitter.x,
                        n1.y + jitter.y,
                        n2.x + jitter.x,
                        n2.y + jitter.y,
                        4.0 * zoom * avg_health, // Thicker when healthy
                        Color::new(0.6, 0.2, 0.3, avg_health), // Bloody/fleshy color
                    );
                }
            }
        }

        // Draw nodes (Cells/Organs)
        for (i, node) in graph.nodes.iter().enumerate() {
            let pos = node.pos * zoom + offset;

            // Fleshy coloring based on health
            let color = if node.health > 0.8 {
                Color::new(0.8, 0.4, 0.5, 1.0) // Healthy pink tissue
            } else if node.health > 0.3 {
                Color::new(0.5, 0.5, 0.2, 1.0) // Necrotic yellow/green
            } else {
                Color::new(0.2, 0.1, 0.1, 1.0) // Dead black
            };

            let radius = 10.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            // Pulse effect
            if node.health > 0.5 {
                let pulse = (get_time() as f32 * 5.0 + i as f32).sin() * 2.0;
                draw_circle_lines(
                    pos.x,
                    pos.y,
                    radius + pulse,
                    1.0,
                    Color::new(1.0, 0.8, 0.8, 0.3),
                );
            }

            // Draw name
            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + radius + 5.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        // UI Overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            draw_rectangle(
                10.0,
                10.0,
                400.0,
                screen_height() - 20.0,
                Color::new(0.05, 0.05, 0.05, 0.95),
            );
            draw_rectangle_lines(10.0, 10.0, 400.0, screen_height() - 20.0, 2.0, WHITE);

            draw_text(&node.name, 20.0, 40.0, 30.0, Color::new(0.8, 0.4, 0.5, 1.0));
            draw_text(
                &format!("Integrity: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                let display_line: String = line.chars().take(50).collect();
                draw_text(
                    &display_line,
                    20.0,
                    100.0 + j as f32 * 15.0,
                    14.0,
                    LIGHTGRAY,
                );
            }
        }

        draw_text(
            &format!("Tissue Rot (Entropy): {:.4}", entropy),
            screen_width() - 300.0,
            30.0,
            20.0,
            RED,
        );
        draw_text(
            "Right Click: Pan | Scroll: Zoom | Hover: Heal | L-Click: Drag",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await
    }
}
