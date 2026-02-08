use macroquad::prelude::*;
use phonology::{Phoneme, Features};
use physics::{World};

mod phonology;
mod physics;

fn create_phoneme(c: char) -> Phoneme {
    let features = match c {
        'a' => Features { voice: 1.0, place: 0.5, manner: 1.0 },
        'e' => Features { voice: 1.0, place: 0.3, manner: 0.8 },
        'i' => Features { voice: 1.0, place: 0.2, manner: 0.9 },
        'o' => Features { voice: 1.0, place: 0.7, manner: 0.8 },
        'u' => Features { voice: 1.0, place: 0.8, manner: 0.9 },
        'p' => Features { voice: 0.0, place: 0.0, manner: 0.0 },
        't' => Features { voice: 0.0, place: 0.2, manner: 0.0 },
        'k' => Features { voice: 0.0, place: 0.8, manner: 0.0 },
        'b' => Features { voice: 1.0, place: 0.0, manner: 0.0 },
        'd' => Features { voice: 1.0, place: 0.2, manner: 0.0 },
        'g' => Features { voice: 1.0, place: 0.8, manner: 0.0 },
        'f' => Features { voice: 0.0, place: 0.1, manner: 0.4 },
        's' => Features { voice: 0.0, place: 0.2, manner: 0.4 },
        'h' => Features { voice: 0.0, place: 0.9, manner: 0.5 },
        'm' => Features { voice: 1.0, place: 0.0, manner: 0.2 }, // nasal as low manner?
        'n' => Features { voice: 1.0, place: 0.2, manner: 0.2 },
        'l' => Features { voice: 1.0, place: 0.3, manner: 0.6 },
        'r' => Features { voice: 1.0, place: 0.3, manner: 0.5 },
        _ => Features { voice: 0.5, place: 0.5, manner: 0.5 },
    };
    Phoneme { features, symbol: c }
}

#[macroquad::main("Morph Physics")]
async fn main() {
    let mut world = World::new();

    // Initial Word: "pater"
    let word_str = "pater";
    let start_x = 300.0;
    let start_y = 300.0;

    let mut last_idx = None;

    for (i, c) in word_str.chars().enumerate() {
        let p = create_phoneme(c);
        let pos = Vec2::new(start_x + i as f32 * 60.0, start_y);
        world.add_particle(p, pos);

        let current_idx = world.particles.len() - 1;
        if let Some(prev) = last_idx {
            world.add_spring(prev, current_idx, 60.0);
        }
        last_idx = Some(current_idx);
    }

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Space) {
             // Heat: random velocity
             for p in &mut world.particles {
                 let r1 = rand::gen_range(-10.0, 10.0);
                 let r2 = rand::gen_range(-10.0, 10.0);
                 p.vel += Vec2::new(r1, r2);

                 // Lenition: High velocity increases manner (becomes more vowel-like)
                 if p.vel.length() > 5.0 {
                     p.phoneme.features.manner = (p.phoneme.features.manner + 0.005).min(1.0);
                 }
             }
        }

        if is_key_pressed(KeyCode::R) {
            // Reset logic could go here, for now just spawn 'mater'
            let word_str = "mater";
            let start_x = rand::gen_range(100.0, 500.0);
            let start_y = rand::gen_range(100.0, 500.0);
            let mut last = None;
            for (i, c) in word_str.chars().enumerate() {
                let p = create_phoneme(c);
                let pos = Vec2::new(start_x + i as f32 * 60.0, start_y);
                world.add_particle(p, pos);
                let cur = world.particles.len() - 1;
                if let Some(prev) = last {
                     world.add_spring(prev, cur, 60.0);
                }
                last = Some(cur);
            }
        }

        world.update(get_frame_time());

        // Draw Springs
        for &(i, j, _) in &world.springs {
             if i < world.particles.len() && j < world.particles.len() {
                draw_line(
                    world.particles[i].pos.x, world.particles[i].pos.y,
                    world.particles[j].pos.x, world.particles[j].pos.y,
                    2.0, GRAY
                );
             }
        }

        // Draw Particles
        for p in &world.particles {
            // Color mapping:
            // R = Place (0=Labial, 1=Glottal)
            // G = Manner (0=Stop, 1=Vowel)
            // B = Voice (0=Unvoiced, 1=Voiced)
            let color = Color::new(
                p.phoneme.features.place,
                p.phoneme.features.manner,
                p.phoneme.features.voice,
                1.0
            );
            draw_circle(p.pos.x, p.pos.y, 20.0, color);

            // Draw Symbol
            // Use a simple offset
            draw_text(&p.phoneme.symbol.to_string(), p.pos.x - 8.0, p.pos.y + 8.0, 30.0, WHITE);

            // Draw Feature bars
            draw_rectangle(p.pos.x - 20.0, p.pos.y + 25.0, 40.0 * p.phoneme.features.manner, 5.0, GREEN);
        }

        draw_text("Morph Physics: [SPACE] Heat (Lenition) | [R] Spawn 'mater'", 10.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
