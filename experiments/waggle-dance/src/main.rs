mod simulation;
use macroquad::prelude::*;
use simulation::{BeeState, World};

#[macroquad::main("Waggle Dance")]
async fn main() {
    let mut world = World::new(vec2(screen_width() / 2.0, screen_height() / 2.0), 1000);

    // Add some initial sources
    world.add_source(vec2(100.0, 100.0), 1.0); // High quality
    world.add_source(vec2(screen_width() - 100.0, screen_height() - 100.0), 0.5); // Low quality

    loop {
        // Handle Input
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if let Some(source) = world.sources.first_mut() {
                source.position = vec2(mouse_pos.0, mouse_pos.1);
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
             if let Some(source) = world.sources.first_mut() {
                source.quality = (source.quality + 0.1).min(2.0);
            }
        }

        if is_key_pressed(KeyCode::Space) {
             let mut rng = ::rand::thread_rng();
             use ::rand::Rng;
             world.add_source(vec2(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height())), rng.gen_range(0.1..1.5));
        }

        if is_key_pressed(KeyCode::R) {
            world = World::new(vec2(screen_width() / 2.0, screen_height() / 2.0), 1000);
            world.add_source(vec2(100.0, 100.0), 1.0);
            world.add_source(vec2(screen_width() - 100.0, screen_height() - 100.0), 0.5);
        }

        // Update
        world.update();

        // Draw
        clear_background(BLACK);

        // Draw Hive
        draw_circle(world.hive_position.x, world.hive_position.y, world.hive_radius, Color::new(0.5, 0.4, 0.2, 1.0));
        draw_circle_lines(world.hive_position.x, world.hive_position.y, world.hive_radius, 2.0, GOLD);

        // Draw Sources
        for source in &world.sources {
            let radius = source.radius * (0.5 + source.quality); // Visual size correlates to quality
            let color = Color::new(0.2, 0.8 * source.quality.min(1.0), 0.2, 1.0);
            draw_circle(source.position.x, source.position.y, radius, color);
            draw_text(&format!("{:.1}", source.quality), source.position.x - 10.0, source.position.y, 20.0, WHITE);
        }

        // Draw Bees
        for bee in &world.bees {
            let color = match bee.state {
                BeeState::Scouting => WHITE,
                BeeState::Returning => SKYBLUE,
                BeeState::Dancing => GOLD,
                BeeState::Observing => DARKGRAY,
                BeeState::Foraging => ORANGE,
            };

            // Draw bee
            draw_circle(bee.position.x, bee.position.y, 2.0, color);

            // Draw dance vector if dancing
            if bee.state == BeeState::Dancing {
                let dance_vec = vec2(bee.dance_angle.cos(), bee.dance_angle.sin()) * 50.0;
                draw_line(bee.position.x, bee.position.y, bee.position.x + dance_vec.x, bee.position.y + dance_vec.y, 1.0, Color::new(1.0, 0.84, 0.0, 0.5));
            }
        }

        // UI
        draw_text("Waggle Dance Simulation", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left Click: Move Source 0", 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Right Click: Increase Quality Source 0", 10.0, 70.0, 20.0, LIGHTGRAY);
        draw_text("Space: Add Random Source", 10.0, 90.0, 20.0, LIGHTGRAY);
        draw_text("R: Reset", 10.0, 110.0, 20.0, LIGHTGRAY);

        // Stats
        let scouting_count = world.bees.iter().filter(|b| b.state == BeeState::Scouting).count();
        let foraging_count = world.bees.iter().filter(|b| b.state == BeeState::Foraging).count();
        let dancing_count = world.bees.iter().filter(|b| b.state == BeeState::Dancing).count();

        draw_text(&format!("Scouting: {}", scouting_count), screen_width() - 150.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Foraging: {}", foraging_count), screen_width() - 150.0, 40.0, 20.0, ORANGE);
        draw_text(&format!("Dancing: {}", dancing_count), screen_width() - 150.0, 60.0, 20.0, GOLD);


        next_frame().await
    }
}
