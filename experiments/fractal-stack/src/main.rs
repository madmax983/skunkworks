use macroquad::prelude::*;
use crate::lsystem::{LSystem, Operation};

mod lsystem;

struct TurtleState {
    position: Vec3,
    rotation: Quat,
    thickness: f32,
    color: Color,
    depth: u32,
}

#[macroquad::main("Fractal Stack")]
async fn main() {
    // Seed the garden
    // Plant 1: A tree-like structure
    // Axiom: F
    // Rule: F -> F[+F]F[-F][F]
    // Or a classic 3D tree
    let mut plant = LSystem::new("F", std::f32::consts::PI / 7.0, 1.0);
    // 3D Tree rule
    // F -> F[&+F]F[->F][&F]
    // Let's try a simple one first
    plant.add_rule('F', "F[&+F]F[->F][&F]");

    let mut iterations = 0;
    let max_iterations = 4;
    let mut operations = plant.expand(iterations);

    let mut cam = Camera3D {
        position: vec3(0., 10., 30.),
        target: vec3(0., 10., 0.),
        up: vec3(0., 1., 0.),
        fovy: 45.,
        projection: Projection::Perspective,
        ..Default::default()
    };

    let mut draw_limit = operations.len();
    let mut animate = false;
    let mut animation_speed = 1; // Operations per frame

    loop {
        clear_background(BLACK);

        // Input handling
        if is_key_pressed(KeyCode::Space) {
            iterations = (iterations + 1).min(max_iterations);
            operations = plant.expand(iterations);
            draw_limit = 0; // Reset animation
            animate = true;
            println!("Growing to iteration {}", iterations);
        }
        if is_key_pressed(KeyCode::R) {
            iterations = 0;
            operations = plant.expand(iterations);
            draw_limit = operations.len();
            animate = false;
        }
        if is_key_pressed(KeyCode::Enter) {
            animate = !animate;
        }
        if is_key_pressed(KeyCode::Right) {
             draw_limit = (draw_limit + 1).min(operations.len());
        }
        if is_key_pressed(KeyCode::Left) {
             draw_limit = draw_limit.saturating_sub(1);
        }

        if animate && draw_limit < operations.len() {
            draw_limit = (draw_limit + animation_speed).min(operations.len());
        } else if animate && draw_limit == operations.len() {
             animate = false;
        }

        // Camera Orbit
        let time = get_time();
        let radius = 40.0;
        cam.position = vec3(
            (time * 0.2).cos() as f32 * radius,
            20.0 + (time * 0.1).sin() as f32 * 10.0,
            (time * 0.2).sin() as f32 * radius,
        );
        cam.target = vec3(0., 10., 0.);

        set_camera(&cam);

        draw_grid(20, 1., GRAY, GRAY);

        // Draw the plant
        let mut stack: Vec<TurtleState> = Vec::new();
        let mut turtle = TurtleState {
            position: vec3(0., 0., 0.),
            rotation: Quat::IDENTITY,
            thickness: 0.5,
            color: GREEN,
            depth: 0,
        };

        // Base trunk color
        let start_color = BROWN;
        let end_color = GREEN;

        for (i, op) in operations.iter().enumerate() {
            if i >= draw_limit {
                // Draw cursor at current position
                draw_sphere(turtle.position, 0.5, None, WHITE);
                break;
            }

            match op {
                Operation::DrawForward(dist) => {
                    let forward = turtle.rotation * Vec3::Y;
                    let end_pos = turtle.position + forward * *dist;

                    // Color gradient based on depth/stack
                    let t = turtle.depth as f32 / (iterations as f32 * 2.0 + 1.0);
                    let color = Color::new(
                        start_color.r + (end_color.r - start_color.r) * t,
                        start_color.g + (end_color.g - start_color.g) * t,
                        start_color.b + (end_color.b - start_color.b) * t,
                        1.0
                    );

                    draw_line_3d(turtle.position, end_pos, color);
                    turtle.position = end_pos;
                }
                Operation::MoveForward(dist) => {
                    let forward = turtle.rotation * Vec3::Y;
                    turtle.position += forward * *dist;
                }
                Operation::Turn(yaw, pitch, roll) => {
                    let rot = Quat::from_euler(macroquad::math::EulerRot::YXZ, *yaw, *pitch, *roll);
                    turtle.rotation = turtle.rotation * rot;
                }
                Operation::PushStack => {
                    stack.push(TurtleState {
                        position: turtle.position,
                        rotation: turtle.rotation,
                        thickness: turtle.thickness * 0.8,
                        color: turtle.color,
                        depth: turtle.depth + 1,
                    });
                    turtle.thickness *= 0.8;
                    turtle.depth += 1;

                    // Visualize the "Joint" or "Stack Frame"
                    draw_sphere(turtle.position, 0.2, None, RED);
                }
                Operation::PopStack => {
                    if let Some(state) = stack.pop() {
                        turtle = state;
                    }
                }
                Operation::EnterFrame(_) => {}
                Operation::ExitFrame => {}
                Operation::ScaleWidth(s) => turtle.thickness *= s,
            }
        }

        set_default_camera();

        // UI Overlay
        draw_text("Fractal Stack", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Iteration: {} | Ops: {}/{}", iterations, draw_limit, operations.len()), 20.0, 60.0, 20.0, WHITE);
        draw_text("Space: Grow | Enter: Animate | Left/Right: Step", 20.0, 80.0, 20.0, WHITE);

        // Stack Visualization (2D Overlay)
        let stack_height = stack.len();
        draw_text(&format!("Stack Depth: {}", stack_height), 20.0, 110.0, 20.0, YELLOW);
        // Draw a bar representing the stack
        for i in 0..stack_height {
            draw_rectangle(20.0, 120.0 + i as f32 * 10.0, 20.0, 8.0, RED);
        }

        next_frame().await;
    }
}
