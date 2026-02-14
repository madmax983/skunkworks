use macroquad::prelude::*;
use thermo_defense::agent::AgentType;
use thermo_defense::grid::{self, HEIGHT, Material, WIDTH};
use thermo_defense::sim::World;

#[macroquad::main("Thermo-Defense")]
async fn main() {
    let mut world = World::new();

    // Initial Setup
    // Central Server
    world.add_server(WIDTH / 2 - 10, HEIGHT / 2 - 10, 20, 20);

    // Spawn 1000 Termites
    world.spawn_agents(1000, AgentType::Termite);

    // Spawn 500 Locusts
    world.spawn_agents(500, AgentType::Locust);

    // Create a texture to draw the grid
    let texture =
        Texture2D::from_image(&Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    loop {
        // Handle Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map screen coords to grid coords
            // Assume texture fills screen
            let grid_x = (mx / screen_width() * WIDTH as f32) as usize;
            let grid_y = (my / screen_height() * HEIGHT as f32) as usize;

            if grid_x < WIDTH && grid_y < HEIGHT {
                let idx = grid::Grid::get_index(grid_x, grid_y);
                world.grid.cells[idx].material = Material::Server;
                world.grid.cells[idx].heat += 500.0;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let grid_x = (mx / screen_width() * WIDTH as f32) as usize;
            let grid_y = (my / screen_height() * HEIGHT as f32) as usize;

            if grid_x < WIDTH && grid_y < HEIGHT {
                let idx = grid::Grid::get_index(grid_x, grid_y);
                world.grid.cells[idx].material = Material::Wall;
            }
        }

        if is_key_pressed(KeyCode::T) {
            world.spawn_agents(100, AgentType::Termite);
        }
        if is_key_pressed(KeyCode::L) {
            world.spawn_agents(100, AgentType::Locust);
        }
        if is_key_pressed(KeyCode::R) {
            world = World::new();
            world.add_server(WIDTH / 2 - 10, HEIGHT / 2 - 10, 20, 20);
        }

        // Update Simulation
        world.update();

        // Render
        // Draw Grid
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = grid::Grid::get_index(x, y);
                let cell = &world.grid.cells[idx];

                let color = match cell.material {
                    Material::Server => RED,
                    Material::Wall => WHITE,
                    Material::Empty => {
                        // Heat Map + Pheromone
                        let h = (cell.heat / 100.0).clamp(0.0, 1.0);
                        let pd = (cell.pheromone_defense / 50.0).clamp(0.0, 1.0);
                        let pa = (cell.pheromone_attack / 50.0).clamp(0.0, 1.0);

                        // Mix colors
                        Color::new(h + pa * 0.5, pa + pd * 0.2, pd + h * 0.1, 1.0)
                    }
                    Material::Vent => GRAY,
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw Agents
        for agent in &world.agents {
            let x = agent.position.x as u32;
            let y = agent.position.y as u32;
            if x < WIDTH as u32 && y < HEIGHT as u32 {
                let color = match agent.kind {
                    AgentType::Termite => {
                        if agent.carrying {
                            YELLOW
                        } else {
                            BLUE
                        }
                    }
                    AgentType::Locust => GREEN,
                };
                image.set_pixel(x, y, color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_text(
            "Left: Spawn Server | Right: Spawn Wall | T: Termites | L: Locusts",
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Agents: {}", world.agents.len()),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 60.0, 20.0, WHITE);

        next_frame().await;
    }
}
