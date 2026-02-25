use macroquad::prelude::*;

mod field;
mod organism;
mod physics;

use field::MagneticField;
use organism::{Hypha, HyphaAction};

fn window_conf() -> Conf {
    Conf {
        window_title: "Ferrous Mycelium".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut field = MagneticField::new(200, 150); // Aspect ratio 4:3
    let mut hyphae = Vec::new();

    // Spawn initial spores
    for i in 0..5 {
        let angle = (i as f32 / 5.0) * std::f32::consts::PI * 2.0;
        let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let vel = vec2(angle.cos(), angle.sin()) * 50.0;
        hyphae.push(Hypha::new(pos, vel, Hypha::random_dna()));
    }

    loop {
        let dt = get_frame_time(); // Seconds

        // Field Physics
        // Decay slightly every frame
        field.decay(0.995);

        // Organism Physics
        let mut new_hyphae = Vec::new();
        let mut dead_indices = Vec::new();

        for (i, hypha) in hyphae.iter_mut().enumerate() {
            let action = hypha.update(&mut field, dt);
            match action {
                HyphaAction::Branch(child) => {
                    new_hyphae.push(child);
                }
                HyphaAction::Die => {
                    dead_indices.push(i);
                }
                HyphaAction::None => {}
            }
        }

        // Remove dead
        // Sort indices in descending order to remove efficiently
        dead_indices.sort_unstable_by(|a, b| b.cmp(a));
        for i in dead_indices {
            hyphae.swap_remove(i);
        }

        // Add born
        hyphae.append(&mut new_hyphae);

        // Cap population to prevent explosion
        if hyphae.len() > 2000 {
            // Kill oldest? Or random? swap_remove shuffles order, so last part of vec is randomish
            hyphae.truncate(2000);
        }

        // Respawn if extinct
        if hyphae.is_empty() {
            for i in 0..5 {
                let angle = (i as f32 / 5.0) * std::f32::consts::PI * 2.0;
                let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
                let vel = vec2(angle.cos(), angle.sin()) * 50.0;
                hyphae.push(Hypha::new(pos, vel, Hypha::random_dna()));
            }
        }

        // Draw
        clear_background(BLACK);

        field.update_texture();
        field.draw();

        // Draw Hyphae Tips
        for hypha in &hyphae {
            // Color based on magnetism?
            let color = if hypha.magnetism < 0.5 {
                SKYBLUE
            } else {
                MAGENTA
            };
            draw_circle(hypha.pos.x, hypha.pos.y, 2.0, color);
        }

        draw_text(
            &format!("Hyphae: {}", hyphae.len()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, WHITE);

        next_frame().await;
    }
}
