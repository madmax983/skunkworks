use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point, TilingConsts};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod phonology;
mod shader;

use phonology::{Features, Phoneme};

// Generate a deterministic float seed from a path (used for shader background)
#[allow(dead_code)]
fn hash_path(path: &[u8]) -> f32 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let h = hasher.finish();
    (h as f64 / u64::MAX as f64) as f32
}

struct Particle {
    pos: Point,
    phoneme: Phoneme,
    vel: Point, // Tangent velocity
}

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
        'm' => Features { voice: 1.0, place: 0.0, manner: 0.2 },
        'n' => Features { voice: 1.0, place: 0.2, manner: 0.2 },
        's' => Features { voice: 0.0, place: 0.2, manner: 0.4 },
        'l' => Features { voice: 1.0, place: 0.3, manner: 0.6 },
        'r' => Features { voice: 1.0, place: 0.3, manner: 0.5 },
        _ => Features { voice: 0.5, place: 0.5, manner: 0.5 },
    };
    Phoneme { features, symbol: c }
}

#[macroquad::main("Hyperbolic Lexicon")]
async fn main() {
    let tiling = TilingConsts::new_4_5();

    // Camera State
    let mut player_pos = Point::new(0.0, 0.0);

    // Particles
    let mut particles = Vec::new();
    let word = "pater";
    for c in word.chars() {
        particles.push(Particle {
            pos: Point::new(rand::gen_range(-0.1, 0.1), rand::gen_range(-0.1, 0.1)),
            phoneme: create_phoneme(c),
            vel: Point::new(rand::gen_range(-0.001, 0.001), rand::gen_range(-0.001, 0.001)),
        });
    }

    // Shader Setup
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX,
            fragment: shader::FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float3),
                UniformDesc::new("iTime", UniformType::Float1),
                UniformDesc::new("u_player_pos", UniformType::Float2),
                UniformDesc::new("u_neighbor_offset", UniformType::Float1),
                UniformDesc::new("u_root_seed", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let start_time = get_time();

    loop {
        // --- Input & Camera Movement ---
        let speed = 0.02;
        let mut move_vec = Point::new(0.0, 0.0);
        if is_key_down(KeyCode::W) { move_vec.im += speed; }
        if is_key_down(KeyCode::S) { move_vec.im -= speed; }
        if is_key_down(KeyCode::A) { move_vec.re -= speed; }
        if is_key_down(KeyCode::D) { move_vec.re += speed; }

        if move_vec.norm() > 0.0 {
            player_pos = mobius_add(player_pos, move_vec);
        }

        // --- Particle Simulation ---
        if is_key_pressed(KeyCode::Space) {
            // Trigger Drift (Great Vowel Shift)
            for p in &mut particles {
                p.vel = Point::new(
                    rand::gen_range(-0.01, 0.01),
                    rand::gen_range(-0.01, 0.01),
                );
            }
        }

        for p in &mut particles {
            // Brownian motion / Drift
            p.pos = mobius_add(p.pos, p.vel);
            // Damping? No, let them drift forever to infinity.
            // But keep velocity small or they fly off too fast.
        }

        // --- Rendering ---

        // 1. Background Shader
        let root_seed = 0.5; // Constant for now, or could change based on depth if we tracked path
        material.set_uniform("iResolution", (screen_width(), screen_height(), 0.0));
        material.set_uniform("iTime", (get_time() - start_time) as f32);
        material.set_uniform("u_player_pos", (player_pos.re as f32, player_pos.im as f32));
        material.set_uniform("u_neighbor_offset", tiling.neighbor_offset as f32);
        material.set_uniform("u_root_seed", root_seed);

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        // 2. Render Particles
        // We need to transform WorldPos -> ScreenPos (Disk space)
        // ScreenPos (z_screen) = mobius_sub(WorldPos, PlayerPos)
        // Then map Disk space [-1, 1] to Pixels.

        let min_dim = screen_width().min(screen_height());
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let scale = min_dim / 2.0;

        for p in &particles {
            let z_local = mobius_sub(p.pos, player_pos);

            // Only draw if inside the disk (norm < 1.0)
            if z_local.norm_sqr() < 0.99 {
                let screen_x = center.x + z_local.re as f32 * scale;
                // Flip Y because GL coords vs Screen coords?
                // Macroquad Y is down. Complex plane Y is up.
                // So Im -> -Y.
                let screen_y = center.y - z_local.im as f32 * scale;

                let color = Color::new(
                    p.phoneme.features.place,
                    p.phoneme.features.manner,
                    p.phoneme.features.voice,
                    1.0,
                );

                draw_circle(screen_x, screen_y, 10.0, color);
                draw_text(&p.phoneme.symbol.to_string(), screen_x - 5.0, screen_y - 15.0, 20.0, WHITE);
            }
        }

        // UI
        draw_text("Hyperbolic Lexicon", 10.0, 30.0, 30.0, WHITE);
        draw_text("WASD: Move | SPACE: Drift", 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
