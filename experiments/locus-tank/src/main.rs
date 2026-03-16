use crossbeam_channel::bounded;
use locus::{flocking::{compute_force, FlockingParams}, Vec2 as LocusVec2};
use macroquad::prelude::*;
use ::rand::Rng;
use resonance_audio::audio::{AudioCommand, AudioModel};

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const BOID_COUNT: usize = 200;

#[derive(Clone, Copy)]
pub struct Boid {
    pub pos: LocusVec2,
    pub vel: LocusVec2,
}

#[macroquad::main("Locus Tank")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    let mut rng = ::rand::thread_rng();
    let mut boids: Vec<Boid> = (0..BOID_COUNT)
        .map(|_| Boid {
            pos: LocusVec2::new(
                rng.gen_range(0.0..(GRID_WIDTH as f64)),
                rng.gen_range(0.0..(GRID_HEIGHT as f64)),
            ),
            vel: LocusVec2::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ).normalize() * 2.0,
        })
        .collect();

    let flocking_params = FlockingParams {
        view_radius: 15.0,
        separation_radius: 5.0,
        max_speed: 2.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    loop {
        // Step Simulation
        for _ in 0..4 {
            model.grid.step();
        }

        if let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        // Boid physics
        let positions: Vec<LocusVec2> = boids.iter().map(|b| b.pos).collect();
        let velocities: Vec<LocusVec2> = boids.iter().map(|b| b.vel).collect();

        for (i, boid) in boids.iter_mut().enumerate() {
            let force = compute_force(&positions, &velocities, i, &flocking_params);
            boid.vel += force;

            if boid.vel.magnitude() > flocking_params.max_speed {
                boid.vel = boid.vel.normalize() * flocking_params.max_speed;
            }

            boid.pos += boid.vel;

            // Wrap around
            if boid.pos.x < 0.0 {
                boid.pos.x = GRID_WIDTH as f64;
            } else if boid.pos.x >= GRID_WIDTH as f64 {
                boid.pos.x = 0.0;
            }

            if boid.pos.y < 0.0 {
                boid.pos.y = GRID_HEIGHT as f64;
            } else if boid.pos.y >= GRID_HEIGHT as f64 {
                boid.pos.y = 0.0;
            }

            // Interact with the ripple tank
            let gx = boid.pos.x.round() as usize;
            let gy = boid.pos.y.round() as usize;

            if gx < GRID_WIDTH && gy < GRID_HEIGHT {
                let _ = cmd_tx.try_send(AudioCommand::Pluck {
                    x: gx,
                    y: gy,
                    strength: boid.vel.magnitude() as f32 * 0.1,
                });
            }
        }

        // Render the tank
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = y * GRID_WIDTH + x;
                let val = current_snapshot[idx];

                let c = if val > 0.0 {
                    let v = (val * 255.0).clamp(0.0, 255.0) as u8;
                    Color::from_rgba(v, v, 255, 255)
                } else {
                    let v = (-val * 255.0).clamp(0.0, 255.0) as u8;
                    Color::from_rgba(0, 0, v, 255)
                };

                image.set_pixel(x as u32, y as u32, c);
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        let scale = f32::min(
            screen_width() / GRID_WIDTH as f32,
            screen_height() / GRID_HEIGHT as f32,
        );

        let offset_x = (screen_width() - GRID_WIDTH as f32 * scale) / 2.0;
        let offset_y = (screen_height() - GRID_HEIGHT as f32 * scale) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GRID_WIDTH as f32 * scale, GRID_HEIGHT as f32 * scale)),
                ..Default::default()
            },
        );

        // Draw Boids
        for boid in &boids {
            let px = offset_x + boid.pos.x as f32 * scale;
            let py = offset_y + boid.pos.y as f32 * scale;
            draw_circle(px, py, 2.0, YELLOW);
        }

        next_frame().await;
    }
}
