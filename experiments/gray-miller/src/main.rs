//! # gray-miller
//!
//! A hybrid experiment combining `miller-lattice` and `gray-scott`.
//!
//! ## Concept: Reaction-Diffusion Codebase Crystals
//!
//! **Lineage:**
//! - Inherits the 3D procedural generation of a file system crystal lattice from `miller-lattice`.
//! - Inherits the continuous chemical reaction-diffusion simulation from `gray-scott`.
//! - **Novel Trait:** The discrete crystalline file structure is squashed onto the 2D plane and serves as the initial "seed" points (spores of chemical V) in a continuous reaction-diffusion field. The codebase literally blooms and grows into Turing patterns.
//!
//! ## Running
//!
//! ```bash
//! cargo run -p gray-miller --release
//! ```
//!
use gray_scott::GrayScott;
use miller_lattice::Crystal;
use std::path::Path;

const GRID_SIZE: usize = 200;

fn main() {
    let mut dish = GrayScott::new(GRID_SIZE, GRID_SIZE);

    // 1. Build the crystal lattice
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap();

    // 2. Map the 3D crystal onto the 2D Gray-Scott dish to deposit 'seeds' of chemical V
    let mut max_abs_val = 1; // avoid division by zero
    for atom in &crystal.atoms {
        max_abs_val = max_abs_val.max(atom.position.x.abs());
        max_abs_val = max_abs_val.max(atom.position.y.abs());
    }

    let scale = (GRID_SIZE as f32 * 0.4) / (max_abs_val as f32);
    let center = GRID_SIZE as f32 / 2.0;

    for atom in &crystal.atoms {
        let px = (atom.position.x as f32 * scale + center) as usize;
        let py = (atom.position.y as f32 * scale + center) as usize;

        let px = px.clamp(0, GRID_SIZE - 1);
        let py = py.clamp(0, GRID_SIZE - 1);

        dish.add_chemical(px, py, 1.0);
    }

    let (feed, kill) = (0.055, 0.062); // standard spots

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--headless") {
        for _ in 0..10 {
            dish.update(feed, kill, 1.0);
        }
        println!("Headless execution successful.");
        return;
    }

    start_macroquad(dish, feed, kill);
}

#[cfg(not(test))]
fn start_macroquad(dish: GrayScott, feed: f32, kill: f32) {
    macroquad::Window::from_config(
        macroquad::window::Conf {
            window_title: "Gray-Miller Morphogenesis".to_owned(),
            window_width: 800,
            window_height: 800,
            ..Default::default()
        },
        amain(dish, feed, kill),
    );
}

#[cfg(test)]
fn start_macroquad(_dish: GrayScott, _feed: f32, _kill: f32) {}

#[allow(dead_code)]
async fn amain(mut dish: GrayScott, feed: f32, kill: f32) {
    use macroquad::prelude::*;
    let mut image = Image::gen_image_color(GRID_SIZE as u16, GRID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        for _ in 0..10 {
            dish.update(feed, kill, 1.0);
        }

        let w = dish.width();
        let v_buf = dish.v();

        for y in 0..dish.height() {
            for x in 0..w {
                let v = v_buf[y * w + x];
                let c = (v * 255.0).clamp(0.0, 255.0) as u8;
                image.set_pixel(x as u32, y as u32, Color::from_rgba(0, c, c, 255));
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

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_miller_headless() {
        let mut dish = GrayScott::new(10, 10);
        let crystal = Crystal::build_from_path(Path::new(".")).unwrap();

        for _atom in &crystal.atoms {
            dish.add_chemical(5, 5, 1.0); // Simplified seed
        }

        dish.update(0.055, 0.062, 1.0);

        // Ensure V diffuses successfully
        let v_buf = dish.v();
        let sum: f32 = v_buf.iter().sum();
        assert!(sum > 0.0, "Simulation failed to update V concentrations");
    }
}
