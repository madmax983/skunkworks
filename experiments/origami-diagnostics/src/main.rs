mod diagnostics;
mod origami;

use macroquad::prelude::*;
use origami::PaperModel;
use diagnostics::{Diagnostic, DiagnosticSpan};

fn window_conf() -> Conf {
    Conf {
        window_title: "Origami Diagnostics".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let source_code = include_str!("main.rs");

    // Mock diagnostics for now
    let diagnostics = vec![
        Diagnostic {
            message: "Mock error: Origami overflow".to_string(),
            code: None,
            level: "error".to_string(),
            spans: vec![
                DiagnosticSpan {
                    file_name: "src/main.rs".to_string(),
                    line_start: 10,
                    line_end: 12,
                    column_start: 0,
                    column_end: 0,
                    is_primary: true,
                    text: vec![],
                    label: Some("It folds too much!".to_string()),
                }
            ],
        }
    ];

    let mut paper = PaperModel::new(source_code, &diagnostics);

    // Create render target for text
    let texture_width = 800.0;
    let texture_height = paper.lines.len() as f32 * 20.0; // 20px per line
    let render_target = render_target(texture_width as u32, texture_height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    // Draw code to texture once
    set_camera(&Camera2D {
        zoom: vec2(1.0 / (texture_width / 2.0), -1.0 / (texture_height / 2.0)), // Flip Y
        target: vec2(texture_width / 2.0, texture_height / 2.0),
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    clear_background(WHITE);
    for (i, line) in paper.lines.iter().enumerate() {
        let y = i as f32 * 20.0 + 15.0; // Baseline
        let color = if line.is_error { RED } else { BLACK };

        // Highlight background for errors
        if line.is_error {
             draw_rectangle(0.0, i as f32 * 20.0, texture_width, 20.0, Color::new(1.0, 0.8, 0.8, 1.0));
        } else if !line.is_folded {
             // Context background
             draw_rectangle(0.0, i as f32 * 20.0, texture_width, 20.0, Color::new(0.95, 0.95, 0.95, 1.0));
        }

        draw_text(&line.content, 10.0, y, 20.0, color);
    }

    // Reset camera to default
    set_default_camera();

    paper.mesh.texture = Some(render_target.texture.clone());

    let mut fold_t: f32 = 0.0; // 0.0 = Flat, 1.0 = Folded
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.0;
    let mut cam_dist: f32 = 40.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        // Input
        if is_key_down(KeyCode::Right) {
            fold_t += 0.02;
        }
        if is_key_down(KeyCode::Left) {
            fold_t -= 0.02;
        }

        // Mouse drag for camera
        let mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
             let dx = mouse_pos.0 - last_mouse_pos.0;
             let dy = mouse_pos.1 - last_mouse_pos.1;
             cam_yaw -= dx * 0.01;
             cam_pitch += dy * 0.01;
             cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        // Scroll for zoom
        cam_dist -= mouse_wheel().1 * 2.0;
        cam_dist = cam_dist.clamp(5.0, 100.0);

        last_mouse_pos = mouse_pos;
        fold_t = fold_t.clamp(0.0, 1.0);

        // Update model
        paper.update_mesh(fold_t);

        // Render
        clear_background(GRAY);

        // 3D Camera
        let cam_pos = vec3(
            cam_yaw.sin() * cam_dist * cam_pitch.cos(),
            cam_pitch.sin() * cam_dist,
            cam_yaw.cos() * cam_dist * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, -paper.total_height / 2.0, 0.0),
            ..Default::default()
        });

        draw_grid(20, 1.0, BLACK, GRAY);

        // Draw the paper mesh
        draw_mesh(&paper.mesh);

        set_default_camera();

        // UI
        draw_text(&format!("Fold (Left/Right): {:.2}", fold_t), 10.0, 20.0, 30.0, BLACK);
        draw_text("Drag: Orbit | Scroll: Zoom", 10.0, 50.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
