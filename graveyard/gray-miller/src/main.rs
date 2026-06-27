use gray_scott::GrayScott;
use macroquad::prelude::*;
use miller_lattice::Crystal;
use std::env;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gray-Miller Morphogenesis".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn run_sim() {
    let width = 160;
    let height = 120;
    let mut gs = GrayScott::new(width, height);

    // Default Gray-Scott parameters for "Spots"
    let feed = 0.055;
    let kill = 0.062;
    let dt = 1.0;

    let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    // Find bounds to map to the 2D grid
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for atom in &crystal.atoms {
        min_x = min_x.min(atom.position.x);
        max_x = max_x.max(atom.position.x);
        min_y = min_y.min(atom.position.y);
        max_y = max_y.max(atom.position.y);
    }

    let w = (max_x - min_x).max(1) as f32;
    let h = (max_y - min_y).max(1) as f32;

    // Seed the continuous reaction-diffusion grid based on discrete crystal nodes
    for atom in &crystal.atoms {
        let normalized_x = (atom.position.x - min_x) as f32 / w;
        let normalized_y = (atom.position.y - min_y) as f32 / h;

        let grid_x = (normalized_x * (width as f32 - 1.0)).round() as usize;
        let grid_y = (normalized_y * (height as f32 - 1.0)).round() as usize;

        let grid_x = grid_x.clamp(1, width - 2);
        let grid_y = grid_y.clamp(1, height - 2);

        // Directories provide stronger seeds
        let amount = if atom.is_dir { 1.0 } else { 0.5 };
        gs.add_chemical(grid_x, grid_y, amount);

        // Spread the seed slightly
        gs.add_chemical(grid_x + 1, grid_y, amount * 0.5);
        gs.add_chemical(grid_x, grid_y + 1, amount * 0.5);
    }

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        for _ in 0..10 {
            gs.update(feed, kill, dt);
        }

        let pixels = image.get_image_data_mut();
        for y in 0..height {
            for x in 0..width {
                if let Some(idx) = gs.get_index(x, y) {
                    let u = gs.u()[idx];
                    let v = gs.v()[idx];

                    let c = ((v / (u + v + 0.0001)) * 255.0).clamp(0.0, 255.0) as u8;
                    pixels[idx] = [c, c.saturating_sub(50), 255, 255];
                }
            }
        }
        texture.update(&image);

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

        draw_text("Gray-Miller", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Crystalline codebase structures seeding biological morphogenesis",
            10.0,
            50.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode activated. Bypassing execution.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
