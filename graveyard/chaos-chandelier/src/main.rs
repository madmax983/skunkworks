use macroquad::prelude::*;
use rayon::prelude::*;

mod loader;
mod physics;

use physics::DoublePendulum;

const SUB_STEPS: usize = 10;
const TRAIL_LENGTH: usize = 200;

#[macroquad::main("Chaos Chandelier")]
async fn main() {
    // 1. Load Dependency Chains
    let chains = match loader::load_dependency_chains() {
        Ok(c) => {
            println!("Loaded {} chains", c.len());
            c
        }
        Err(e) => {
            eprintln!("Failed to load chains: {}", e);
            Vec::new()
        }
    };

    if chains.is_empty() {
        // Fallback demo
        println!("No chains found (or error). Using demo mode.");
    }

    // 2. Initialize Pendulums
    // We create a pendulum for each chain.
    // We scale masses and lengths for visualization.
    let mut pendulums: Vec<DoublePendulum> = chains
        .iter()
        .map(|config| {
            // Scale values
            // Masses: 1.0 to 10.0
            let m1 = (config.m1 / 5.0).clamp(1.0, 10.0);
            let m2 = (config.m2 / 5.0).clamp(1.0, 10.0);

            // Lengths: 50.0 to 150.0 pixels
            // We want lengths to be somewhat uniform but varied.
            let l1 = 100.0 * config.l1 + (config.m1 * 2.0).clamp(0.0, 50.0);
            let l2 = 100.0 * config.l2 + (config.m2 * 2.0).clamp(0.0, 50.0);

            // Initial state: Start near upright or hanging down?
            // Hanging down is stable. Upright is unstable.
            // Let's start them hanging down but with high energy (kicked).
            // Or start them near horizontal.
            // Chaos is best seen when energy is high.
            // Let's start them all at (PI/2, PI/2) + noise.

            let theta1 = std::f32::consts::PI / 2.0 + rand::gen_range(-0.1, 0.1);
            let theta2 = std::f32::consts::PI / 2.0 + rand::gen_range(-0.1, 0.1);

            DoublePendulum::new(m1, l1, m2, l2, theta1, theta2)
        })
        .collect();

    // If empty, add some demo ones
    if pendulums.is_empty() {
        for i in 0..100 {
            let m1 = rand::gen_range(1.0, 5.0);
            let m2 = rand::gen_range(1.0, 5.0);
            let l1 = rand::gen_range(80.0, 120.0);
            let l2 = rand::gen_range(80.0, 120.0);
            let t1 = std::f32::consts::PI / 2.0 + (i as f32 * 0.001);
            let t2 = std::f32::consts::PI / 2.0 + (i as f32 * 0.001);
            pendulums.push(DoublePendulum::new(m1, l1, m2, l2, t1, t2));
        }
    }

    // Colors: Assign a color to each pendulum based on something
    // We can use a hash of the index or just random.
    let colors: Vec<Color> = (0..pendulums.len())
        .map(|i| macroquad::color::hsl_to_rgb((i as f32 * 0.137) % 1.0, 0.8, 0.5))
        .collect();

    // Trails Texture
    let width = screen_width() as u32;
    let height = screen_height() as u32;
    let trails_texture = render_target(width, height);
    trails_texture.texture.set_filter(FilterMode::Linear);

    let mut camera_pos = Vec2::new(0.0, 200.0);
    let mut zoom = 0.5;

    let mut show_arms = true;
    let mut paused = false;
    let physics_speed = 1.0;

    loop {
        // --- Update ---
        let dt = get_frame_time(); // Time since last frame
                                   // Physics update
                                   // We use a fixed physics timestep for stability, but for visual chaos we can just use real time.
                                   // However, RK4 is sensitive to dt. Let's clamp it.
        let physics_dt = (dt * physics_speed).min(0.05) / SUB_STEPS as f32;

        if !paused {
            // Rayon parallel update
            pendulums.par_iter_mut().for_each(|p| {
                for _ in 0..SUB_STEPS {
                    p.step(physics_dt);
                }
            });
        }

        // --- Render Trails to Texture ---
        // We want to fade the trails.
        // Option 1: Draw a semi-transparent black quad over the texture.
        // Option 2: Just draw new lines.
        // Macroquad `render_target` clears every frame if we use `set_camera` with it?
        // No, we can draw cumulatively if we don't clear.

        // Actually, managing persistent trails in Macroquad with a fade effect requires ping-pong buffers
        // or drawing a full-screen quad with alpha.
        // For simplicity, let's just keep a `VecDeque` of points for each pendulum and draw them.
        // Drawing thousands of trails might be heavy.
        // Let's stick to drawing "Instant" trails (last N points) to screen,
        // AND drawing "Permanent" trails to a texture that fades.

        // Let's implement the "Fade" effect:
        // Draw the texture onto itself with alpha?
        // Actually, let's just use the `trail` field in `DoublePendulum` for now.
        // RK4 step doesn't update `trail` yet. We should add that.
        // Wait, `DoublePendulum` has `trail: Vec<Vec2>`. We need to push to it.

        // Let's update trails in the main thread (or parallel if safe)
        if !paused {
            for p in &mut pendulums {
                let pos2 = p.get_pos2();
                p.trail.push(pos2);
                if p.trail.len() > TRAIL_LENGTH {
                    p.trail.remove(0);
                }
            }
        }

        // --- Input ---
        if is_key_down(KeyCode::Right) {
            camera_pos.x -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::Left) {
            camera_pos.x += 10.0 / zoom;
        }
        if is_key_down(KeyCode::Up) {
            camera_pos.y += 10.0 / zoom;
        }
        if is_key_down(KeyCode::Down) {
            camera_pos.y -= 10.0 / zoom;
        }
        if is_key_down(KeyCode::Equal) {
            zoom *= 1.05;
        }
        if is_key_down(KeyCode::Minus) {
            zoom *= 0.95;
        }

        if is_key_pressed(KeyCode::Space) {
            // Kick
            for p in &mut pendulums {
                p.omega1 += rand::gen_range(-2.0, 2.0);
                p.omega2 += rand::gen_range(-2.0, 2.0);
            }
        }
        if is_key_pressed(KeyCode::S) {
            show_arms = !show_arms;
        }
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset
            // We'd need to store initial state or just re-init.
            // For now, let's just randomize angles.
            for p in &mut pendulums {
                p.theta1 = std::f32::consts::PI / 2.0 + rand::gen_range(-0.1, 0.1);
                p.theta2 = std::f32::consts::PI / 2.0 + rand::gen_range(-0.1, 0.1);
                p.omega1 = 0.0;
                p.omega2 = 0.0;
                p.trail.clear();
            }
        }

        // --- Draw ---
        clear_background(BLACK);

        set_camera(&Camera2D {
            target: camera_pos,
            zoom: Vec2::new(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw Center
        draw_circle(0.0, 0.0, 5.0 / zoom, WHITE);

        // Draw Pendulums
        for (i, p) in pendulums.iter().enumerate() {
            let color = colors[i % colors.len()];
            let pos0 = Vec2::ZERO;
            let pos1 = p.get_pos1();
            let pos2 = p.get_pos2();

            // Draw Arms
            if show_arms {
                let alpha_color = Color::new(color.r, color.g, color.b, 0.15); // Faint arms
                draw_line(pos0.x, pos0.y, pos1.x, pos1.y, 2.0 / zoom, alpha_color);
                draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0 / zoom, alpha_color);
                draw_circle(pos1.x, pos1.y, 3.0 / zoom, alpha_color);
                draw_circle(pos2.x, pos2.y, 3.0 / zoom, alpha_color);
            }

            // Draw Trail
            if p.trail.len() > 1 {
                for j in 0..p.trail.len() - 1 {
                    let p1 = p.trail[j];
                    let p2 = p.trail[j + 1];
                    // Fade out
                    let fade = j as f32 / p.trail.len() as f32;
                    let trail_color = Color::new(color.r, color.g, color.b, fade * 0.8);
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0 / zoom, trail_color);
                }
            }
        }

        set_default_camera();

        // UI
        draw_text("Chaos Chandelier", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Pendulums: {}", pendulums.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Controls: Arrows=Cam, +/-=Zoom, Space=Kick, S=Toggle Arms, P=Pause, R=Reset",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
