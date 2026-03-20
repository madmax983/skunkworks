//! 🧬 Splice: Cross `chaos-pendulum` × `ferrous-fluid`
//!
//! This hybrid integrates the chaotic macroscopic motions of `chaos-pendulum`
//! as dynamic magnetic nodes acting upon the microscopic fluid particle
//! simulation from `ferrous-fluid`.

use macroquad::prelude::*;
use ::rand::Rng;

mod physics;
use physics::{PendulumSystem, Universe};

// Lineage (chaos-pendulum): The original chaotic double pendulum setup.
fn init_chaos_pendulum() -> PendulumSystem {
    let mut sys = PendulumSystem::new();

    let root = sys.add_node(
        locus::Vec2::new(500.0, 500.0),
        10.0,
        true,
        "root".to_string(),
    );
    let p1 = sys.add_node(
        locus::Vec2::new(500.0, 400.0),
        2.0,
        false,
        "p1".to_string(),
    );
    let p2 = sys.add_node(
        locus::Vec2::new(500.0, 300.0),
        1.5,
        false,
        "p2".to_string(),
    );
    let p3 = sys.add_node(
        locus::Vec2::new(500.0, 200.0),
        1.0,
        false,
        "p3".to_string(),
    );

    sys.add_link(root, p1, 100.0);
    sys.add_link(p1, p2, 100.0);
    sys.add_link(p2, p3, 100.0);

    sys
}

#[macroquad::main("Chaos Fluid")]
async fn main() {
    let mut pendulum = init_chaos_pendulum();
    let mut universe = Universe::new(1000.0, 1000.0);

    let mut zoom = 1.0;
    let mut target = vec2(500.0, 500.0);

    loop {
        let mut dt = get_frame_time();
        if dt > 0.1 {
            dt = 0.1;
        }

        if is_key_down(KeyCode::D) {
            target.x += 10.0 / zoom;
        }
        if is_key_down(KeyCode::A) {
            target.x -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::S) {
            target.y -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::W) {
            target.y += 10.0 / zoom;
        }
        if is_key_down(KeyCode::Equal) {
            zoom *= 1.05;
        }
        if is_key_down(KeyCode::Minus) {
            zoom *= 0.95;
        }

        if is_key_pressed(KeyCode::R) {
            pendulum = init_chaos_pendulum();
            universe = Universe::new(1000.0, 1000.0);
        }

        if is_key_pressed(KeyCode::K) {
            let mut rng = ::rand::thread_rng();
            for node in &mut pendulum.nodes {
                if !node.fixed {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let force = locus::Vec2::new(angle.cos(), angle.sin()) * 50.0;
                    node.prev_pos -= force * dt as f64;
                }
            }
        }

        let mouse_pos = mouse_position();
        // Since the camera is set with an inverted Y axis (zoom: Vec2::new(..., -zoom / ...)),
        // we invert the screen Y when transforming to world coordinates.
        let mouse_vec = vec2(mouse_pos.0, -mouse_pos.1);
        let world_mouse =
            (mouse_vec - vec2(screen_width() / 2.0, -screen_height() / 2.0)) / zoom + target;

        let mut closest_dist = 20.0 / zoom;
        let mut hovered_idx = None;

        for (i, node) in pendulum.nodes.iter().enumerate() {
            let n_pos = vec2(node.pos.x as f32, node.pos.y as f32);
            let dist = n_pos.distance(world_mouse);
            if dist < closest_dist {
                closest_dist = dist;
                hovered_idx = Some(i);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(idx) = hovered_idx {
                pendulum.nodes[idx].pos = locus::Vec2::new(world_mouse.x as f64, world_mouse.y as f64);
                pendulum.nodes[idx].prev_pos =
                    locus::Vec2::new(world_mouse.x as f64, world_mouse.y as f64);
            }
        }

        pendulum.step(dt as f64);
        universe.update(0.05, &pendulum);

        clear_background(BLACK);

        set_camera(&Camera2D {
            target,
            zoom: vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw Fluid Particles
        for p in &universe.particles {
            draw_circle(p.pos.x as f32, p.pos.y as f32, 2.0 / zoom, Color::new(0.0, 1.0, 1.0, 1.0));
        }

        // Draw Pendulum Links
        for link in &pendulum.links {
            let pos_a = pendulum.nodes[link.a].pos;
            let pos_b = pendulum.nodes[link.b].pos;
            draw_line(
                pos_a.x as f32,
                pos_a.y as f32,
                pos_b.x as f32,
                pos_b.y as f32,
                2.0 / zoom,
                LIGHTGRAY,
            );
        }

        // Draw Pendulum Nodes
        for (i, node) in pendulum.nodes.iter().enumerate() {
            let color = if node.fixed {
                RED
            } else if Some(i) == hovered_idx {
                YELLOW
            } else {
                BLUE
            };

            let size = (node.mass * 2.0).clamp(3.0, 15.0) as f32;
            draw_circle(node.pos.x as f32, node.pos.y as f32, size / zoom, color);
        }

        set_default_camera();

        draw_text("CHAOS FLUID", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Particles: {}", universe.particles.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "WASD: Cam | K: Kick Pendulum | R: Reset",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
