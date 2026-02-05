use ::rand::Rng;
use macroquad::prelude::*;
use rayon::prelude::*; // Explicitly use rand crate trait

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

    // Initialize "Ghosts"
    let ghost_count = 100;
    let mut rng = ::rand::thread_rng();

    let mut ghosts: Vec<PendulumSystem> = (0..ghost_count)
        .map(|_| {
            let mut g = real_system.clone();
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
        .collect();

    let mut zoom = 1.0;
    let mut target = Vec2::new(600.0, 400.0);
    let mut show_ghosts = true;

    loop {
        if is_key_down(KeyCode::Right) {
            target.x += 10.0 / zoom;
        }
        if is_key_down(KeyCode::Left) {
            target.x -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::Down) {
            target.y += 10.0 / zoom;
        }
        if is_key_down(KeyCode::Up) {
            target.y -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::Equal) {
            zoom *= 1.05;
        }
        if is_key_down(KeyCode::Minus) {
            zoom *= 0.95;
        }
        if is_key_pressed(KeyCode::G) {
            show_ghosts = !show_ghosts;
        }

        // Mouse Drag
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
            let world_mouse = (mouse_vec - Vec2::new(screen_width() / 2.0, screen_height() / 2.0))
                / zoom
                + target;

            let mut closest_dist = 50.0 / zoom;
            let mut closest_idx = None;

            for (i, node) in real_system.nodes.iter().enumerate() {
                let dist = node.pos.distance(world_mouse);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_idx = Some(i);
                }
            }

            if let Some(idx) = closest_idx {
                real_system.nodes[idx].pos = world_mouse;
                real_system.nodes[idx].prev_pos = world_mouse;

                if is_key_down(KeyCode::Space) {
                    for g in &mut ghosts {
                        g.nodes[idx].pos = world_mouse;
                        g.nodes[idx].prev_pos = world_mouse;
                    }
                }
            }
        }

        let dt = get_frame_time();

        real_system.step(dt);
        ghosts.par_iter_mut().for_each(|g| g.step(dt));

        clear_background(BLACK);

        set_camera(&Camera2D {
            target,
            zoom: Vec2::new(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

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

        for link in &real_system.links {
            let pos_a = real_system.nodes[link.a].pos;
            let pos_b = real_system.nodes[link.b].pos;
            draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 2.0 / zoom, LIGHTGRAY);
        }

        for node in &real_system.nodes {
            let color = if node.fixed { RED } else { WHITE };
            let size = if node.mass > 2.0 { 5.0 } else { 2.0 };
            draw_circle(node.pos.x, node.pos.y, size / zoom, color);
        }

        set_default_camera();

        draw_text("CHAOS PENDULUM", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Nodes: {}", real_system.nodes.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Ghosts: {}", ghosts.len()),
            20.0,
            80.0,
            20.0,
            Color::new(0.0, 1.0, 1.0, 1.0),
        );
        draw_text(
            "Arrows: Move | +/-: Zoom | Mouse: Drag | G: Toggle Ghosts",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
