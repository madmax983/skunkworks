use macroquad::prelude::*;
use quipu_cradle::quipu::{Pendant, Knot, KnotType};
use quipu_cradle::serializer::to_quipu;
use serde::Serialize;

#[derive(Serialize)]
struct Census {
    village_id: u32,
    population: u32,
    corn_harvest: u32,
    potato_harvest: u32,
    llamas: u32,
    taxes: u32,
    details: Details,
}

#[derive(Serialize)]
struct Details {
    year: u32,
    scribe: u32,
}

#[macroquad::main("Quipu Cradle")]
async fn main() {
    let data = Census {
        village_id: 101,
        population: 452,
        corn_harvest: 1250,
        potato_harvest: 3000,
        llamas: 58,
        taxes: 25,
        details: Details {
            year: 1450,
            scribe: 7,
        },
    };

    let quipu = to_quipu(&data).unwrap();

    loop {
        clear_background(LIGHTGRAY);

        draw_text("Incan Quipu Data Serializer", 20.0, 30.0, 30.0, BLACK);
        draw_text("Hover over knots to read values", 20.0, 50.0, 20.0, DARKGRAY);

        let time = get_time() as f32;

        // Draw Main Cord
        let start_x = 50.0;
        let start_y = 100.0;
        let end_x = screen_width() - 50.0;

        draw_line(start_x, start_y, end_x, start_y, 4.0, BROWN);

        // Draw Pendants
        let num_pendants = quipu.pendants.len();
        let spacing = (end_x - start_x) / (num_pendants as f32 + 1.0);

        for (i, pendant) in quipu.pendants.iter().enumerate() {
            let x = start_x + (i as f32 + 1.0) * spacing;
            let y = start_y;

            // Sway effect
            let sway = (time * 2.0 + x * 0.1).sin() * 5.0; // Angle in degrees
            let sway_rad = sway.to_radians();

            draw_pendant(pendant, x, y, sway_rad, 0);
        }

        next_frame().await
    }
}

fn draw_pendant(pendant: &Pendant, start_x: f32, start_y: f32, angle: f32, depth: usize) {
    let length = 400.0 / (depth as f32 + 1.0); // Subsidiaries are shorter
    let end_x = start_x + angle.sin() * length;
    let end_y = start_y + angle.cos() * length;

    let color = Color::new(
        pendant.color[0] as f32 / 255.0,
        pendant.color[1] as f32 / 255.0,
        pendant.color[2] as f32 / 255.0,
        1.0
    );

    draw_line(start_x, start_y, end_x, end_y, 2.0, color);

    // Draw Knots
    // In Quipu, higher power is higher up.
    // My implementation:
    // - add_number iterates high power to low power.
    // - pushes to `knots` vector.
    // So `knots[0]` is highest power (top), `knots[last]` is lowest (bottom).
    // This matches visual order (top to bottom).

    let num_knots = pendant.knots.len();
    if num_knots > 0 {
        let step = length / (num_knots as f32 + 1.0);
        for (k_idx, knot) in pendant.knots.iter().enumerate() {
            let dist = (k_idx as f32 + 1.0) * step;
            let k_x = start_x + angle.sin() * dist;
            let k_y = start_y + angle.cos() * dist;

            draw_knot(knot, k_x, k_y, color);

            // Interaction
            let mouse_pos = mouse_position();
            if (mouse_pos.0 - k_x).abs() < 10.0 && (mouse_pos.1 - k_y).abs() < 10.0 {
                 draw_text(
                    &format!("Val: {}, Pow: 10^{}", knot.value, knot.power),
                    mouse_pos.0 + 10.0,
                    mouse_pos.1,
                    20.0,
                    BLACK
                );
            }
        }
    }

    // Draw Subsidiaries
    // They hang from the connection point (or somewhere along the string?)
    // Real Quipus: Subsidiaries hang from knots or specific points.
    // My serializer: `subsidiaries` vector.
    // Let's attach them at the bottom for now, or distribute them?
    // Distributing them along the parent cord is better.

    let num_subs = pendant.subsidiaries.len();
    if num_subs > 0 {
        let step = length / (num_subs as f32 + 1.0);
        for (s_idx, sub) in pendant.subsidiaries.iter().enumerate() {
            // Attach point
            let dist = (s_idx as f32 + 0.5) * step; // Shifted slightly so they don't overlap knots exactly
             let attach_x = start_x + angle.sin() * dist;
             let attach_y = start_y + angle.cos() * dist;

             // Subsidiary angle: slightly offset from parent
             let sub_angle = angle + 0.3; // Radian offset

             draw_pendant(sub, attach_x, attach_y, sub_angle, depth + 1);
        }
    }
}

fn draw_knot(knot: &Knot, x: f32, y: f32, color: Color) {
    match knot.knot_type {
        KnotType::Single => {
            draw_circle(x, y, 5.0, color);
            draw_circle_lines(x, y, 5.0, 2.0, BLACK);
        }
        KnotType::Long(turns) => {
            // Draw a longer shape (cylinder/rect)
            let h = 10.0 + (turns as f32) * 2.0;
            draw_rectangle(x - 4.0, y - h/2.0, 8.0, h, color);
            draw_rectangle_lines(x - 4.0, y - h/2.0, 8.0, h, 2.0, BLACK);
            // Draw turns lines
            for t in 0..turns {
                let ty = (y - h/2.0) + (t as f32) * 2.0 + 2.0;
                draw_line(x - 4.0, ty, x + 4.0, ty, 1.0, BLACK);
            }
        }
        KnotType::FigureEight => {
             draw_text("8", x - 5.0, y + 5.0, 20.0, BLACK);
        }
        KnotType::Empty => {}
    }
}
