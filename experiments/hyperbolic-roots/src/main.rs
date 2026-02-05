mod geometry;
mod simulation;

use macroquad::prelude::*;
use num_complex::Complex;
use simulation::RootSystem;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyperbolic Roots".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut root_system = RootSystem::new();
    root_system.spawn_nutrients(300);

    let mut show_nutrients = true;

    loop {
        // --- Update ---

        if is_key_pressed(KeyCode::R) {
            root_system = RootSystem::new();
            root_system.spawn_nutrients(300);
        }

        if is_key_pressed(KeyCode::Space) {
            show_nutrients = !show_nutrients;
        }

        // Add nutrient on click
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if let Some(pos) = screen_to_disk(mx, my) {
                // Add a cluster
                for _ in 0..3 {
                    let offset = Complex::from_polar(rand::gen_range(0.0, 0.05), rand::gen_range(0.0, 6.28));
                    let p = pos + offset;
                    // Ensure inside disk
                    if p.norm() < 0.99 {
                        root_system.add_nutrient(p);
                    }
                }
            }
        }

        root_system.grow_step();

        // --- Draw ---
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark Blue/Black space

        let (cx, cy, scale) = get_disk_params();

        // Draw Boundary
        draw_circle_lines(cx, cy, scale, 2.0, WHITE);

        // Draw Nutrients
        if show_nutrients {
            for nutrient in &root_system.nutrients {
                let (nx, ny) = disk_to_screen(nutrient.pos, cx, cy, scale);
                draw_circle(nx, ny, 2.0, Color::new(1.0, 0.8, 0.2, 0.6));
            }
        }

        // Draw Roots
        for node in &root_system.nodes {
            if let Some(parent_idx) = node.parent {
                let parent = &root_system.nodes[parent_idx];

                let (x1, y1) = disk_to_screen(parent.pos, cx, cy, scale);
                let (x2, y2) = disk_to_screen(node.pos, cx, cy, scale);

                // For small segments, straight line is fine.
                // Color gradient based on distance from center?
                let dist = node.pos.norm();
                let hue = (dist * 0.5 + 0.3) % 1.0; // Shift color as it grows out
                let color = hsl_to_rgb(hue as f32, 0.8, 0.6);

                draw_line(x1, y1, x2, y2, node.thickness, color);
            }
        }

        // UI
        draw_text("Hyperbolic Roots", 20.0, 30.0, 30.0, WHITE);
        draw_text("Space: Toggle Nutrients | R: Reset | Click: Feed", 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Nodes: {}", root_system.nodes.len()), 20.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}

fn get_disk_params() -> (f32, f32, f32) {
    let w = screen_width();
    let h = screen_height();
    let min_dim = w.min(h);
    let scale = min_dim / 2.0 * 0.9;
    (w / 2.0, h / 2.0, scale)
}

fn disk_to_screen(p: Complex<f64>, cx: f32, cy: f32, scale: f32) -> (f32, f32) {
    (
        cx + p.re as f32 * scale,
        cy - p.im as f32 * scale // Flip Y for screen coords
    )
}

fn screen_to_disk(x: f32, y: f32) -> Option<Complex<f64>> {
    let (cx, cy, scale) = get_disk_params();
    let dx = (x - cx) / scale;
    let dy = (cy - y) / scale; // Flip Y back
    let p = Complex::new(dx as f64, dy as f64);
    if p.norm() < 1.0 {
        Some(p)
    } else {
        None
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}
