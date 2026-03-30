mod agent;
mod lattice;

use agent::Agent;
use lattice::Lattice;
use macroquad::prelude::*;

const GRID_W: usize = 400;
const GRID_H: usize = 300;
const NUM_AGENTS: usize = 100;

#[macroquad::main("Chimera Chaos")]
async fn main() {
    let mut lattice = Lattice::new(GRID_W, GRID_H);
    let mut agents: Vec<Agent> = Vec::new();

    // Spawn initial agents
    for i in 0..NUM_AGENTS {
        let x = rand::gen_range(0, GRID_W);
        let y = rand::gen_range(0, GRID_H);
        agents.push(Agent::new(i, x, y));
    }

    // Texture for the lattice
    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut frame_count = 0;
    let mut paused = false;

    loop {
        // --- Input ---
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            lattice = Lattice::new(GRID_W, GRID_H);
            agents.clear();
            for i in 0..NUM_AGENTS {
                let x = rand::gen_range(0, GRID_W);
                let y = rand::gen_range(0, GRID_H);
                agents.push(Agent::new(i, x, y));
            }
        }

        // Perturb
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let lattice_h = screen_h; // Full screen for now

            if my <= lattice_h {
                let gx = (mx / screen_w * GRID_W as f32) as usize;
                let gy = (my / lattice_h * GRID_H as f32) as usize;
                lattice.perturb(gx, gy, 10);
            }
        }

        // Spawn Agent
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let lattice_h = screen_h;

            if my <= lattice_h {
                let gx = (mx / screen_w * GRID_W as f32) as usize;
                let gy = (my / lattice_h * GRID_H as f32) as usize;
                agents.push(Agent::new(agents.len(), gx, gy));
            }
        }

        // --- Update ---
        if !paused {
            lattice.update();

            // Update Agents
            for agent in &mut agents {
                agent.update(&mut lattice);
            }

            // Remove dead agents
            agents.retain(|a| a.energy > 0.0);

            frame_count += 1;
        }

        // --- Render ---

        // 1. Update Texture from Lattice
        for (i, val) in lattice.cells.iter().enumerate() {
            let x = i % GRID_W;
            let y = i / GRID_W;
            let r_val = lattice.r_map[i];

            // Color mapping:
            // Value -> Brightness
            // R -> Hue/Tint
            // Low R (Stable) = Blue
            // High R (Chaos) = Red

            let r_norm = (r_val / 4.0).clamp(0.0, 1.0);
            let v_norm = val.clamp(0.0, 1.0);

            let color = Color::new(
                v_norm * r_norm,         // Red increases with Chaos & Value
                v_norm * 0.5,            // Green is constant-ish
                v_norm * (1.0 - r_norm), // Blue increases with Stability & Value
                1.0,
            );

            image.set_pixel(x as u32, y as u32, color);
        }
        texture.update(&image);

        clear_background(DARKGRAY);

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw Lattice
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                ..Default::default()
            },
        );

        // Draw Agents
        for agent in &agents {
            let screen_x = (agent.x as f32 / GRID_W as f32) * screen_w;
            let screen_y = (agent.y as f32 / GRID_H as f32) * screen_h;

            draw_circle(screen_x, screen_y, 3.0, Color::new(1.0, 1.0, 0.0, 0.8));
        }

        // --- UI ---
        draw_text("Chimera Chaos", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            60.0,
            20.0,
            YELLOW,
        );
        draw_text(
            "Left Click: Spawn Agent | Right Click: Perturb",
            10.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
