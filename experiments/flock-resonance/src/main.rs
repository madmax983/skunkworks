use flocking::{compute_force, FlockingParams};
use locus::Vec2 as FlockingVec2;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
use resonance_audio::physics::{Material, PhysicsGrid};

// Constants
const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 120;
const FLOCK_SIZE: usize = 150;

#[macroquad::main("Flock Resonance")]
async fn main() {
    let mut grid = PhysicsGrid::new(GRID_WIDTH, GRID_HEIGHT);

    // Add boundaries (walls)
    for x in 0..GRID_WIDTH {
        grid.set_material(x, 0, Material::Wall);
        grid.set_material(x, GRID_HEIGHT - 1, Material::Wall);
    }
    for y in 0..GRID_HEIGHT {
        grid.set_material(0, y, Material::Wall);
        grid.set_material(GRID_WIDTH - 1, y, Material::Wall);
    }

    // Initialize flock
    let mut positions = Vec::with_capacity(FLOCK_SIZE);
    let mut velocities = Vec::with_capacity(FLOCK_SIZE);
    let mut rng = thread_rng();

    for _ in 0..FLOCK_SIZE {
        positions.push(FlockingVec2::new(
            rng.gen_range(10.0..(GRID_WIDTH as f64 - 10.0)),
            rng.gen_range(10.0..(GRID_HEIGHT as f64 - 10.0)),
        ));
        velocities.push(FlockingVec2::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ));
    }

    let flock_params = FlockingParams {
        view_radius: 15.0,
        separation_radius: 3.0,
        max_speed: 1.5,
        max_force: 0.05,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    let cell_size = 6.0;

    // For rendering the acoustic wave
    let mut image = Image::gen_image_color(
        GRID_WIDTH as u16,
        GRID_HEIGHT as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);

    loop {
        // --- 1. Step the flocking simulation ---
        let mut new_velocities = velocities.clone();
        for i in 0..FLOCK_SIZE {
            let force = compute_force(&positions, &velocities, i, &flock_params);
            new_velocities[i] += force;

            // Speed limit
            if new_velocities[i].magnitude_squared() > flock_params.max_speed * flock_params.max_speed {
                new_velocities[i] = new_velocities[i].normalize() * flock_params.max_speed;
            }
        }
        velocities = new_velocities;

        for i in 0..FLOCK_SIZE {
            positions[i] += velocities[i];

            // Bounce off walls smoothly to keep them inside the grid
            if positions[i].x < 2.0 {
                positions[i].x = 2.0;
                velocities[i].x *= -1.0;
            } else if positions[i].x > (GRID_WIDTH as f64 - 2.0) {
                positions[i].x = GRID_WIDTH as f64 - 2.0;
                velocities[i].x *= -1.0;
            }

            if positions[i].y < 2.0 {
                positions[i].y = 2.0;
                velocities[i].y *= -1.0;
            } else if positions[i].y > (GRID_HEIGHT as f64 - 2.0) {
                positions[i].y = GRID_HEIGHT as f64 - 2.0;
                velocities[i].y *= -1.0;
            }

            // --- 2. Inject acoustic pressure from swarm ---
            // Boids inject pressure proportional to their velocity magnitude
            let speed = velocities[i].magnitude_squared() as f32;
            let px = positions[i].x.round() as usize;
            let py = positions[i].y.round() as usize;

            if px > 0 && px < GRID_WIDTH - 1 && py > 0 && py < GRID_HEIGHT - 1 {
                 grid.pluck(px, py, speed * 0.05);
            }
        }

        // --- 3. Step the acoustic wave simulation ---
        // Run physics multiple times per frame for stability and wave speed
        for _ in 0..2 {
             grid.step();
        }

        // --- 4. Render ---
        clear_background(BLACK);

        // Render Wave Grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let p = grid.get(x, y).abs() * 5.0; // scale pressure for visibility
                let c = if p > 0.0 {
                    Color::new(p.clamp(0.0, 1.0), p.clamp(0.0, 0.5), p.clamp(0.0, 0.8), 1.0)
                } else {
                    BLACK
                };
                image.set_pixel(x as u32, y as u32, c);
            }
        }
        texture.update(&image);

        // Draw the acoustic texture scaled up
        draw_texture_ex(
            &texture,
            (screen_width() - GRID_WIDTH as f32 * cell_size) / 2.0,
            (screen_height() - GRID_HEIGHT as f32 * cell_size) / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GRID_WIDTH as f32 * cell_size, GRID_HEIGHT as f32 * cell_size)),
                ..Default::default()
            },
        );

        // Draw the swarm on top
        let offset_x = (screen_width() - GRID_WIDTH as f32 * cell_size) / 2.0;
        let offset_y = (screen_height() - GRID_HEIGHT as f32 * cell_size) / 2.0;
        for pos in &positions {
            draw_circle(
                offset_x + pos.x as f32 * cell_size,
                offset_y + pos.y as f32 * cell_size,
                2.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
