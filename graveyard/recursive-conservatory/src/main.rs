use macroquad::prelude::*;

mod grammar;
mod turtle;

use grammar::LSystem;
use turtle::{Action, Turtle};

struct Preset {
    name: String,
    axiom: String,
    rules: Vec<(char, &'static str)>,
    angle: f32,
    start_angle: f32,
}

struct RenderData {
    actions: Vec<(Action, usize)>, // Action + Stack Depth
    max_step: usize,
    max_depth: usize,
}

impl RenderData {
    fn new(preset: &Preset, iterations: u32) -> Self {
        let system = LSystem::new(&preset.axiom, preset.rules.clone());
        let expanded = system.expand(iterations);

        let mut turtle = Turtle::new(
            Vec2::ZERO,
            preset.start_angle.to_radians(),
            1.0,
            preset.angle.to_radians(),
        );
        let mut actions = Vec::new();
        let mut max_depth = 0;
        let mut current_depth = 0;

        for c in expanded.chars() {
            let action = turtle.interpret(c);
            match action {
                Action::Push(d) => {
                    current_depth = d;
                    if d > max_depth {
                        max_depth = d;
                    }
                }
                Action::Pop(d) => {
                    current_depth = d;
                }
                _ => {}
            }
            actions.push((action, current_depth));
        }

        let max_step = actions.len();
        Self {
            actions,
            max_step,
            max_depth,
        }
    }
}

#[macroquad::main("Recursive Conservatory")]
async fn main() {
    let presets = [
        Preset {
            name: "Fractal Plant".to_string(),
            axiom: "X".to_string(),
            rules: vec![('X', "F+[[X]-X]-F[-FX]+X"), ('F', "FF")],
            angle: 25.0,
            start_angle: -90.0, // Upwards
        },
        Preset {
            name: "Dragon Curve".to_string(),
            axiom: "FX".to_string(),
            rules: vec![('X', "X+YF+"), ('Y', "-FX-Y")],
            angle: 90.0,
            start_angle: 0.0,
        },
        Preset {
            name: "Sierpinski Triangle".to_string(),
            axiom: "F-G-G".to_string(),
            rules: vec![('F', "F-G+F+G-F"), ('G', "GG")],
            angle: 120.0,
            start_angle: 0.0,
        },
        Preset {
            name: "Binary Tree".to_string(),
            axiom: "G".to_string(),
            rules: vec![('G', "F[-G]+G"), ('F', "FF")],
            angle: 45.0,
            start_angle: -90.0,
        },
    ];

    let mut current_preset_index = 0;
    let mut iterations = 4;
    let mut data = RenderData::new(&presets[current_preset_index], iterations);

    let mut cam = Camera2D {
        zoom: vec2(0.01, 0.01),
        target: vec2(0.0, 0.0),
        ..Default::default()
    };

    let mut current_step = 0;
    let mut playing = true;
    let mut speed = 10;

    loop {
        // Input Handling
        if is_key_pressed(KeyCode::Space) {
            playing = !playing;
        }
        if is_key_pressed(KeyCode::R) {
            current_step = 0;
        }
        if is_key_pressed(KeyCode::Tab) {
            current_preset_index = (current_preset_index + 1) % presets.len();
            current_step = 0;
            data = RenderData::new(&presets[current_preset_index], iterations);
        }
        if is_key_pressed(KeyCode::Up) {
            iterations += 1;
            current_step = 0;
            data = RenderData::new(&presets[current_preset_index], iterations);
        }
        if is_key_pressed(KeyCode::Down) {
            iterations = iterations.saturating_sub(1);
            current_step = 0;
            data = RenderData::new(&presets[current_preset_index], iterations);
        }
        if is_key_pressed(KeyCode::Right) {
            speed = (speed * 2).min(1000);
        }
        if is_key_pressed(KeyCode::Left) {
            speed = (speed / 2).max(1);
        }

        // Camera Controls
        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            cam.target.x -= delta.x / cam.zoom.x;
            cam.target.y += delta.y / cam.zoom.y;
        }
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            cam.zoom *= if wheel > 0.0 { 1.1 } else { 0.9 };
        }

        // Logic
        if playing {
            current_step = (current_step + speed).min(data.max_step);
        }

        // Rendering
        clear_background(BLACK);
        set_camera(&cam);

        // Draw Ground Line
        draw_line(-1000.0, 0.0, 1000.0, 0.0, 0.1, DARKGRAY);

        // Draw Tree & Roots
        // We iterate only up to current_step
        for i in 0..current_step {
            let (action, depth) = &data.actions[i];

            // Tree
            if let Action::Move(start, end) = action {
                // Color based on depth?
                let color = Color::new(0.2, 0.8 + (*depth as f32 * 0.05).sin() * 0.2, 0.2, 1.0);
                draw_line(start.x, start.y, end.x, end.y, 0.1, color);
            }

            // Roots (Flame Graph)
            let root_x = i as f32 * 0.1; // 0.1 width per step
            let root_y = *depth as f32 * 1.0; // 1.0 height per depth

            let mut root_color = Color::new(0.8, 0.4 + (*depth as f32 * 0.1).sin() * 0.4, 0.2, 0.5);
            match action {
                Action::Push(_) => root_color = RED,
                Action::Pop(_) => root_color = BLUE,
                Action::Turn(_) => root_color = GREEN,
                Action::Move(_, _) => {} // Keep depth color
                _ => root_color.a = 0.1, // Dim unknown actions
            }
            root_color.a = 0.5; // Translucency
            draw_rectangle(root_x, root_y, 0.1, 1.0, root_color);
        }

        // Draw "Cursor" connecting Tree to Root
        if current_step > 0 && current_step < data.max_step {
            let (action, depth) = &data.actions[current_step - 1];
            let root_x = (current_step - 1) as f32 * 0.1;
            if let Action::Move(_, end) = action {
                draw_line(end.x, end.y, root_x, *depth as f32, 0.05, YELLOW);
            }
        }

        set_default_camera();

        // UI
        draw_text(
            &format!("Preset: {} (Tab)", presets[current_preset_index].name),
            20.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Iterations: {} (Up/Down)", iterations),
            20.0,
            50.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Step: {} / {} (Space/R)", current_step, data.max_step),
            20.0,
            80.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Speed: {} (Left/Right)", speed),
            20.0,
            110.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Max Depth: {}", data.max_depth),
            20.0,
            140.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}
