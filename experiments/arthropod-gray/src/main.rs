//! # Arthropod Gray 🐜🧪
//!
//! **Concept:** Interactive Chemical Morphogenesis.
//!
//! This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the continuous chemical reaction-diffusion simulation of `gray-scott`.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and color logic.
//! - **Parent B (crates/gray-scott):** Provides the continuous thermodynamic reaction-diffusion simulation.
//!
//! ## Novel Trait
//! The continuous morphogenetic simulation is wrapped with `arthropod`'s interactive layer. The thermodynamic rules aren't static; by clicking discrete buttons, the user dynamically modulates the `feed` and `kill` chemical rates in real-time.
//!
//! ## Predicted Phenotype
//! An emergent, interactive playground. The continuous biological patterns can be instantly forced to change from spots to stripes, or dissipate entirely, through pure UI interaction, blending abstract graphical controls directly with morphogenetic biological models.

use arthropod::Button;
use gray_scott::GrayScott;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Gray".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Bypass macroquad::main to support headless execution
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut gs = GrayScott::new(100, 100);
    // seed
    for x in 45..55 {
        for y in 45..55 {
            gs.add_chemical(x, y, 1.0);
        }
    }

    let mut feed = 0.055;
    let mut kill = 0.062;

    let btn_spots = Button::new("Spots", 20.0, 20.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_stripes = Button::new("Stripes", 20.0, 70.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_solitons = Button::new("Solitons", 20.0, 120.0, 100.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);
    let btn_reset = Button::new("Reset Seed", 20.0, 170.0, 100.0, 40.0).with_colors(GRAY, LIGHTGRAY, BLACK);

    let mut texture = Texture2D::empty();
    texture.set_filter(FilterMode::Nearest);
    let mut image = Image::gen_image_color(100, 100, BLACK);

    loop {
        clear_background(BLACK);

        if btn_spots.draw() {
            feed = 0.055;
            kill = 0.062;
        }

        if btn_stripes.draw() {
            feed = 0.022;
            kill = 0.051;
        }

        if btn_solitons.draw() {
            feed = 0.030;
            kill = 0.055;
        }

        if btn_reset.draw() {
            gs = GrayScott::new(100, 100);
            for x in 45..55 {
                for y in 45..55 {
                    gs.add_chemical(x, y, 1.0);
                }
            }
        }

        // Run multiple simulation steps per frame for speed
        for _ in 0..10 {
            gs.update(feed, kill, 1.0);
        }

        let v_slice = gs.v();
        for y in 0..100 {
            for x in 0..100 {
                let idx = gs.get_index(x, y).unwrap_or(0);
                let val = v_slice[idx];
                let c = (val * 255.0).clamp(0.0, 255.0) as u8;
                image.set_pixel(x as u32, y as u32, Color::from_rgba(c, c, c, 255));
            }
        }
        texture = Texture2D::from_image(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(800.0, 600.0)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
