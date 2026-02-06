mod parser;
mod origami;

use macroquad::prelude::*;
use macroquad::models::Vertex;
use parser::{parse_code, Scope};
use origami::OrigamiMesh;
use std::collections::HashMap;

const SAMPLE_CODE: &str = r#"fn main() {
    println!("Hello, World!");

    // A nested block
    {
        let x = 5;
        let y = 10;
        println!("Sum: {}", x + y);
    }
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    } else {
        let a = fibonacci(n - 1);
        let b = fibonacci(n - 2);
        return a + b;
    }
}

struct App {
    name: String,
}

impl App {
    fn new() -> Self {
        Self {
            name: "Syntax Fold".to_string(),
        }
    }

    fn run(&self) {
        loop {
            // Infinite loop
            break;
        }
    }
}
"#;

#[macroquad::main("Syntax Fold")]
async fn main() {
    let (lines, root_scope) = parse_code(SAMPLE_CODE);
    let origami = OrigamiMesh::new();

    let mut fold_states: HashMap<usize, f32> = HashMap::new();
    let mut target_fold_states: HashMap<usize, f32> = HashMap::new();

    // Initialize all potential scopes with 0.0
    init_fold_states(&root_scope, &mut fold_states);
    init_fold_states(&root_scope, &mut target_fold_states);

    let mut cam_yaw = 0.0f32;
    let mut cam_pitch = 0.0f32;
    let mut cam_dist = 40.0f32;

    loop {
        let dt = get_frame_time();

        // Update Animation
        let speed = 5.0;
        for (k, current) in fold_states.iter_mut() {
            if let Some(&target) = target_fold_states.get(k) {
                if (*current - target).abs() > 0.001 {
                    *current += (target - *current) * speed * dt;
                } else {
                    *current = target;
                }
            }
        }

        // Camera Input
        if is_mouse_button_down(MouseButton::Right) {
            cam_yaw += mouse_delta_position().x * 2.0;
            cam_pitch += mouse_delta_position().y * 2.0;
        }
        cam_dist = (cam_dist + mouse_wheel().1 * -1.0).clamp(10.0, 100.0);

        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        // 3D Render
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., -10., 0.),
            ..Default::default()
        });

        draw_grid(20, 1.0, BLACK, GRAY);

        // Calculate Transforms
        let transforms = origami.calculate_transforms(&lines, &root_scope, &fold_states);

        // Draw Paper Strips
        for (i, transform) in transforms.iter().enumerate() {
            let _line = &lines[i];

            // Transform points
            let w = 20.0;
            let h = 1.0;

            let p1 = transform.transform_point3(vec3(-w/2.0, 0.0, 0.0));
            let p2 = transform.transform_point3(vec3(w/2.0, 0.0, 0.0));
            let p3 = transform.transform_point3(vec3(w/2.0, -h, 0.0)); // Down is negative Y in local
            let p4 = transform.transform_point3(vec3(-w/2.0, -h, 0.0));

            let color = if i % 2 == 0 { Color::new(0.9, 0.9, 0.9, 1.0) } else { Color::new(0.85, 0.85, 0.85, 1.0) };

            // Draw Quad
            draw_quad_3d(p1, p2, p3, p4, color);

            // Draw Outline
            draw_line_3d(p1, p2, BLACK);
            draw_line_3d(p2, p3, BLACK);
            draw_line_3d(p3, p4, BLACK);
            draw_line_3d(p4, p1, BLACK);
        }

        set_default_camera();

        // 2D Overlay & Interaction
        let start_y = 50.0;
        let line_h = 18.0;
        let start_x = 20.0;

        draw_text("Syntax Fold ⚛️📄", 20.0, 30.0, 30.0, WHITE);
        draw_text("Left Click: Toggle Fold | Right Drag: Rotate Camera", 20.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        // Maekawa Check
        let is_flat = origami.check_foldability(&lines, &root_scope, &fold_states);
        let status = if is_flat { "PASS" } else { "FAIL" };
        let color = if is_flat { GREEN } else { RED };
        draw_text(&format!("Maekawa Check: {}", status), screen_width() - 250.0, 30.0, 20.0, color);

        let cp_info = origami.get_crease_pattern_info(&lines, &root_scope, &fold_states);
        for (i, (_line_idx, info)) in cp_info.iter().enumerate() {
            draw_text(info, screen_width() - 250.0, 60.0 + i as f32 * 20.0, 16.0, WHITE);
        }

        // Draw 2D Interactive List
        for (i, line) in lines.iter().enumerate() {
            let y = start_y + i as f32 * line_h;

            // Highlight if hovering
            let mouse_y = mouse_position().1;
            let is_hover = mouse_y >= y && mouse_y < y + line_h && mouse_position().0 < 400.0;

            let bg_color = if is_hover { Color::new(0.3, 0.3, 0.3, 0.5) } else { Color::new(0.0, 0.0, 0.0, 0.5) };
            draw_rectangle(start_x, y, 380.0, line_h, bg_color);

            let prefix = "  ".repeat(line.indent);
            draw_text(&format!("{}{}", prefix, line.content), start_x + 5.0, y + 14.0, 16.0, WHITE);

            // If this line starts a scope, show toggle
            if let Some(_scope) = find_scope_starting_at(&root_scope, i) {
                let state = target_fold_states.get(&i).copied().unwrap_or(0.0);
                let symbol = if state > 0.5 { "[+]" } else { "[-]" };
                draw_text(symbol, start_x + 350.0, y + 14.0, 16.0, YELLOW);

                if is_hover && is_mouse_button_pressed(MouseButton::Left) {
                    let new_target = if state > 0.5 { 0.0 } else { 1.0 };
                    target_fold_states.insert(i, new_target);
                }
            }
        }

        next_frame().await
    }
}

fn init_fold_states(scope: &Scope, states: &mut HashMap<usize, f32>) {
    states.insert(scope.start_line, 0.0);
    for child in &scope.children {
        init_fold_states(child, states);
    }
}

fn find_scope_starting_at<'a>(scope: &'a Scope, line_idx: usize) -> Option<&'a Scope> {
    if scope.start_line == line_idx && !scope.children.is_empty() {
         if scope.end_line > scope.start_line {
             return Some(scope);
         }
    }
    for child in &scope.children {
        if let Some(s) = find_scope_starting_at(child, line_idx) {
            return Some(s);
        }
    }
    None
}

fn draw_quad_3d(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3, color: Color) {
    let color: [u8; 4] = color.into();
    // Macroquad's Vertex normal is often Vec4 in newer versions? Or maybe not?
    // Error said expected Vec4.
    let normal = vec4(0.0, 0.0, 1.0, 0.0);
    let mesh = Mesh {
        vertices: vec![
            Vertex { position: p1, uv: Vec2::ZERO, color, normal },
            Vertex { position: p2, uv: Vec2::ZERO, color, normal },
            Vertex { position: p3, uv: Vec2::ZERO, color, normal },
            Vertex { position: p4, uv: Vec2::ZERO, color, normal },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: None,
    };
    draw_mesh(&mesh);
}
