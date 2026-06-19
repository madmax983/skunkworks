use gray_scott::GrayScott;
use macroquad::prelude::*;
use rayon::prelude::*;

mod agent;
mod audio;
// mod grid; // Extracted to crate

use agent::{Agent, Settings};
use audio::Synthesizer;
// use grid::GrayScottGrid;

fn window_conf() -> Conf {
    Conf {
        window_title: "Myco-Diffusion".to_string(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Parameters
    let grid_w = 300;
    let grid_h = 300;
    let num_agents = 5000;

    // Simulation Objects
    let mut grid = GrayScott::new(grid_w, grid_h);

    // Customize Settings
    // GrayScott defaults are diff_u=0.16, diff_v=0.08 which matches myco-diffusion
    let settings = Settings {
        move_speed: 1.0,
        deposit_amount: 0.5, // Strong deposit to trigger reaction
        sensor_dist: 5.0,
        ..Default::default()
    };

    // Initialize Agents
    let mut agents = Vec::with_capacity(num_agents);
    let mut rng = ::rand::thread_rng();
    use ::rand::Rng;

    // Create a few "Cities" (Attractors) for agents to commute between
    let num_cities = 4;
    let mut cities = Vec::new();
    for _ in 0..num_cities {
        cities.push(vec2(
            rng.gen_range(50.0..(grid_w as f32 - 50.0)),
            rng.gen_range(50.0..(grid_h as f32 - 50.0)),
        ));
    }

    for _ in 0..num_agents {
        let home = cities[rng.gen_range(0..num_cities)];
        let work = cities[rng.gen_range(0..num_cities)];

        // Start near home with random offset
        let start_pos = home + vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        agents.push(Agent::new(start_pos, angle, home, work));
    }

    let mut synthesizer = Synthesizer::new();

    // Visualization
    let mut image = Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        // 1. Update Audio / Rhythm
        let dt = get_frame_time();
        synthesizer.update(dt);
        let (feed, kill) = synthesizer.get_params();

        // 2. Update Agents
        // Parallel update for movement/sensing
        // Use a read-only reference to grid for sensing
        agents.par_iter_mut().for_each(|agent| {
            agent.update(&grid, &settings);
        });

        // Serial deposit (avoid race conditions on grid.v)
        // Agents deposit 'V' (the active chemical)
        let w = grid.width();
        let h = grid.height();
        for agent in &agents {
            let x = agent.pos.x as usize;
            let y = agent.pos.y as usize;
            if x < w && y < h {
                grid.add_chemical(x, y, settings.deposit_amount);
            }
        }

        // 3. Update Grid (Reaction-Diffusion)
        // Run multiple steps for stability/speed ratio?
        // GS usually needs small dt (~1.0) but many iterations if real-time is slow.
        // Let's try 1 step per frame with dt=1.0 for visual effect.
        grid.update(feed, kill, 1.0);

        // 4. Render
        clear_background(BLACK);

        // Map Grid to Image
        // Use parallel iterator to generate pixels, then copy to image?
        // Image::set_pixel is not thread safe.
        // Let's create a buffer of colors then copy.
        let colors: Vec<Color> = (0..grid_w * grid_h)
            .into_par_iter()
            .map(|i| {
                let u = grid.u()[i];
                let v = grid.v()[i];

                // Visualization Scheme:
                // U is background (usually 1.0). V is the pattern (growing).
                // We want V to be glowing.

                // Palette:
                // V=0 -> Black/Dark Blue
                // V>0 -> Cyan/Purple/White

                let r = v * 3.0; // Red channel
                let g = v * 1.5 + u * 0.1; // Green channel
                let b = v * 4.0 + u * 0.2; // Blue channel

                Color::new(r.min(1.0), g.min(1.0), b.min(1.0), 1.0)
            })
            .collect();

        for (i, col) in colors.iter().enumerate() {
            let x = (i % grid_w) as u32;
            let y = (i / grid_w) as u32;
            image.set_pixel(x, y, *col);
        }

        texture.update(&image);

        // Draw Texture Scaled
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Agents (Optional, maybe too cluttered? Let's draw faint dots)
        // Scaling
        let scale_x = screen_width() / grid_w as f32;
        let scale_y = screen_height() / grid_h as f32;

        // Only draw a subset if too many? 5000 is okay for points.
        // Actually, the agents ARE the deposit, so seeing them is redundant if the trail is visible.
        // But let's draw them as tiny bright specks to show the "source".
        if is_key_down(KeyCode::A) {
            // Toggle with A?
            for agent in &agents {
                let sx = agent.pos.x * scale_x;
                let sy = agent.pos.y * scale_y;
                draw_rectangle(sx, sy, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.3));
            }
        }

        // Draw Cities
        for city in &cities {
            let sx = city.x * scale_x;
            let sy = city.y * scale_y;
            draw_circle_lines(sx, sy, 10.0, 2.0, YELLOW);
        }

        // UI
        draw_text("MYCO-DIFFUSION", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Feed: {:.4} Kill: {:.4}", feed, kill),
            20.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
