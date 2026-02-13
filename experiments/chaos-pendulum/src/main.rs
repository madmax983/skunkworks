use ::rand::Rng;
use macroquad::prelude::*;
use rayon::prelude::*;

mod loader;
mod physics;

use physics::PendulumSystem;

#[macroquad::main("Chaos Pendulum")]
async fn main() {
    // Load the "Real" system
    let mut real_system = match loader::load_dependencies() {
        Ok(sys) => {
            println!(
                "Loaded {} nodes and {} links",
                sys.nodes.len(),
                sys.links.len()
            );
            sys
        }
        Err(e) => {
            eprintln!("Failed to load dependencies: {}", e);
            PendulumSystem::new()
        }
    };

    // Save initial state for reset
    let initial_system = real_system.clone();

    // Initialize "Ghosts"
    let ghost_count = 100;
    // We create a local scope rng for main, but init_ghosts needs its own logic
    let mut rng = ::rand::thread_rng();

    // Helper to init ghosts
    let init_ghosts = |base_sys: &PendulumSystem, count: usize| -> Vec<PendulumSystem> {
        // We use par_iter logic inside or just standard iter?
        // Standard iter is fast enough for 100 ghosts init
        let mut rng = ::rand::thread_rng();
        (0..count)
            .map(|_| {
                let mut g = base_sys.clone();
                for node in &mut g.nodes {
                    if !node.fixed {
                        let offset_x = rng.gen_range(-0.5..0.5);
                        let offset_y = rng.gen_range(-0.5..0.5);
                        node.pos.x += offset_x;
                        node.pos.y += offset_y;
                        node.prev_pos.x += offset_x;
                        node.prev_pos.y += offset_y;
                    }
                }
                g
            })
            .collect()
    };

    let mut ghosts = init_ghosts(&real_system, ghost_count);

    let mut zoom = 1.0;
    let mut target = Vec2::new(600.0, 400.0);
    let mut show_ghosts = true;

    loop {
        let dt = get_frame_time();

        // --- Input Handling ---

        // Camera (WASD)
        if is_key_down(KeyCode::D) { target.x += 10.0 / zoom; }
        if is_key_down(KeyCode::A) { target.x -= 10.0 / zoom; }
        if is_key_down(KeyCode::S) { target.y += 10.0 / zoom; }
        if is_key_down(KeyCode::W) { target.y -= 10.0 / zoom; }
        if is_key_down(KeyCode::Equal) { zoom *= 1.05; }
        if is_key_down(KeyCode::Minus) { zoom *= 0.95; }
        if is_key_pressed(KeyCode::G) { show_ghosts = !show_ghosts; }

        // Physics Parameters (Arrows)
        // Gravity Y (Up/Down)
        if is_key_down(KeyCode::Up) {
             real_system.gravity.y -= 0.1;
             for g in &mut ghosts { g.gravity.y = real_system.gravity.y; }
        }
        if is_key_down(KeyCode::Down) {
             real_system.gravity.y += 0.1;
             for g in &mut ghosts { g.gravity.y = real_system.gravity.y; }
        }
        // Friction (Left/Right)
        if is_key_down(KeyCode::Right) { // Less friction (higher value)
             real_system.friction = (real_system.friction + 0.001).min(1.0);
             for g in &mut ghosts { g.friction = real_system.friction; }
        }
        if is_key_down(KeyCode::Left) { // More friction (lower value)
             real_system.friction = (real_system.friction - 0.001).max(0.0);
             for g in &mut ghosts { g.friction = real_system.friction; }
        }

        // Reset (R)
        if is_key_pressed(KeyCode::R) {
             real_system = initial_system.clone();
             ghosts = init_ghosts(&real_system, ghost_count);
        }

        // Kick (K)
        if is_key_pressed(KeyCode::K) {
             let kick_strength = 50.0; // Stronger kick
             // Kick Real
             for node in &mut real_system.nodes {
                 if !node.fixed {
                     let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                     let force = Vec2::new(angle.cos(), angle.sin()) * kick_strength;
                     node.prev_pos -= force * dt;
                 }
             }
             // Kick Ghosts (differently)
             for g in &mut ghosts {
                 for node in &mut g.nodes {
                     if !node.fixed {
                         let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                         let force = Vec2::new(angle.cos(), angle.sin()) * kick_strength;
                         node.prev_pos -= force * dt;
                     }
                 }
             }
        }

        // --- Physics Update ---
        real_system.step(dt);
        ghosts.par_iter_mut().for_each(|g| g.step(dt));

        // --- Interaction Logic (Mouse) ---
        let mouse_pos = mouse_position();
        let mouse_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
        let world_mouse = (mouse_vec - Vec2::new(screen_width() / 2.0, screen_height() / 2.0))
            / zoom
            + target;

        let mut closest_dist = 20.0 / zoom;
        let mut hovered_idx = None;

        for (i, node) in real_system.nodes.iter().enumerate() {
            let dist = node.pos.distance(world_mouse);
            if dist < closest_dist {
                closest_dist = dist;
                hovered_idx = Some(i);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(idx) = hovered_idx {
                real_system.nodes[idx].pos = world_mouse;
                real_system.nodes[idx].prev_pos = world_mouse;

                if is_key_down(KeyCode::Space) { // Drag all
                     for g in &mut ghosts {
                        g.nodes[idx].pos = world_mouse;
                        g.nodes[idx].prev_pos = world_mouse;
                    }
                }
            }
        }

        // --- Render ---
        clear_background(BLACK);

        set_camera(&Camera2D {
            target,
            zoom: Vec2::new(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw Ghosts
        if show_ghosts {
            let alpha = 0.05;
            let ghost_color = Color::new(0.0, 1.0, 1.0, alpha);
            for g in &ghosts {
                for link in &g.links {
                    let pos_a = g.nodes[link.a].pos;
                    let pos_b = g.nodes[link.b].pos;
                    draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 0.5 / zoom, ghost_color);
                }
            }
        }

        // Draw Real System Links
        for link in &real_system.links {
            let pos_a = real_system.nodes[link.a].pos;
            let pos_b = real_system.nodes[link.b].pos;
            draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 2.0 / zoom, LIGHTGRAY);
        }

        // Draw Real System Nodes (Color Coded)
        for (i, node) in real_system.nodes.iter().enumerate() {
            let color = if node.fixed {
                RED
            } else if Some(i) == hovered_idx {
                YELLOW
            } else if node.mass <= 1.2 {
                GREEN
            } else {
                BLUE
            };

            let size = (node.mass * 2.0).clamp(3.0, 15.0);
            draw_circle(node.pos.x, node.pos.y, size / zoom, color);
        }

        set_default_camera();

        // Draw Hover Name
        if let Some(idx) = hovered_idx {
             let node = &real_system.nodes[idx];
             draw_text(&node.name, mouse_pos.0 + 15.0, mouse_pos.1, 20.0, YELLOW);
        }

        // --- Metrics ---
        let mut total_divergence = 0.0;
        let mut count = 0;
        for g in &ghosts {
            for (i, node) in g.nodes.iter().enumerate() {
                if !node.fixed {
                    total_divergence += (real_system.nodes[i].pos - node.pos).length();
                    count += 1;
                }
            }
        }
        let avg_divergence = if count > 0 { total_divergence / count as f32 } else { 0.0 };

        draw_text("CHAOS PENDULUM", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Nodes: {} | Ghosts: {}", real_system.nodes.len(), ghosts.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
         draw_text(
            &format!("Gravity: {:.1} | Friction: {:.3}", real_system.gravity.y, real_system.friction),
            20.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
         draw_text(
            &format!("Divergence: {:.2}", avg_divergence),
            20.0,
            100.0,
            20.0,
            Color::new(1.0, 0.5, 0.0, 1.0),
        );
         draw_text(
            "WASD: Cam | Arrows: Phys | K: Kick | R: Reset | G: Ghosts",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
