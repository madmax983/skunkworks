mod simulation;

use chrontext::blame::BlameAnalyzer;
use macroquad::prelude::*;
use simulation::{Target, World, WORLD_SIZE};
use std::path::Path;

/// 🧬 Lineage: experiments/chron-ddos
///
/// This experiment crosses the visual macroquad botnet swarm from `locust-ddos`
/// with the git blame chronological age parsing of `chrontext`.
///
/// Novel trait: Chronological Cyberwarfare. The botnet swarm targets lines of code
/// based on their chronological age, turning code text into multiple, dynamically
/// sized targets. Older code lines act as stronger attractors (larger radii or
/// higher priority) for the DDoS packets, while newly refactored sections are ignored.

#[macroquad::main("Chronological DDoS")]
async fn main() {
    let mut world = World::new();

    // Use chrontext to analyze this file itself
    let start_path = ".";
    let analyzer = BlameAnalyzer::new(start_path);
    let current_file = Path::new(file!());

    let lines_info = match analyzer.analyze(current_file) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to analyze file: {}", e);
            std::process::exit(1);
        }
    };

    let total_lines = lines_info.len() as f32;
    let target_spacing_y = WORLD_SIZE / total_lines;

    // Map LineInfo to World Targets
    for info in &lines_info {
        let y_pos = info.line_number as f32 * target_spacing_y;
        let x_pos = WORLD_SIZE / 2.0; // Center the targets

        // Age score: 0.0 (Oldest) to 1.0 (Newest)
        // Older code gets higher attractiveness and more health
        let attractiveness = 1.0 - info.age_score as f32;

        world.targets.push(Target {
            pos: vec2(x_pos, y_pos),
            attractiveness,
            health: 1000.0 * attractiveness,
            max_health: 1000.0 * attractiveness,
        });
    }

    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Input
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.add_firewall(world_mouse, 20.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Update
        world.update();

        // Render to buffer
        world.render_to_buffer(&mut image.bytes, width, height);
        texture.update(&image);

        // Draw
        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Targets based on age
        for target in &world.targets {
            if target.health > 0.0 {
                let tx = target.pos.x / WORLD_SIZE * screen_width();
                let ty = target.pos.y / WORLD_SIZE * screen_height();

                let health_pct = (target.health / target.max_health).clamp(0.0, 1.0);
                let radius = 5.0 + 10.0 * target.attractiveness * health_pct;

                let color = Color::new(1.0 - health_pct, 0.0, health_pct, 1.0); // Blue to Red

                draw_circle(tx, ty, radius, color);
            }
        }

        // Draw Firewalls overlay
        for (pos, radius) in &world.firewalls {
            let sx = pos.x / WORLD_SIZE * screen_width();
            let sy = pos.y / WORLD_SIZE * screen_height();
            let sr = radius / WORLD_SIZE * screen_width();
            draw_circle(sx, sy, sr, Color::new(1.0, 0.0, 0.0, 0.1));
        }

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Packets: {}", world.agents.len()),
            10.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Left Click: Deploy Firewall | C: Clear Rules",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
