mod world;

use ::rand::Rng;
use clap::Parser;
use macroquad::prelude::*;
use world::{AgentKind, Material, World, HEIGHT, WIDTH}; // Use the external rand crate trait

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in headless mode (no window, output image)
    #[arg(long)]
    headless: bool,
}

#[macroquad::main("Thermo-Termites")]
async fn main() {
    let args = Args::parse();

    let mut world = World::new();
    let mut rng = ::rand::thread_rng();

    // Init World
    // Add Server Rack (Grid Layout)
    let margin = 50;
    let rack_spacing = 80;
    let rack_size = 30;
    for y in 0..4 {
        for x in 0..5 {
            let sx = margin + x * rack_spacing;
            let sy = margin + y * rack_spacing + 100; // Lower half
            world.add_server_block(sx, sy, rack_size, rack_size);
        }
    }

    // Add some initial random walls (noise) for termites to erode/build upon
    for _ in 0..2000 {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let idx = world.get_index(x, y);
        if world.grid[idx].material == Material::Empty {
            world.grid[idx].material = Material::Wall;
        }
    }

    // Add Agents
    // 50k Air, 20k Termites
    for _ in 0..50000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_air(x, y));
    }
    for _ in 0..20000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_termite(x, y));
    }

    let texture =
        Texture2D::from_image(&Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK));
    let mut render_target = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    let max_frames = if args.headless { 100 } else { u64::MAX };

    // View Modes
    let mut view_mode = 0; // 0=Heat, 1=Pheromone, 2=Velocity
    let mut show_termites = true;
    let mut show_air = true;
    let mut paused = false;

    loop {
        // Input Handling
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::Key1) {
            view_mode = 0;
        }
        if is_key_pressed(KeyCode::Key2) {
            view_mode = 1;
        }
        if is_key_pressed(KeyCode::Key3) {
            view_mode = 2;
        }
        if is_key_pressed(KeyCode::T) {
            show_termites = !show_termites;
        }
        if is_key_pressed(KeyCode::A) {
            show_air = !show_air;
        }

        if !paused {
            world.update();
        }

        // Render to Image
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = world.get_cell(x, y);
                let color = match cell.material {
                    Material::Server => RED,
                    Material::Wall => Color::new(0.8, 0.8, 0.8, 1.0),
                    Material::Empty => {
                        match view_mode {
                            0 => {
                                // Heat Map
                                let h = (cell.heat / 100.0).clamp(0.0, 1.0);
                                // Cold Blue -> Hot Red
                                Color::new(h, 0.2, 1.0 - h, 1.0)
                            }
                            1 => {
                                // Pheromone Map
                                let p = (cell.pheromone / 50.0).clamp(0.0, 1.0);
                                Color::new(0.0, p, 0.0, 1.0)
                            }
                            2 => {
                                // Velocity Map
                                let vx = cell.air_vx;
                                let vy = cell.air_vy;
                                let speed = (vx * vx + vy * vy).sqrt();
                                let s = (speed * 5.0).clamp(0.0, 1.0);
                                Color::new(s, s, s, 1.0)
                            }
                            _ => BLACK,
                        }
                    }
                };
                render_target.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw Agents
        for agent in &world.agents {
            let x = agent.x as u32;
            let y = agent.y as u32;
            if x < WIDTH as u32 && y < HEIGHT as u32 {
                match agent.kind {
                    AgentKind::Air => {
                        if show_air {
                            // Air color based on temp
                            let h = (agent.heat / 50.0).clamp(0.0, 1.0);
                            let air_color = Color::new(h, 0.2, 1.0 - h, 1.0);

                            if view_mode == 0 {
                                // Additive-ish for Heat Mode
                                let current = render_target.get_pixel(x, y);
                                render_target.set_pixel(
                                    x,
                                    y,
                                    Color::new(
                                        (current.r + air_color.r * 0.3).min(1.0),
                                        (current.g + air_color.g * 0.3).min(1.0),
                                        (current.b + air_color.b * 0.3).min(1.0),
                                        1.0,
                                    ),
                                );
                            } else {
                                // Simple draw for other modes
                                render_target.set_pixel(x, y, air_color);
                            }
                        }
                    }
                    AgentKind::Termite => {
                        if show_termites {
                            let color = if agent.carrying { GREEN } else { BLUE };
                            render_target.set_pixel(x, y, color);
                        }
                    }
                };
            }
        }

        texture.update(&render_target);

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Agents: {}", world.agents.len()),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("Step: {}", world.step), 10.0, 60.0, 20.0, WHITE);
        draw_text(
            "1:Heat 2:Phero 3:Vel T:Termites A:Air Space:Pause",
            10.0,
            HEIGHT as f32 - 10.0,
            20.0,
            WHITE,
        );

        if args.headless {
            if world.step >= max_frames {
                // Save using image crate
                let bytes = &render_target.bytes;
                if let Err(e) = image::save_buffer(
                    "output.png",
                    bytes,
                    WIDTH as u32,
                    HEIGHT as u32,
                    image::ColorType::Rgba8,
                ) {
                    eprintln!("Failed to save image: {}", e);
                } else {
                    println!("Headless mode complete. Saved output.png");
                }
                break;
            }
        }

        next_frame().await;
    }
}
