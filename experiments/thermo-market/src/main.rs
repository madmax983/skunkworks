mod world;

use ::rand::Rng;
use clap::Parser;
use macroquad::prelude::*;
use market_sim::Particle;
use world::{AgentKind, Material, World, HEIGHT, WIDTH};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in headless mode (no window, output image)
    #[arg(long)]
    headless: bool,
}

#[macroquad::main("Thermo-Market")]
async fn main() {
    let args = Args::parse();

    let mut world = World::new();
    let mut rng = ::rand::thread_rng();

    // Init World
    // Add "Central Bank" / Exchange
    // world.add_server_block(WIDTH / 2 - 20, HEIGHT / 2 - 20, 40, 40);
    // Actually, let the market generate the heat.

    // Add some initial random walls (noise)
    for _ in 0..2000 {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let idx = world.get_index(x, y);
        if world.grid[idx].material == Material::Empty {
            world.grid[idx].material = Material::Wall;
        }
    }

    // Add Agents
    // 20k Air, 5k Termites
    for _ in 0..20000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_air(x, y));
    }
    for _ in 0..5000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_termite(x, y));
    }

    let texture =
        Texture2D::from_image(&Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK));
    let mut render_target = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    let max_frames = if args.headless { 100 } else { u64::MAX };

    // View Modes
    let mut view_mode = 0; // 0=Heat+Market, 1=Pheromone, 2=MarketOnly
    let mut show_termites = true;
    let mut show_air = false;
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
                let market_p = world.market.get(x, y);

                let base_color = match cell.material {
                    Material::Server => RED,
                    Material::Wall => Color::new(0.5, 0.5, 0.5, 1.0),
                    Material::Empty => {
                        match view_mode {
                            0 => {
                                // Heat Map
                                let h = (cell.heat / 100.0).clamp(0.0, 1.0);
                                // Cold Blue -> Hot Red
                                Color::new(h, 0.1, 0.2, 1.0)
                            }
                            1 => {
                                // Pheromone Map
                                let p = (cell.pheromone / 50.0).clamp(0.0, 1.0);
                                Color::new(0.0, p, 0.0, 1.0)
                            }
                            2 => BLACK,
                            _ => BLACK,
                        }
                    }
                };

                // Overlay Market Particles
                let final_color = match market_p {
                    Particle::Bid(_) => GREEN,
                    Particle::Ask(_) => RED,
                    Particle::Trade { age } => {
                        let intensity = (age as f32 / 5.0).clamp(0.2, 1.0);
                        Color::new(1.0, 1.0, 0.0, intensity)
                    }
                    Particle::Wall => {
                        // Market sees a wall
                        if view_mode == 2 {
                            WHITE
                        } else {
                            base_color
                        }
                    }
                    Particle::Empty => base_color,
                };

                render_target.set_pixel(x as u32, y as u32, final_color);
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
                            render_target.set_pixel(x, y, Color::new(0.5, 0.5, 1.0, 0.5));
                        }
                    }
                    AgentKind::Termite => {
                        if show_termites {
                            let color = if agent.carrying { BLUE } else { WHITE };
                            render_target.set_pixel(x, y, color);
                        }
                    }
                };
            }
        }

        texture.update(&render_target);

        clear_background(BLACK);

        // Scale to fit window?
        let scale = (screen_height() / HEIGHT as f32).min(screen_width() / WIDTH as f32);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH as f32 * scale, HEIGHT as f32 * scale)),
                ..Default::default()
            },
        );

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Step: {}", world.step), 10.0, 40.0, 20.0, WHITE);
        draw_text(
            &format!("Trades: {}", world.market.trade_count),
            10.0,
            60.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!(
                "Bids: {} Asks: {}",
                world.market.total_bids, world.market.total_asks
            ),
            10.0,
            80.0,
            20.0,
            GREEN,
        );

        draw_text(
            "1:Heat 2:Phero 3:Mkt T:Termites A:Air Space:Pause",
            10.0,
            screen_height() - 10.0,
            20.0,
            WHITE,
        );

        if args.headless {
            if world.step >= max_frames {
                let bytes = &render_target.bytes;
                if let Err(e) = image::save_buffer(
                    "output.png",
                    bytes,
                    WIDTH as u32,
                    HEIGHT as u32,
                    image::ColorType::Rgba8,
                ) {
                    eprintln!("Failed to save image: {}", e);
                }
                break;
            }
        }

        next_frame().await;
    }
}
