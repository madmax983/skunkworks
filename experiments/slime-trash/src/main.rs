use macroquad::prelude::*;
use slime_trash::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Slime Trash: Garbage Truck Pathfinding".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = 800;
    let height = 600;
    let mut world = World::new(width, height);

    // Initial setup
    // Random Agents
    for _ in 0..1000 {
        world.add_agent(macroquad::rand::gen_range(0.0, width as f32), macroquad::rand::gen_range(0.0, height as f32));
    }

    // Random Trash
    for _ in 0..5 {
        world.add_trash(
            macroquad::rand::gen_range(100.0, width as f32 - 100.0),
            macroquad::rand::gen_range(100.0, height as f32 - 100.0),
            500.0,
        );
    }

    // Random Dumps
    for _ in 0..2 {
        world.add_dump(
            macroquad::rand::gen_range(50.0, width as f32 - 50.0),
            macroquad::rand::gen_range(50.0, height as f32 - 50.0),
        );
    }

    let mut pheromone_image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let pheromone_texture = Texture2D::from_image(&pheromone_image);

    loop {
        if is_key_pressed(KeyCode::R) {
            world = World::new(width, height);
            for _ in 0..1000 {
                world.add_agent(macroquad::rand::gen_range(0.0, width as f32), macroquad::rand::gen_range(0.0, height as f32));
            }
             for _ in 0..5 {
                world.add_trash(
                    macroquad::rand::gen_range(100.0, width as f32 - 100.0),
                    macroquad::rand::gen_range(100.0, height as f32 - 100.0),
                    500.0,
                );
            }
             for _ in 0..2 {
                world.add_dump(
                    macroquad::rand::gen_range(50.0, width as f32 - 50.0),
                    macroquad::rand::gen_range(50.0, height as f32 - 50.0),
                );
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            world.add_trash(mx, my, 10.0);
        }

        world.update(get_frame_time());

        clear_background(BLACK);

        // Update Pheromone Texture
        // This is slow (CPU -> GPU), but fine for < 1M pixels in a toy sim.
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let t = world.trash_trails[idx]; // Greenish
                let d = world.dump_trails[idx];  // Reddish

                // Color mapping:
                // Trash Trail (Attracts Empty agents to Trash) -> Should look enticing?
                // Wait, if I am seeking Trash, I follow Trash Trail.
                // Let's visualize:
                // Trash Pheromone (Green)
                // Dump Pheromone (Red)

                let r = (d * 255.0).min(255.0) as u8;
                let g = (t * 255.0).min(255.0) as u8;
                let b = 0;

                pheromone_image.set_pixel(x as u32, y as u32, Color::from_rgba(r, g, b, 255));
            }
        }
        pheromone_texture.update(&pheromone_image);
        draw_texture(&pheromone_texture, 0.0, 0.0, WHITE);

        // Draw Trash
        for trash in &world.trash_piles {
            draw_circle(trash.0, trash.1, 10.0 + trash.2 * 0.05, GREEN);
        }

        // Draw Dumps
        for dump in &world.dumps {
            draw_circle(dump.0, dump.1, 20.0, RED);
            draw_circle_lines(dump.0, dump.1, 20.0, 2.0, WHITE);
        }

        // Draw Agents
        // Too many agents to draw individually? 1000 is fine.
        for agent in &world.agents {
            let color = if agent.cargo > 0.0 { BROWN } else { WHITE };
            draw_circle(agent.position.0, agent.position.1, 2.0, color);
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 30.0, WHITE);
        draw_text("Click to add trash. R to reset.", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
