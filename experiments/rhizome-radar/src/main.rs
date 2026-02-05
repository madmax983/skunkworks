mod simulation;

use macroquad::prelude::*;
use simulation::{Nutrient, RootSystem};

// Explicitly use the rand crate to avoid conflict with macroquad::prelude::rand
use ::rand::Rng;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rhizome Radar".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initial Setup
    let mut root_system = create_simulation(screen_width(), screen_height());
    let mut show_radar = true;

    loop {
        // Update
        if is_key_pressed(KeyCode::R) {
            root_system = create_simulation(screen_width(), screen_height());
        }
        if is_key_pressed(KeyCode::Space) {
            show_radar = !show_radar;
        }

        // Mouse interaction: Add nutrient
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Add a cluster of nutrients
            let mut rng = ::rand::thread_rng();
            for _ in 0..5 {
                root_system.nutrients.push(Nutrient {
                    pos: Vec2::new(
                        mx + rng.gen_range(-10.0..10.0),
                        my + rng.gen_range(-10.0..10.0),
                    ),
                    strength: 1.0,
                });
            }
        }

        // Run multiple simulation steps per frame for speed? Or just one to watch it grow?
        root_system.grow_step();

        // Draw
        clear_background(Color::new(0.05, 0.02, 0.0, 1.0)); // Darker soil color

        // Draw Sensing Lines (The Radar)
        // We do this BEFORE roots so roots are on top
        if show_radar {
            for nutrient in &root_system.nutrients {
                // Find closest node (naive O(N*M), optimize if slow)
                // Only draw if within detection radius * 1.5 (visual range)
                let mut closest_dist = root_system.detection_radius;
                let mut closest_pos = None;

                for node in &root_system.nodes {
                    let dist = node.pos.distance(nutrient.pos);
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_pos = Some(node.pos);
                    }
                }

                if let Some(pos) = closest_pos {
                    // Draw faint line
                    draw_line(
                        nutrient.pos.x,
                        nutrient.pos.y,
                        pos.x,
                        pos.y,
                        0.5,
                        Color::new(0.3, 0.3, 0.3, 0.5),
                    );
                }
            }
        }

        // Draw Nutrients (The Radar Targets)
        for nutrient in &root_system.nutrients {
            // Pulse size based on time?
            draw_circle(
                nutrient.pos.x,
                nutrient.pos.y,
                1.5 * nutrient.strength,
                Color::new(1.0, 0.8, 0.2, 0.8),
            );
        }

        // Draw Roots (The Path)
        for node in &root_system.nodes {
            if let Some(parent_idx) = node.parent {
                let parent = &root_system.nodes[parent_idx];
                draw_line(
                    parent.pos.x,
                    parent.pos.y,
                    node.pos.x,
                    node.pos.y,
                    2.0 * node.thickness,
                    Color::new(0.8, 0.9, 0.7, 1.0), // Pale organic color
                );
            }
        }

        draw_text("Rhizome Radar", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "R: Reset | Space: Toggle Radar | Click: Feed",
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Roots: {}", root_system.nodes.len()),
            20.0,
            90.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Nutrients: {}", root_system.nutrients.len()),
            20.0,
            110.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}

fn create_simulation(width: f32, height: f32) -> RootSystem {
    let mut rng = ::rand::thread_rng();
    let mut nutrients = Vec::new();

    // Create a circular distribution of nutrients
    for _ in 0..1500 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        // Distribute in a ring
        let dist = rng.gen_range(50.0..width.min(height) / 2.0 - 20.0);
        let pos = Vec2::new(
            width / 2.0 + angle.cos() * dist,
            height / 2.0 + angle.sin() * dist,
        );
        nutrients.push(Nutrient { pos, strength: 1.0 });
    }

    RootSystem::new(Vec2::new(width / 2.0, height / 2.0), nutrients)
}
