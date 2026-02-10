mod render;
mod simulation;

use macroquad::prelude::*;
use poincare_disk::{Mobius, Point, TilingConsts};
use render::{draw_bee, draw_source, draw_tile, to_hyperbolic};
use simulation::{BeeState, World};

#[macroquad::main("Hyperbolic Waggle")]
async fn main() {
    let mut world = World::new(100);
    let tiling = TilingConsts::new_4_5(); // Hyperbolic tiling

    loop {
        // Input
        let scale = screen_height().min(screen_width()) * 0.45;

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let screen_pos = vec2(mouse_pos.0, mouse_pos.1);
            let p = to_hyperbolic(screen_pos, scale);
            if p.norm() < 0.99 {
                // Add source
                let quality = ::rand::random::<f64>() + 0.5;
                world.add_source(p, quality);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            // Spawn 10 more bees
            for _ in 0..10 {
                world.bees.push(simulation::Bee {
                    pos: Point::new(0.0, 0.0),
                    state: BeeState::Scouting,
                    target_source: None,
                    dance_angle: 0.0,
                    dance_quality: 0.0,
                    dance_timer: 0.0,
                    forage_timer: 0.0,
                });
            }
        }

        if is_key_pressed(KeyCode::R) {
            world = World::new(100);
        }

        // Update
        world.update();

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // Draw Tiling Background
        draw_tile(Mobius::identity(), 0, &tiling, None);

        // Draw Sources
        for source in &world.sources {
            draw_source(source.pos, source.quality, scale);
        }

        // Draw Hive (Origin)
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 15.0, Color::new(0.8, 0.6, 0.2, 0.5));
        draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, 15.0, 2.0, GOLD);

        // Draw Bees
        for bee in &world.bees {
            let color = match bee.state {
                BeeState::Scouting => WHITE,
                BeeState::Returning => SKYBLUE,
                BeeState::Dancing => GOLD,
                BeeState::Observing => DARKGRAY,
                BeeState::Foraging => ORANGE,
            };
            draw_bee(bee.pos, color, scale);

            // If dancing, draw a line indicating direction
            if bee.state == BeeState::Dancing {
                // Draw a small line from center in direction of dance_angle
                let center = vec2(screen_width()/2.0, screen_height()/2.0);
                let len = 20.0;
                let angle = bee.dance_angle as f32; // In screen space, angle matches?
                // Yes, because at origin, Poincare angle matches Euclidean angle.
                // But Y is flipped in screen coords relative to standard math?
                // to_screen: y - p.im * scale. So +im is UP.
                // Bee angle comes from source.arg(). Source.arg() is standard math (atan2(im, re)).
                // So +PI/2 is +im (UP).
                // Screen coordinates: +Y is DOWN.
                // So we need to negate angle or flip Y.
                // Let's check visually.

                let end = vec2(
                    center.x + len * angle.cos(),
                    center.y - len * angle.sin(), // Flip sin for screen Y
                );
                draw_line(center.x, center.y, end.x, end.y, 2.0, GOLD);
            }
        }

        // UI
        draw_text("Hyperbolic Waggle Dance", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left Click: Add Food Source", 10.0, 40.0, 20.0, GRAY);
        draw_text("Space: Spawn Bees", 10.0, 60.0, 20.0, GRAY);

        let counts = [
            ("Scout", BeeState::Scouting),
            ("Return", BeeState::Returning),
            ("Dance", BeeState::Dancing),
            ("Observe", BeeState::Observing),
            ("Forage", BeeState::Foraging),
        ];

        for (i, (name, state)) in counts.iter().enumerate() {
            let count = world.bees.iter().filter(|b| b.state == *state).count();
            draw_text(&format!("{}: {}", name, count), 10.0, 100.0 + i as f32 * 20.0, 20.0, WHITE);
        }

        next_frame().await
    }
}
