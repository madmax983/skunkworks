mod simulation;
use macroquad::prelude::*;
use simulation::{World, WORLD_SIZE};

#[macroquad::main("Locust Reaper")]
async fn main() {
    let mut world = World::new();

    // The logic runs 60 times a second approximately, using Macroquad's frame loop
    loop {
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.add_garbage(world_mouse, 20.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_garbage();
        }

        world.update();

        clear_background(BLACK);

        world.draw();

        // Draw HUD
        draw_text("Locust Reaper (DDoS + GC)", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Left Click: Spawn Dead Nodes (Garbage)",
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text("C: Clear nodes", 10.0, 80.0, 20.0, GRAY);

        let agents_alive = world.agents.iter().filter(|a| a.state == 0).count();
        draw_text(
            &format!("Agents: {}", agents_alive),
            10.0,
            110.0,
            20.0,
            GREEN,
        );

        next_frame().await;
    }
}
