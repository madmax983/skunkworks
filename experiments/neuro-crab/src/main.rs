use macroquad::prelude::*;

mod neuron;
mod network;
mod crab;

use crab::Crab;

#[macroquad::main("Neuro Crab")]
async fn main() {
    let mut crab = Crab::new();
    let history_len = 300;
    let mut spike_history: Vec<Vec<bool>> = vec![vec![false; 12]; history_len];

    loop {
        clear_background(BLACK);

        let mouse_pos = mouse_position();
        let drive = (mouse_pos.0 / screen_width()) * 20.0; // 0 to 20

        // Update physics (2 steps per frame for speed)
        for _ in 0..2 {
            crab.update(drive);

            // Record spikes
            let current_spikes: Vec<bool> = (0..12).map(|i| crab.network.is_spiking(i)).collect();
            spike_history.remove(0);
            spike_history.push(current_spikes);
        }

        // Draw Crab
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0 - 50.0);
        let body_radius = 40.0;

        // Draw body
        draw_circle(center.x, center.y, body_radius, RED);

        for leg in &crab.legs {
            // Leg base position
            let attach_pos = center + vec2(leg.base_angle.cos(), leg.base_angle.sin()) * body_radius;

            // Leg segments
            // Thigh (short)
            let thigh_len = 40.0;
            // Angle logic: swing around base angle
            let thigh_angle = leg.angle;
            let knee_pos = attach_pos + vec2(thigh_angle.cos(), thigh_angle.sin()) * thigh_len;

            // Shin (long) - extend same direction
            let shin_len = 60.0;
            // Add slight curve or offset? Nah, straight leg for now.
            let foot_pos = knee_pos + vec2(thigh_angle.cos(), thigh_angle.sin()) * shin_len;

            draw_line(attach_pos.x, attach_pos.y, knee_pos.x, knee_pos.y, 8.0, DARKGREEN);
            draw_line(knee_pos.x, knee_pos.y, foot_pos.x, foot_pos.y, 4.0, GREEN);

            draw_circle(knee_pos.x, knee_pos.y, 4.0, YELLOW);
            draw_circle(foot_pos.x, foot_pos.y, 3.0, BLUE);
        }

        // Draw Raster Plot
        let raster_y = screen_height() - 150.0;
        let raster_h = 140.0;
        let cell_w = screen_width() / history_len as f32;
        let cell_h = raster_h / 12.0;

        // Draw background for raster
        draw_rectangle(0.0, raster_y, screen_width(), raster_h, Color::new(0.1, 0.1, 0.1, 1.0));

        for (t, spikes) in spike_history.iter().enumerate() {
            let x = t as f32 * cell_w;
            for (n, &spiked) in spikes.iter().enumerate() {
                if spiked {
                    let y = raster_y + n as f32 * cell_h;
                    // Color based on Flexor/Extensor
                    // Flexors (Even) = Green, Extensors (Odd) = Red
                    let color = if n % 2 == 0 { GREEN } else { RED };
                    draw_rectangle(x, y, cell_w, cell_h, color);
                }
            }
        }

        // Labels for raster
        draw_text("L1 F", 5.0, raster_y + cell_h * 0.8, 10.0, WHITE);
        draw_text("L1 E", 5.0, raster_y + cell_h * 1.8, 10.0, WHITE);

        // UI
        draw_text(&format!("Drive Current: {:.1}", drive), 10.0, 30.0, 30.0, WHITE);
        draw_text("Move mouse X to control speed", 10.0, 60.0, 20.0, GRAY);

        next_frame().await
    }
}
