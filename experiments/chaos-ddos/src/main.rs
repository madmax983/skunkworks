use ::rand::Rng;
use macroquad::prelude::*;

mod physics;
mod simulation;

use physics::PendulumSystem;
use simulation::{World, WORLD_SIZE};

#[macroquad::main("Chaos DDoS")]
async fn main() {
    let mut world = World::new();

    // Texture setup (Fixed internal resolution)
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    // Physics setup (Double Pendulum)
    let mut pendulum = PendulumSystem::new();
    let root = pendulum.add_node(Vec2::new(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0), 1.0, true, "Root".to_string());
    let joint1 = pendulum.add_node(Vec2::new(WORLD_SIZE / 2.0 + 100.0, WORLD_SIZE / 2.0), 10.0, false, "Joint1".to_string());
    let joint2 = pendulum.add_node(Vec2::new(WORLD_SIZE / 2.0 + 200.0, WORLD_SIZE / 2.0), 10.0, false, "Joint2".to_string());
    pendulum.add_link(root, joint1, 150.0);
    pendulum.add_link(joint1, joint2, 150.0);

    // Initial kick
    let mut rng = ::rand::thread_rng();
    pendulum.nodes[joint2].prev_pos += Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));

    loop {
        let dt = get_frame_time();

        // Update physics
        pendulum.step(dt);

        // Update target to the tip of the pendulum
        let target_pos = pendulum.nodes[joint2].pos;
        world.target = macroquad::math::Vec2::new(target_pos.x, target_pos.y);

        // Input for firewalls
        let mouse_pos = mouse_position();
        let world_mouse = macroquad::math::Vec2::new(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.add_firewall(world_mouse, 20.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Update swarm
        world.update();

        // Render to buffer
        world.render_to_buffer(&mut image.bytes, width as usize, height as usize);
        texture.update(&image);

        // Draw the simulation
        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the chaotic pendulum over it
        let screen_scale_x = screen_width() / WORLD_SIZE;
        let screen_scale_y = screen_height() / WORLD_SIZE;

        for link in &pendulum.links {
            let p1 = pendulum.nodes[link.a].pos;
            let p2 = pendulum.nodes[link.b].pos;
            draw_line(
                p1.x * screen_scale_x,
                p1.y * screen_scale_y,
                p2.x * screen_scale_x,
                p2.y * screen_scale_y,
                3.0,
                GRAY,
            );
        }

        for node in &pendulum.nodes {
            let color = if node.fixed { RED } else { BLUE };
            let radius = if node.fixed { 10.0 } else { 15.0 };
            draw_circle(node.pos.x * screen_scale_x, node.pos.y * screen_scale_y, radius, color);
        }

        // UI
        draw_text("Chaos DDoS", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Server Health: {:.1}%", (world.server_health / world.max_health) * 100.0),
            20.0,
            60.0,
            20.0,
            if world.server_health > 0.0 { GREEN } else { RED },
        );
        draw_text("Left Click: Deploy Firewall | C: Clear Firewalls", 20.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}
