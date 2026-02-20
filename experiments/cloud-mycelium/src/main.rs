use macroquad::prelude::*;
use cloud_mycelium::World;
use ::rand::Rng; // Disambiguate to use the rand crate

#[derive(Clone)]
struct RainDrop {
    pos: Vec2,
    speed: f32,
}

#[macroquad::main("Cloud Mycelium")]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());
    let mut rain: Vec<RainDrop> = Vec::new();
    let mut rng = ::rand::thread_rng(); // Use rand crate RNG

    // Initial Setup
    for _ in 0..10 {
        let x = rng.gen_range(50.0..screen_width() - 50.0);
        let y = rng.gen_range(50.0..screen_height() - 50.0);
        world.add_mushroom(glam::Vec2::new(x, y));
    }

    loop {
        let dt = get_frame_time();

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            world.add_mushroom(glam::Vec2::new(mx, my));
        }

        if is_key_pressed(KeyCode::R) {
            world = World::new(screen_width(), screen_height());
            rain.clear();
            for _ in 0..10 {
                let x = rng.gen_range(50.0..screen_width() - 50.0);
                let y = rng.gen_range(50.0..screen_height() - 50.0);
                world.add_mushroom(glam::Vec2::new(x, y));
            }
        }

        let storm_mode = is_key_down(KeyCode::Space);

        // Update Rain
        // Spawn Rain
        if storm_mode || rng.gen_bool(0.1) { // 10% chance per frame normally, 100% in storm
             let x = rng.gen_range(0.0..screen_width());
             rain.push(RainDrop {
                 pos: vec2(x, -10.0),
                 speed: rng.gen_range(200.0..400.0),
             });
        }

        let mut dead_rain = Vec::new();
        for (i, drop) in rain.iter_mut().enumerate() {
            drop.pos.y += drop.speed * dt;

            // Check collision with mushrooms
            for mushroom in world.mushrooms.iter_mut() {
                // We need to access mushroom position which is Vec2 (glam)
                let m_pos = vec2(mushroom.pos.x, mushroom.pos.y);
                if drop.pos.distance(m_pos) < 20.0 { // Radius 20
                    mushroom.load += 10.0;
                    dead_rain.push(i);
                    break;
                }
            }

            if drop.pos.y > screen_height() {
                dead_rain.push(i);
            }
        }

        // Remove duplicates and sort to remove efficiently
        dead_rain.sort_unstable();
        dead_rain.dedup();
        for i in dead_rain.into_iter().rev() {
            rain.remove(i);
        }

        // Update World
        world.update(dt);

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Deep Blue

        // Draw Hyphae
        for hypha in &world.hyphae {
            let start = world.mushrooms[hypha.from].pos;
            let end = world.mushrooms[hypha.to].pos;

            // Pulse effect
            let pulse = (get_time() * 2.0).sin() as f32 * 0.5 + 0.5;
            let alpha = 0.2 + pulse * 0.3;

            draw_line(start.x, start.y, end.x, end.y, 2.0, Color::new(0.8, 0.8, 1.0, alpha));
        }

        // Draw Packets
        for packet in &world.packets {
            draw_circle(packet.pos.x, packet.pos.y, 3.0, YELLOW);
        }

        // Draw Rain
        let cyan = Color::new(0.0, 1.0, 1.0, 1.0);
        for drop in &rain {
            draw_line(drop.pos.x, drop.pos.y, drop.pos.x, drop.pos.y + 10.0, 1.0, cyan);
        }

        // Draw Mushrooms
        for mushroom in &world.mushrooms {
            let ratio = (mushroom.load / mushroom.capacity).clamp(0.0, 1.0);
            let color = Color::new(
                ratio, // R increases with load
                1.0 - ratio, // G decreases with load
                0.2,
                1.0
            );

            // Radius pulses with load
            let radius = 20.0 + ratio * 10.0 + (get_time() * 5.0).sin() as f32 * 2.0;

            draw_circle(mushroom.pos.x, mushroom.pos.y, radius, color);
            draw_circle_lines(mushroom.pos.x, mushroom.pos.y, radius, 2.0, WHITE);

            // Draw Load Text
            // draw_text args: text, x, y, font_size, color
            draw_text(
                &format!("{:.0}", mushroom.load),
                mushroom.pos.x - 10.0,
                mushroom.pos.y + 5.0,
                20.0,
                BLACK
            );
        }

        // UI
        draw_text("Cloud Mycelium", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Mushrooms: {} | Packets: {}", world.mushrooms.len(), world.packets.len()),
            20.0,
            60.0,
            20.0,
            GRAY
        );
        draw_text(
            "Click: Spawn | Space: Storm | R: Reset",
            20.0,
            90.0,
            20.0,
            GRAY
        );

        next_frame().await
    }
}
