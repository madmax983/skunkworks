use macroquad::prelude::*;
use ::rand::Rng;
use rayon::prelude::*;
use locus::flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocVec2;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const NUM_BOIDS: usize = 500;
const DAMPING: f32 = 0.99;

#[derive(Clone, Copy)]
struct BoidAgent {
    position: LocVec2,
    velocity: LocVec2,
}

impl BoidAgent {
    fn new(x: f64, y: f64) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
        let speed: f64 = rng.gen_range(20.0..50.0);

        let vel_x = angle.cos() * speed;
        let vel_y = angle.sin() * speed;

        Self {
            position: LocVec2::new(x, y),
            velocity: LocVec2::new(vel_x, vel_y),
        }
    }
}

/// 🧬 Splice Lineage
/// - From `crates/locus`: The `Boid` flocking logic (Separation, Alignment, Cohesion).
/// - From `experiments/ripple-tank`: The 2D physical acoustic wave tank simulation.
/// - Novel trait: "Acoustic Swarming". Boids deposit kinetic energy into the wave tank, generating pressure waves.
///   These ripples actively advect and push other boids around the environment.
fn world_to_grid(wx: f32, wy: f32, w: f32, h: f32) -> (f32, f32) {
    let scale_x = w / GRID_WIDTH as f32;
    let scale_y = h / GRID_HEIGHT as f32;
    (wx / scale_x, wy / scale_y)
}

#[macroquad::main("Luminous Tank")]
async fn main() {
    let mut current_wave: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut previous_wave: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut next_wave: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    let mut grid_forces: Vec<LocVec2> = vec![LocVec2::new(0.0, 0.0); NUM_BOIDS];
    let mut positions: Vec<LocVec2> = vec![LocVec2::new(0.0, 0.0); NUM_BOIDS];
    let mut velocities: Vec<LocVec2> = vec![LocVec2::new(0.0, 0.0); NUM_BOIDS];

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let w = screen_width();
    let h = screen_height();

    let mut agents: Vec<BoidAgent> = (0..NUM_BOIDS).map(|_| {
        let mut rng = ::rand::thread_rng();
        BoidAgent::new(rng.gen_range(0.0..w as f64), rng.gen_range(0.0..h as f64))
    }).collect();

    loop {
        let w = screen_width();
        let h = screen_height();
        let dt = get_frame_time() as f64;

        // Boid parameters
        let params = FlockingParams {
            view_radius: 50.0,
            separation_radius: 20.0,
            max_speed: 150.0,
            max_force: 50.0,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        // Update wave tank
        next_wave.copy_from_slice(&current_wave);
        for y in 1..GRID_HEIGHT - 1 {
            for x in 1..GRID_WIDTH - 1 {
                let idx = y * GRID_WIDTH + x;

                let val = (current_wave[idx - 1]
                         + current_wave[idx + 1]
                         + current_wave[idx - GRID_WIDTH]
                         + current_wave[idx + GRID_WIDTH]) / 2.0
                        - previous_wave[idx];

                next_wave[idx] = val * DAMPING;
            }
        }
        std::mem::swap(&mut previous_wave, &mut current_wave);
        std::mem::swap(&mut current_wave, &mut next_wave);

        // Boids sense the wave tank gradient
        // And they also drop waves
        grid_forces.fill(LocVec2::new(0.0, 0.0));

        for (i, agent) in agents.iter().enumerate() {
            let (gx, gy) = world_to_grid(agent.position.x as f32, agent.position.y as f32, w, h);
            let igx = gx as usize;
            let igy = gy as usize;

            if igx > 0 && igx < GRID_WIDTH - 1 && igy > 0 && igy < GRID_HEIGHT - 1 {
                // Deposit wave (acoustic pluck) based on speed
                let idx = igy * GRID_WIDTH + igx;
                current_wave[idx] += (agent.velocity.magnitude() * 0.05) as f32;

                // Read gradient to compute force
                let dx = current_wave[idx + 1] - current_wave[idx - 1];
                let dy = current_wave[idx + GRID_WIDTH] - current_wave[idx - GRID_WIDTH];

                // Boids are repelled by high pressure (crests) and pushed into troughs
                // Or maybe they surf the waves? Let's make them surf down the gradient.
                let wave_force = LocVec2::new(-dx as f64, -dy as f64) * 200.0;
                grid_forces[i] = wave_force;
            }
        }

        // Standard Boids Update
        for (i, agent) in agents.iter().enumerate() {
            positions[i] = agent.position;
            velocities[i] = agent.velocity;
        }

        // Fetch mouse pos outside par_iter_mut
        let (mx, my) = mouse_position();
        let mouse_pos = LocVec2::new(mx as f64, my as f64);

        agents.par_iter_mut().enumerate().for_each(|(i, agent)| {
            let mut acc = LocVec2::new(0.0, 0.0);

            // Flocking forces
            let flock_force = compute_force(&positions, &velocities, i, &params);
            acc += flock_force;

            // Wave pressure force
            acc += grid_forces[i];

            // Mouse repeller (optional interaction)
            let dist_to_mouse = agent.position.distance(mouse_pos);
            if dist_to_mouse < 100.0 {
                let repel = (agent.position - mouse_pos).normalize() * 500.0;
                acc += repel;
            }

            agent.velocity += acc * dt;

            // Limit speed
            let speed = agent.velocity.magnitude();
            if speed > params.max_speed {
                agent.velocity = agent.velocity.normalize() * params.max_speed;
            } else if speed < 20.0 {
                agent.velocity = agent.velocity.normalize() * 20.0;
            }

            agent.position += agent.velocity * dt;

            // Wrap around screen
            if agent.position.x < 0.0 { agent.position.x += w as f64; }
            if agent.position.x >= w as f64 { agent.position.x -= w as f64; }
            if agent.position.y < 0.0 { agent.position.y += h as f64; }
            if agent.position.y >= h as f64 { agent.position.y -= h as f64; }
        });

        // Rendering wave tank
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let val = current_wave[y * GRID_WIDTH + x];

                // Color mapping:
                // positive (crest) -> Cyan
                // negative (trough) -> Blue
                let r = 0;
                let mut g = 0;
                let b;

                if val > 0.0 {
                    let intensity = (val * 255.0).clamp(0.0, 255.0) as u8;
                    g = intensity / 2;
                    b = intensity;
                } else {
                    let intensity = (-val * 255.0).clamp(0.0, 255.0) as u8;
                    b = intensity / 2;
                }

                image.set_pixel(x as u32, y as u32, Color::from_rgba(r, g, b, 255));
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        // Draw the wave tank covering the background
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        // Draw Boids
        for agent in &agents {
            let x = agent.position.x as f32;
            let y = agent.position.y as f32;
            let angle = agent.velocity.y.atan2(agent.velocity.x) as f32;

            let p1 = vec2(x + angle.cos() * 6.0, y + angle.sin() * 6.0);
            let p2 = vec2(x + (angle + std::f32::consts::PI * 0.8).cos() * 4.0, y + (angle + std::f32::consts::PI * 0.8).sin() * 4.0);
            let p3 = vec2(x + (angle - std::f32::consts::PI * 0.8).cos() * 4.0, y + (angle - std::f32::consts::PI * 0.8).sin() * 4.0);

            draw_triangle(p1, p2, p3, YELLOW);
        }

        draw_text("LUMINOUS TANK - Acoustic Swarming", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Boids: {}", NUM_BOIDS), 10.0, 60.0, 20.0, LIGHTGRAY);
        draw_text("Hover mouse to agitate", 10.0, 80.0, 20.0, GRAY);

        next_frame().await
    }
}
