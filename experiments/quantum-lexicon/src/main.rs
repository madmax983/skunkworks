mod neuron;
mod phonology;
mod retina;
mod shader;

use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point, TilingConsts};
use retina::Retina;
use phonology::{Features, Phoneme};

const EYE_RES: usize = 128;

struct Particle {
    pos: Point,
    phoneme: Phoneme,
    vel: Point,
    stability: f32, // 0.0 = Chaos, 1.0 = Frozen
    target_phoneme: Option<Phoneme>, // If stabilizing, drift towards this
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
        'c' => Features { voice: 0.0, place: 0.8, manner: 0.4 }, // Hard C /k/ or Soft C /s/
        'h' => Features { voice: 0.0, place: 1.0, manner: 0.4 },
        'q' => Features { voice: 0.0, place: 0.9, manner: 0.0 },
        'x' => Features { voice: 0.0, place: 0.8, manner: 0.4 },
        'y' => Features { voice: 1.0, place: 0.2, manner: 0.9 },
        _ => Features { voice: 0.5, place: 0.5, manner: 0.5 },
    };
    Phoneme { features, symbol: c }
}

#[macroquad::main("Quantum Lexicon")]
async fn main() {
    let tiling = TilingConsts::new_4_5();
    let mut retina = Retina::new(EYE_RES, EYE_RES);
    let render_target = render_target(EYE_RES as u32, EYE_RES as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut player_pos = Point::new(0.0, 0.0);
    let mut particles = Vec::new();

    let words = vec!["chaos", "quantum", "lexicon", "eye", "drift", "void"];

    for (i, word) in words.iter().enumerate() {
        let angle = (i as f64 / words.len() as f64) * std::f64::consts::TAU;
        let r = 0.5;
        let center = Point::new(r * angle.cos(), r * angle.sin());

        for (j, c) in word.chars().enumerate() {
            // Offset slightly for each char
            let char_offset = Point::new(j as f64 * 0.05, 0.0);
            // This is Euclidean offset, effectively. In hyperbolic, we should use mobius, but for initialization this is fine.
            let pos = mobius_add(center, char_offset); // Rough approx

            particles.push(Particle {
                pos,
                phoneme: create_phoneme(c),
                vel: Point::new(rand::gen_range(-0.002, 0.002), rand::gen_range(-0.002, 0.002)),
                stability: 0.0,
                target_phoneme: Some(create_phoneme(c)),
            });
        }
    }

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
    ).unwrap();

    let start_time = get_time();
    let mut retina_input = vec![0.0; EYE_RES * EYE_RES];

    loop {
        // --- Input ---
        let speed = 0.02;
        let mut move_vec = Point::new(0.0, 0.0);
        if is_key_down(KeyCode::W) { move_vec.im += speed; }
        if is_key_down(KeyCode::S) { move_vec.im -= speed; }
        if is_key_down(KeyCode::A) { move_vec.re -= speed; }
        if is_key_down(KeyCode::D) { move_vec.re += speed; }
        if move_vec.norm() > 0.0 {
            player_pos = mobius_add(player_pos, move_vec);
        }

        // --- Render to Retina Target ---
        // We render the scene from the camera's perspective into the low-res eye buffer
        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        clear_background(BLACK);

        // Draw Particles for Retina (Just white dots)
        // Need to replicate the projection logic
        // Camera2D with render target changes the coordinate system.
        // If we use default Camera2D, 0,0 is center? No, -1,-1 to 1,1 usually.
        // Macroquad default camera is screen coordinates (0..width, 0..height).
        // Since render target is 128x128, coordinates are 0..128.

        let rt_center = vec2(EYE_RES as f32 / 2.0, EYE_RES as f32 / 2.0);
        let rt_scale = EYE_RES as f32 / 2.0;

        for p in &particles {
            let z_local = mobius_sub(p.pos, player_pos);
            if z_local.norm_sqr() < 0.99 {
                let sx = rt_center.x + z_local.re as f32 * rt_scale;
                let sy = rt_center.y - z_local.im as f32 * rt_scale;
                draw_circle(sx, sy, 3.0, WHITE);
            }
        }
        set_default_camera();

        // --- Process Retina ---
        let texture = &render_target.texture;
        let image = texture.get_texture_data();

        for (i, pixel) in image.bytes.chunks(4).enumerate() {
            if i < retina_input.len() {
                // Luminance
                retina_input[i] = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
            }
        }

        let spikes = retina.update(&retina_input);

        // --- Stability Field Calculation ---
        // Create a coarse stability map from spikes
        // Map spikes (x,y in 0..128) to screen space (0..screen_width)
        let sw = screen_width();
        let sh = screen_height();

        // Reset stability for all particles (decay)
        for p in &mut particles {
            p.stability *= 0.90; // Decay
        }

        // Apply spikes to particles
        for (sx, sy) in &spikes {
            // Convert spike to complex plane coordinates relative to screen center
            // Spike (0,0) is top-left of retina, which maps to top-left of screen.
            // Screen center is (sw/2, sh/2).
            // Normalized: sx/w, sy/h => 0..1
            // Complex plane: -1..1

            let nx = (*sx as f32 / EYE_RES as f32) * 2.0 - 1.0;
            let ny = -((*sy as f32 / EYE_RES as f32) * 2.0 - 1.0); // Flip Y

            // Check distance to particles (in screen space / local complex plane)
            let spike_pos = Point::new(nx as f64, ny as f64);

            for p in &mut particles {
                let z_local = mobius_sub(p.pos, player_pos);
                // Distance in projected space
                let dist_sq = (z_local.re - spike_pos.re).powi(2) + (z_local.im - spike_pos.im).powi(2);

                if dist_sq < 0.05 { // Influence radius
                    p.stability += 0.2;
                }
            }
        }

        // --- Particle Update ---
        for p in &mut particles {
            p.stability = p.stability.clamp(0.0, 1.0);

            if p.stability > 0.5 {
                // Stabilizing: Move towards target phoneme / initial position relative to word?
                // For now, just stop moving.
                // Or maybe revert to "correct" letter if we had one.
                if let Some(target) = &p.target_phoneme {
                    // Interpolate features towards target
                    p.phoneme.features.voice += (target.features.voice - p.phoneme.features.voice) * 0.1;
                    p.phoneme.features.place += (target.features.place - p.phoneme.features.place) * 0.1;
                    p.phoneme.features.manner += (target.features.manner - p.phoneme.features.manner) * 0.1;

                    // If close enough, snap symbol
                    if (p.phoneme.features.voice - target.features.voice).abs() < 0.1 {
                        p.phoneme.symbol = target.symbol;
                    }
                }
            } else {
                // Drifting: Brownian motion
                p.pos = mobius_add(p.pos, p.vel);

                // Mutation (Phonetic Drift)
                if rand::gen_range(0.0, 1.0) < 0.05 {
                    p.phoneme.features.voice = (p.phoneme.features.voice + rand::gen_range(-0.1, 0.1)).clamp(0.0, 1.0);
                    p.phoneme.features.place = (p.phoneme.features.place + rand::gen_range(-0.1, 0.1)).clamp(0.0, 1.0);
                    p.phoneme.features.manner = (p.phoneme.features.manner + rand::gen_range(-0.1, 0.1)).clamp(0.0, 1.0);

                    // Change symbol based on features (Reverse lookup would be hard, so just randomize char occasionally)
                    if rand::gen_range(0.0, 1.0) < 0.01 {
                         let chars = ['a','e','i','o','u','p','t','k','b','d','g','m','n','s','l','r','?','!'];
                         p.phoneme.symbol = chars[rand::gen_range(0, chars.len())];
                    }
                }
            }
        }

        // --- Render Main ---
        material.set_uniform("iResolution", (sw, sh, 0.0));
        material.set_uniform("iTime", (get_time() - start_time) as f32);
        material.set_uniform("u_player_pos", (player_pos.re as f32, player_pos.im as f32));
        material.set_uniform("u_neighbor_offset", tiling.neighbor_offset as f32);
        material.set_uniform("u_root_seed", 0.5);

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, sw, sh, WHITE);
        gl_use_default_material();

        // Draw Particles
        let min_dim = sw.min(sh);
        let center = vec2(sw / 2.0, sh / 2.0);
        let scale = min_dim / 2.0;

        for p in &particles {
            let z_local = mobius_sub(p.pos, player_pos);
            if z_local.norm_sqr() < 0.99 {
                let sx = center.x + z_local.re as f32 * scale;
                let sy = center.y - z_local.im as f32 * scale;

                let color = if p.stability > 0.5 {
                    GREEN // Stabilized
                } else {
                    Color::new(
                        p.phoneme.features.place,
                        p.phoneme.features.manner,
                        p.phoneme.features.voice,
                        1.0,
                    )
                };

                draw_text(&p.phoneme.symbol.to_string(), sx, sy, 20.0, color);
            }
        }

        // Draw Retina Overlay (Spikes)
        for (sx, sy) in &spikes {
            let x = (*sx as f32 / EYE_RES as f32) * sw;
            let y = (*sy as f32 / EYE_RES as f32) * sh;
            draw_rectangle(x, y, sw/EYE_RES as f32, sh/EYE_RES as f32, Color::new(1.0, 1.0, 1.0, 0.3));
        }

        draw_text("Quantum Lexicon", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Stability: {:.2}", particles.iter().map(|p| p.stability).sum::<f32>() / particles.len() as f32), 10.0, 50.0, 20.0, WHITE);
        draw_text("Look at the words to freeze them.", 10.0, sh - 20.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
