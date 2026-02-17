use macroquad::prelude::*;

mod audio;
mod luthier;

use audio::{init_audio, AudioCommand};
use luthier::LuthierString;

const STRING_COUNT: usize = 12;
const STRING_SPACING: f32 = 60.0;
const BASE_FREQ: f32 = 220.0; // A3

#[macroquad::main("Genetic Luthier")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let mut strings: Vec<LuthierString> = Vec::new();

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 50.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        // Start with random frequencies around a scale (target is just a guideline for mutation range)
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(LuthierString::new(pos, 400.0, target_freq));
    }

    let mut evolution_timer = 0.0;
    let evolution_interval = 0.1; // Evolve every 100ms

    let (mx, my) = mouse_position();
    let mut prev_mouse = vec2(mx, my);

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        evolution_timer += dt;

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_delta = mouse_pos - prev_mouse;
        let mouse_speed = mouse_delta.length();

        // 1. Physics & Input
        for i in 0..strings.len() {
            let s = &mut strings[i];
            s.update_physics(dt);

            // Check pluck
            let string_x = s.pos.x + s.vibration;
            let crossed = (prev_mouse.x < string_x && mouse_pos.x >= string_x)
                || (prev_mouse.x > string_x && mouse_pos.x <= string_x);
            let in_range = mouse_pos.y >= s.pos.y && mouse_pos.y <= s.pos.y + s.length;

            if crossed && in_range {
                let strength = mouse_speed.clamp(5.0, 50.0);
                let direction = if mouse_delta.x > 0.0 { 1.0 } else { -1.0 };
                s.pluck(strength * direction);

                // Play Audio
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: (strength / 50.0).clamp(0.1, 0.8),
                });
            }
        }

        // 2. Evolution
        if evolution_timer > evolution_interval {
            evolution_timer = 0.0;

            // Calculate Fitness
            let mut fitnesses = vec![0.0; STRING_COUNT];

            for i in 0..STRING_COUNT {
                let f1 = strings[i].frequency;
                let mut score = 0.0;
                let mut neighbors = 0;

                if i > 0 {
                    score += calculate_consonance(f1, strings[i-1].frequency);
                    neighbors += 1;
                }
                if i < STRING_COUNT - 1 {
                    score += calculate_consonance(f1, strings[i+1].frequency);
                    neighbors += 1;
                }

                if neighbors > 0 {
                    fitnesses[i] = score / neighbors as f32;
                } else {
                    fitnesses[i] = 0.5; // Default for single string?
                }
            }

            // Apply Evolution
            for (i, s) in strings.iter_mut().enumerate() {
                s.fitness = fitnesses[i];
                // Base freq for mutation guidance is the initial target freq
                let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
                s.evolve(target_freq);
            }
        }

        // 3. Draw
        for s in &strings {
            s.draw();
        }

        // UI
        draw_text("Genetic Luthier", 10.0, 30.0, 30.0, WHITE);
        draw_text("Strings evolve to harmonize with neighbors.", 10.0, 50.0, 20.0, GRAY);
        draw_text("Mouse: Pluck strings", 10.0, 70.0, 20.0, GRAY);

        prev_mouse = mouse_pos;
        next_frame().await;
    }
}

fn calculate_consonance(f1: f32, f2: f32) -> f32 {
    let ratio = if f1 > f2 { f1 / f2 } else { f2 / f1 };

    // Standard harmonic intervals
    let intervals = [
        1.0,      // Unison
        2.0,      // Octave
        1.5,      // Perfect Fifth
        1.3333,   // Perfect Fourth
        1.25,     // Major Third
        1.2,      // Minor Third
        1.6666,   // Major Sixth
        1.6,      // Minor Sixth
    ];

    for &target in intervals.iter() {
        let diff = (ratio - target).abs();
        if diff < 0.05 {
            return 1.0 - (diff * 20.0); // 1.0 at match, 0.0 at 0.05 deviation
        }
    }

    0.0
}
