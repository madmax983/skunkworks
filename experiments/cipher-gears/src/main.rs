use cipher_gears::{CipherMachine, Gear};
use macroquad::prelude::*;

fn draw_gear(gear: &Gear, x: f32, y: f32, color: Color) {
    let teeth = gear.teeth;
    let r = gear.radius;
    let angle_step = 2.0 * std::f32::consts::PI / teeth as f32;
    let tooth_depth = 10.0;
    let inner_r = r - tooth_depth;

    // Draw body
    draw_circle_lines(x, y, inner_r, 2.0, color);

    // Draw teeth
    for i in 0..teeth {
        let theta = gear.angle + i as f32 * angle_step;

        let p1 = vec2(
            x + inner_r * (theta - 0.1).cos(),
            y + inner_r * (theta - 0.1).sin(),
        );
        let p2 = vec2(x + r * (theta - 0.05).cos(), y + r * (theta - 0.05).sin());
        let p3 = vec2(x + r * (theta + 0.05).cos(), y + r * (theta + 0.05).sin());
        let p4 = vec2(
            x + inner_r * (theta + 0.1).cos(),
            y + inner_r * (theta + 0.1).sin(),
        );

        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, color);
        draw_line(p2.x, p2.y, p3.x, p3.y, 2.0, color);
        draw_line(p3.x, p3.y, p4.x, p4.y, 2.0, color);
    }

    // Draw orientation marker
    let marker_x = x + inner_r * gear.angle.cos();
    let marker_y = y + inner_r * gear.angle.sin();
    draw_line(x, y, marker_x, marker_y, 2.0, RED);

    // Label
    draw_text(&format!("{}t", teeth), x - 10.0, y, 20.0, color);
}

#[macroquad::main("Cipher Gears")]
async fn main() {
    let mut machine = CipherMachine::new();

    loop {
        clear_background(BLACK);

        draw_text("Cipher Gears", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Current Input: {}", machine.current_input_char),
            20.0,
            60.0,
            20.0,
            GRAY,
        );

        // Positions
        let input_pos = vec2(150.0, 300.0);
        let drive_pos = vec2(350.0, 300.0);

        // Key Rotor meshes with Drive Shaft
        // dist = r1 + r2
        let r_drive = machine.train.gears[machine.drive_shaft].radius;
        let r_key = machine.train.gears[machine.key_rotor].radius;
        let key_pos = vec2(drive_pos.x + r_drive + r_key + 5.0, 300.0);

        let output_pos = vec2(600.0, 300.0);

        // Draw connections (Shafts/Links)
        draw_line(
            input_pos.x,
            input_pos.y,
            output_pos.x,
            output_pos.y,
            5.0,
            DARKGRAY,
        );
        draw_line(
            key_pos.x,
            key_pos.y,
            output_pos.x,
            output_pos.y,
            5.0,
            DARKGRAY,
        );

        // Draw Gears
        draw_gear(
            &machine.train.gears[machine.input_handle],
            input_pos.x,
            input_pos.y,
            GREEN,
        );
        draw_text("Input", input_pos.x - 20.0, input_pos.y + 70.0, 20.0, GREEN);

        draw_gear(
            &machine.train.gears[machine.drive_shaft],
            drive_pos.x,
            drive_pos.y,
            BLUE,
        );
        draw_text("Time", drive_pos.x - 20.0, drive_pos.y + 70.0, 20.0, BLUE);

        draw_gear(
            &machine.train.gears[machine.key_rotor],
            key_pos.x,
            key_pos.y,
            YELLOW,
        );
        draw_text("Key", key_pos.x - 20.0, key_pos.y + 70.0, 20.0, YELLOW);

        draw_gear(
            &machine.train.gears[machine.diff_output],
            output_pos.x,
            output_pos.y,
            RED,
        );
        draw_text(
            "Output",
            output_pos.x - 20.0,
            output_pos.y + 70.0,
            20.0,
            RED,
        );

        // Handle Input
        // Read char from keyboard
        let c = get_char_pressed();
        if let Some(ch) = c {
            if ch.is_alphabetic() {
                let upper = ch.to_ascii_uppercase();
                let _encrypted = machine.encrypt(upper);
            }
        }

        // Display result (based on current angle of output)
        let angle = machine.train.gears[machine.diff_output].angle;
        let normalized = (angle % (2.0 * std::f32::consts::PI) + 2.0 * std::f32::consts::PI)
            % (2.0 * std::f32::consts::PI);
        let step = 2.0 * std::f32::consts::PI / 26.0;
        let idx = (normalized / step).round() as i32 % 26;
        let out_char = (b'A' + idx as u8) as char;

        draw_text(
            &format!("-> {}", out_char),
            output_pos.x - 20.0,
            output_pos.y - 70.0,
            40.0,
            RED,
        );

        draw_text(
            "Type A-Z to encrypt",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
