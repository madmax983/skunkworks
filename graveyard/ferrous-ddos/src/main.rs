use ::rand::Rng;
use macroquad::prelude::*;

mod physics;
mod simulation;

use physics::ParticleSystem;
use simulation::{World, WORLD_SIZE};

#[macroquad::main("Magnetic Cyberwarfare")]
async fn main() {
    let mut world = World::new();

    // Texture setup
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    // Physics setup (Magnetic Particles)
    let mut particles = ParticleSystem::new();

    // Server target emitting magnetic pull
    let server_pos = Vec2::new(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);

    let mut last_update = get_time();

    loop {
        let now = get_time();
        let dt = (now - last_update) as f32;
        last_update = now;

        // Interaction
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            // Map screen coords to WORLD_SIZE
            let wx = (x / screen_w) * WORLD_SIZE;
            let wy = (y / screen_h) * WORLD_SIZE;
            world.add_firewall(wx, wy);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Spawn attackers (DDoS packets) from edges
        if ::rand::thread_rng().gen_range(0..10) < 5 {
            let spawn_x = if ::rand::thread_rng().gen_range(0..2) == 0 {
                0.0
            } else {
                WORLD_SIZE
            };
            let spawn_y = ::rand::thread_rng().gen_range(0.0..WORLD_SIZE);
            particles.add_particle(Vec2::new(spawn_x, spawn_y));
        }

        // 1. Simulation step: Server absorbs particles
        world.update(&mut particles, server_pos);

        // 2. Physics step: Magnetic repulsions & attractions
        particles.update(dt, &world, server_pos);

        // Render to image
        image.bytes.fill(0);

        // Draw Firewall (Magnetic Repulsors)
        for firewall in &world.firewalls {
            let px = (firewall.position.x / WORLD_SIZE * width as f32) as i32;
            let py = (firewall.position.y / WORLD_SIZE * height as f32) as i32;
            let radius = (firewall.radius / WORLD_SIZE * width as f32) as i32;
            draw_circle_on_image(&mut image, px, py, radius, Color::new(1.0, 0.0, 0.0, 0.5));
        }

        // Draw Server (Target)
        let px = (server_pos.x / WORLD_SIZE * width as f32) as i32;
        let py = (server_pos.y / WORLD_SIZE * height as f32) as i32;
        draw_circle_on_image(&mut image, px, py, 30, Color::new(0.0, 1.0, 0.0, 1.0));

        // Draw particles
        for p in &particles.particles {
            let px = (p.position.x / WORLD_SIZE * width as f32) as i32;
            let py = (p.position.y / WORLD_SIZE * height as f32) as i32;
            if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                // Color based on pressure/velocity
                let speed = p.velocity.length();
                let color = Color::new(speed / 50.0, 0.5, 1.0 - (speed / 100.0), 1.0);
                draw_pixel_on_image(&mut image, px, py, color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_text("Magnetic Cyberwarfare", 10.0, 20.0, 20.0, WHITE);
        draw_text(
            "Left Click: Deploy Magnetic Firewall",
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text("C: Clear Firewalls", 10.0, 60.0, 20.0, WHITE);
        draw_text(
            &format!("Packets: {}", particles.particles.len()),
            10.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

// Helper drawing functions for image
fn draw_pixel_on_image(img: &mut Image, x: i32, y: i32, color: Color) {
    if x >= 0 && x < img.width as i32 && y >= 0 && y < img.height as i32 {
        img.set_pixel(x as u32, y as u32, color);
    }
}

fn draw_circle_on_image(img: &mut Image, cx: i32, cy: i32, r: i32, color: Color) {
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= r * r {
                draw_pixel_on_image(img, cx + x, cy + y, color);
            }
        }
    }
}
