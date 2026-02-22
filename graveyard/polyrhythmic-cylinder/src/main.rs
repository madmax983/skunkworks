use macroquad::prelude::*;
use polyrhythmic_cylinder::audio::{start_audio, AudioCommand};
use polyrhythmic_cylinder::mechanism::{Comb, Cylinder, PluckEvent};

const CYLINDER_RADIUS: f32 = 150.0;
const PIN_LENGTH: f32 = 20.0;
const TOOTH_WIDTH: f32 = 15.0;
const TOOTH_GAP: f32 = 5.0;

#[macroquad::main("Polyrhythmic Cylinder")]
async fn main() {
    // Audio
    let (audio_tx, _audio_handle) = start_audio();

    // Mechanism
    let mut cylinder = Cylinder::new(CYLINDER_RADIUS);

    // Add some random pins for testing procedural generation
    // We will use "Euclidean Rhythms" or just noise for now.
    // 3 tracks: Bass, Mid, High
    for i in 0..16 {
        // Track 0 (Low) - 4 beats
        if i % 4 == 0 {
            cylinder.add_pin(i as f32 * std::f32::consts::PI / 8.0, 0);
        }
        // Track 1 (Mid) - 3 against 4?
        if i % 3 == 0 {
            cylinder.add_pin(i as f32 * std::f32::consts::PI / 8.0, 1);
        }
        // Track 2 (High) - Random
        if rand::gen_range(0, 10) < 3 {
            cylinder.add_pin(i as f32 * std::f32::consts::PI / 8.0, 2);
        }
    }

    cylinder.angular_velocity = 1.0; // Slow rotation

    let mut comb = Comb::new(3);
    // Tune comb
    comb.teeth[0].frequency = 110.0; // A2
    comb.teeth[1].frequency = 164.81; // E3
    comb.teeth[2].frequency = 220.0; // A3

    loop {
        clear_background(BLACK);

        // Physics
        let dt = get_frame_time();

        // Input control
        if is_key_down(KeyCode::Space) || is_key_down(KeyCode::W) {
            cylinder.wind(dt * 0.5); // Wind up
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            cylinder.angular_velocity *= 0.9; // Brake
        }

        let events = cylinder.tick(dt, &mut comb);

        // Audio Trigger
        for event in events {
            let _ = audio_tx.send(AudioCommand::Pluck {
                frequency: event.frequency,
                volume: event.volume,
            });
        }

        // Draw
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        // Draw Cylinder (Circle)
        draw_circle_lines(cx, cy, cylinder.radius, 2.0, WHITE);

        // Draw Pins
        // We draw them relative to current angle
        for pin in &cylinder.pins {
            let angle = cylinder.angle + pin.angle;
            let x = cx + angle.cos() * cylinder.radius;
            let y = cy + angle.sin() * cylinder.radius;

            // Pin stick out
            let px = cx + angle.cos() * (cylinder.radius + PIN_LENGTH);
            let py = cy + angle.sin() * (cylinder.radius + PIN_LENGTH);

            // Color based on track
            let color = match pin.track_index {
                0 => RED,
                1 => GREEN,
                2 => BLUE,
                _ => WHITE,
            };

            draw_line(x, y, px, py, 2.0, color);
        }

        // Draw Comb (Bottom)
        // We need to position teeth where the pins hit.
        // Assuming pins hit at Angle 0 (Right side? Or Bottom?)
        // In mechanism logic, we check for 0 crossing.
        // 0 is typically Right in standard trig (cos=1, sin=0).
        // So let's draw the comb on the right side.

        let comb_x = cx + cylinder.radius + PIN_LENGTH + 10.0;
        let comb_y_start = cy - (3.0 * (TOOTH_WIDTH + TOOTH_GAP)) / 2.0;

        for (i, tooth) in comb.teeth.iter().enumerate() {
            let ty = comb_y_start + i as f32 * (TOOTH_WIDTH + TOOTH_GAP);

            // Vibration effect
            let wiggle = (get_time() * 50.0).sin() as f32 * 5.0 * tooth.vibration_amplitude;

            let color = match tooth.track_index {
                0 => RED,
                1 => GREEN,
                2 => BLUE,
                _ => WHITE,
            };

            // Draw Tooth
            draw_rectangle(comb_x + wiggle, ty, 40.0, TOOTH_WIDTH, color);

            // Draw String/Tine visual
            draw_line(
                comb_x + 40.0,
                ty + TOOTH_WIDTH / 2.0,
                comb_x + 100.0,
                ty + TOOTH_WIDTH / 2.0,
                2.0,
                GRAY,
            );
        }

        // UI
        draw_text("Polyrhythmic Cylinder", 20.0, 20.0, 30.0, WHITE);
        draw_text("Hold SPACE/W to Wind", 20.0, 50.0, 20.0, GRAY);
        draw_text(
            &format!(
                "RPM: {:.2}",
                cylinder.angular_velocity * 60.0 / (2.0 * std::f32::consts::PI)
            ),
            20.0,
            70.0,
            20.0,
            YELLOW,
        );

        // Tension Bar
        draw_text("Mainspring Tension:", 20.0, 100.0, 20.0, WHITE);
        draw_rectangle(20.0, 110.0, 200.0, 20.0, GRAY);
        draw_rectangle(
            20.0,
            110.0,
            200.0 * cylinder.mainspring.tension,
            20.0,
            match cylinder.mainspring.tension {
                t if t > 0.8 => RED,
                t if t > 0.4 => YELLOW,
                _ => GREEN,
            },
        );

        next_frame().await
    }
}
