use macroquad::prelude::*;
use std::collections::VecDeque;

mod lorenz;
mod life;

use lorenz::LorenzState;
use life::LifeGrid;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;
const TRAIL_LENGTH: usize = 5000;

#[macroquad::main("Chaos Life")]
async fn main() {
    let mut lorenz = LorenzState::new();
    let mut life = LifeGrid::new(GRID_WIDTH, GRID_HEIGHT);

    // Trail of the attractor
    let mut trail: VecDeque<(f32, f32, f32)> = VecDeque::new();

    // Image for rendering the grid
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let mut paused = false;
    let mut speed = 1;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            life.reset();
            lorenz = LorenzState::new();
            trail.clear();
        }
        if is_key_pressed(KeyCode::Up) {
            speed += 1;
        }
        if is_key_pressed(KeyCode::Down) {
            if speed > 1 { speed -= 1; }
        }

        if !paused {
            for _ in 0..speed {
                // Update Lorenz
                // Use smaller dt for smoother trail, but we need to match Life speed.
                // Life updates are discrete steps. Lorenz is continuous.
                // We'll update Lorenz multiple times per Life step or just once with larger dt?
                // Standard Lorenz dt is 0.01.
                lorenz.update(0.01);

                // Get Rules
                let rules = lorenz.get_rules();

                // Update Life
                life.update(rules);

                // Feedback: Density affects rho
                let density = life.get_density();
                // Target rho based on density.
                // Normal density for GoL is around 0.03 - 0.1?
                // Let's say target density is 0.1.
                // If density > 0.1, increase rho (more chaos).
                // If density < 0.1, decrease rho (less chaos).
                // Lorenz rho default is 28.
                // We want rho to vary between, say, 10 (stable) and 60 (chaotic).

                let target_rho = 10.0 + (density * 200.0).clamp(0.0, 80.0);
                // Smooth transition
                lorenz.rho = lorenz.rho * 0.95 + target_rho * 0.05;

                // Record trail
                trail.push_back((lorenz.x, lorenz.y, lorenz.z));
                if trail.len() > TRAIL_LENGTH {
                    trail.pop_front();
                }
            }
        }

        // Render
        clear_background(BLACK);

        // 1. Render Life Grid (Left Half)
        // Update image pixel by pixel
        // This is slow if we do it pixel by pixel on CPU?
        // 200x200 = 40k pixels. It's fine.
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cell = life.cells[y * GRID_WIDTH + x];
                let color = if cell == 1 {
                    // Color based on Lorenz state (e.g. z)
                    let val = (lorenz.z / 50.0).clamp(0.0, 1.0);
                    Color::new(val, 0.5 * val, 1.0 - val, 1.0)
                } else {
                    BLACK
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        let screen_w = screen_width();
        let screen_h = screen_height();

        let split_x = screen_w * 0.5;

        // Draw texture scaled to left half
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(split_x, screen_h)),
                ..Default::default()
            },
        );

        // 2. Render Lorenz Attractor (Right Half)
        // We project X-Z plane.
        // X range: [-20, 20], Z range: [0, 50]
        // Map to Right Half.

        let plot_w = screen_w - split_x;
        let plot_h = screen_h;
        let plot_x = split_x;
        let plot_y = 0.0;

        // Background for plot
        draw_rectangle(plot_x, plot_y, plot_w, plot_h, Color::new(0.05, 0.05, 0.05, 1.0));

        // Draw axes
        // ...

        // Draw trail
        if trail.len() > 1 {
            for i in 0..trail.len() - 1 {
                let (x1, _y1, z1) = trail[i];
                let (x2, _y2, z2) = trail[i+1];

                // Map (x, z) to screen
                // x: [-25, 25] -> [plot_x, plot_x + plot_w]
                // z: [0, 60] -> [plot_y + plot_h, plot_y] (invert Y)

                let sx1 = map_range(x1, -30.0, 30.0, plot_x, plot_x + plot_w);
                let sy1 = map_range(z1, 0.0, 60.0, plot_y + plot_h - 20.0, plot_y + 20.0);

                let sx2 = map_range(x2, -30.0, 30.0, plot_x, plot_x + plot_w);
                let sy2 = map_range(z2, 0.0, 60.0, plot_y + plot_h - 20.0, plot_y + 20.0);

                // Color based on velocity or index (fade out)
                let alpha = (i as f32 / trail.len() as f32).powf(2.0);
                let color = Color::new(0.0, 1.0, 0.8, alpha);

                draw_line(sx1, sy1, sx2, sy2, 1.5, color);
            }
        }

        // 3. Render Stats/UI
        let rules = lorenz.get_rules();
        let density = life.get_density();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Rho: {:.2}", lorenz.rho), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Density: {:.4}", density), 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Rules: S[{}-{}] B[{}]", rules.0, rules.1, rules.2), 10.0, 80.0, 20.0, GREEN);
        draw_text(&format!("Speed: {}x", speed), 10.0, 100.0, 20.0, YELLOW);

        // Draw dot for current state on plot
        let (cx, _cy, cz) = (lorenz.x, lorenz.y, lorenz.z);
        let csx = map_range(cx, -30.0, 30.0, plot_x, plot_x + plot_w);
        let csy = map_range(cz, 0.0, 60.0, plot_y + plot_h - 20.0, plot_y + 20.0);
        draw_circle(csx, csy, 5.0, RED);

        next_frame().await;
    }
}

fn map_range(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let t = (val - in_min) / (in_max - in_min);
    out_min + t * (out_max - out_min)
}
