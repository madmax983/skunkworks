use ::rand::Rng;
use macroquad::prelude::*;

mod agent;
mod grid;

use agent::{Agent, AgentKind};
use grid::Grid;

const GRID_WIDTH: usize = 400;
const GRID_HEIGHT: usize = 300;
const SCALE: f32 = 2.0;

#[macroquad::main("Logistic Life")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut agents: Vec<Agent> = Vec::new();

    // Initialize random r and initial state
    let mut rng = ::rand::thread_rng();
    for i in 0..grid.cells.len() {
        grid.cells[i] = rng.gen::<f32>();
        // Initialize r near 3.5 (edge of chaos)
        grid.params_r[i] = 3.5 + rng.gen_range(-0.1..0.1);
    }

    // Spawn agents
    for _ in 0..1000 {
        let x = rng.gen_range(0.0..GRID_WIDTH as f32);
        let y = rng.gen_range(0.0..GRID_HEIGHT as f32);
        agents.push(Agent::new(x, y, AgentKind::Red));
    }
    for _ in 0..1000 {
        let x = rng.gen_range(0.0..GRID_WIDTH as f32);
        let y = rng.gen_range(0.0..GRID_HEIGHT as f32);
        agents.push(Agent::new(x, y, AgentKind::Blue));
    }

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut paused = false;
    let mut epsilon = 0.5; // Coupling strength
    let mut show_params = false;

    loop {
        // --- Input Handling ---
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset
            for i in 0..grid.cells.len() {
                grid.cells[i] = rng.gen::<f32>();
                grid.params_r[i] = 3.5 + rng.gen_range(-0.1..0.1);
            }
            agents.clear();
            for _ in 0..1000 {
                agents.push(Agent::new(
                    rng.gen_range(0.0..GRID_WIDTH as f32),
                    rng.gen_range(0.0..GRID_HEIGHT as f32),
                    AgentKind::Red,
                ));
                agents.push(Agent::new(
                    rng.gen_range(0.0..GRID_WIDTH as f32),
                    rng.gen_range(0.0..GRID_HEIGHT as f32),
                    AgentKind::Blue,
                ));
            }
        }
        if is_key_pressed(KeyCode::G) {
            show_params = !show_params;
        }

        if is_key_down(KeyCode::Up) {
            epsilon = (epsilon + 0.01_f32).min(1.0_f32);
        }
        if is_key_down(KeyCode::Down) {
            epsilon = (epsilon - 0.01_f32).max(0.0_f32);
        }

        // Mouse Painting
        if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            // Convert screen coords to grid coords
            // Assume grid is drawn at 0,0 with scale
            // Wait, we center the grid? Let's just draw at 0,0 for now.
            let gx = (mx / SCALE) as isize;
            let gy = (my / SCALE) as isize;

            let brush_size = 10;
            let strength = if is_mouse_button_down(MouseButton::Left) {
                0.05
            } else {
                -0.05
            }; // Left = Chaos (Red), Right = Order (Blue)

            for dy in -brush_size..=brush_size {
                for dx in -brush_size..=brush_size {
                    if dx * dx + dy * dy <= brush_size * brush_size {
                        let idx = grid.get_idx(gx + dx as isize, gy + dy as isize);
                        grid.params_r[idx] = (grid.params_r[idx] + strength).clamp(2.0, 4.0);
                    }
                }
            }
        }

        // --- Simulation ---
        if !paused {
            grid.update(epsilon);

            // Update agents
            // We iterate backwards to remove dead agents
            let mut i = 0;
            while i < agents.len() {
                agents[i].update(&mut grid);

                if agents[i].energy <= 0.0 {
                    agents.swap_remove(i);
                } else {
                    // Reproduction
                    if agents[i].energy > 1.5 && rng.gen_bool(0.01) {
                        agents[i].energy *= 0.5;
                        let child = Agent::new(agents[i].x, agents[i].y, agents[i].kind);
                        agents.push(child);
                        // Note: push adds to end, so we won't process child in this loop (if iterating by index),
                        // but `swap_remove` might mess up order.
                        // Actually `swap_remove` moves the last element to `i`.
                        // If we iterate `while i < len`, we re-process the swapped element. Correct.
                        // But `push` adds to end, which is `len`. `len` increases.
                        // So we WILL process the child immediately?
                        // If `i` is current index, `len` increases.
                        // Standard pattern is usually separate lists or `retain`.
                        // But let's keep it simple. If child is processed, fine.
                    }
                    i += 1;
                }
            }

            // Cap population
            if agents.len() > 10000 {
                agents.truncate(10000);
            }
        }

        // --- Rendering ---
        clear_background(BLACK);

        // Update texture
        for (i, pixel) in image.bytes.chunks_exact_mut(4).enumerate() {
            if i >= grid.cells.len() {
                break;
            }

            if show_params {
                // Visualize r
                let r = grid.params_r[i];
                // Map 2.0..4.0 to color
                // 2.0 = Blue, 3.0 = Green, 3.5 = Yellow, 4.0 = Red
                let val = (r - 2.0) / 2.0; // 0..1
                let color = if val < 0.5 {
                    // Blue to Green
                    let t = val * 2.0;
                    Color::new(0.0, t, 1.0 - t, 1.0)
                } else {
                    // Green to Red
                    let t = (val - 0.5) * 2.0;
                    Color::new(t, 1.0 - t, 0.0, 1.0)
                };
                pixel[0] = (color.r * 255.0) as u8;
                pixel[1] = (color.g * 255.0) as u8;
                pixel[2] = (color.b * 255.0) as u8;
                pixel[3] = 255;
            } else {
                // Visualize x
                let val = grid.cells[i]; // 0..1
                let c = (val * 255.0) as u8;
                pixel[0] = c;
                pixel[1] = c;
                pixel[2] = c;
                pixel[3] = 255;
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GRID_WIDTH as f32 * SCALE, GRID_HEIGHT as f32 * SCALE)),
                ..Default::default()
            },
        );

        // Draw Agents
        for agent in &agents {
            let color = match agent.kind {
                AgentKind::Red => RED,
                AgentKind::Blue => BLUE,
            };
            draw_rectangle(agent.x * SCALE, agent.y * SCALE, SCALE, SCALE, color);
        }

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Coupling (Up/Down): {:.2}", epsilon),
            10.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Left Click: Chaos (Red), Right Click: Order (Blue)",
            10.0,
            80.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Space: Pause, G: Toggle Heatmap, R: Reset",
            10.0,
            100.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
