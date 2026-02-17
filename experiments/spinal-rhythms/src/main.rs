use macroquad::prelude::*;

pub mod lamprey;
pub mod neuro;
pub mod physics;

use lamprey::Lamprey;
use physics::PhysicsWorld;

#[macroquad::main("Spinal Rhythms")]
async fn main() {
    let mut world = PhysicsWorld::new();
    world.bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());

    // Center the lamprey
    let start_pos = vec2(screen_width() / 2.0 - 150.0, screen_height() / 2.0);
    let mut lamprey = Lamprey::new(&mut world, start_pos);

    let mut dragging_point: Option<usize> = None;

    loop {
        // Update
        let dt = get_frame_time().min(0.05); // Cap dt for stability

        // Handle Input
        if is_key_pressed(KeyCode::R) {
            world = PhysicsWorld::new();
            world.bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());
            lamprey = Lamprey::new(&mut world, start_pos);
        }

        if is_key_pressed(KeyCode::M) {
            lamprey.cpg.mutate();
        }

        // Drive Control (Base Drive adjustment)
        if is_key_down(KeyCode::Up) {
            lamprey.base_drive += 0.5;
        }
        if is_key_down(KeyCode::Down) {
            lamprey.base_drive -= 0.5;
        }
        lamprey.base_drive = lamprey.base_drive.clamp(0.0, 50.0);

        // Drive Override
        let drive_override = if is_key_down(KeyCode::Space) {
            Some(30.0)
        } else {
            None
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = vec2(mouse_position().0, mouse_position().1);
            // Find closest point
            let mut closest = None;
            let mut min_dist = 50.0;

            for (i, p) in world.points.iter().enumerate() {
                let d = p.pos.distance(mpos);
                if d < min_dist {
                    min_dist = d;
                    closest = Some(i);
                }
            }
            dragging_point = closest;
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_point = None;
        }

        if let Some(idx) = dragging_point {
            let mpos = vec2(mouse_position().0, mouse_position().1);
            if idx < world.points.len() {
                world.points[idx].pos = mpos;
                world.points[idx].old_pos = mpos;
                world.points[idx].locked = true;
            } else {
                dragging_point = None;
            }
        } else {
            for p in &mut world.points {
                p.locked = false;
            }
        }

        // Physics & AI
        lamprey.update(dt, &mut world, drive_override);
        world.update(dt);

        // Draw
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Draw Lamprey Body
        for i in 0..lamprey.num_segments {
            let l_muscle = world.constraints[lamprey.left_muscles[i]];
            let r_muscle = world.constraints[lamprey.right_muscles[i]];

            let l1 = world.points[l_muscle.p1].pos;
            let l2 = world.points[l_muscle.p2].pos;
            let r1 = world.points[r_muscle.p1].pos;
            let r2 = world.points[r_muscle.p2].pos;

            // Draw Quad (Triangles)
            draw_triangle(l1, r1, r2, Color::new(0.4, 0.6, 0.8, 1.0));
            draw_triangle(l1, r2, l2, Color::new(0.4, 0.6, 0.8, 1.0));

            // Neural activity visualization on body
            let (l_idx, r_idx) = lamprey.neuron_map[i];
            let l_v = lamprey.cpg.neurons[l_idx].v;
            let r_v = lamprey.cpg.neurons[r_idx].v;

            let l_act = ((l_v + 65.0) / 100.0).clamp(0.0, 1.0);
            let r_act = ((r_v + 65.0) / 100.0).clamp(0.0, 1.0);

            draw_circle(
                l1.x,
                l1.y,
                3.0 + l_act * 5.0,
                Color::new(1.0, 1.0 - l_act, 1.0 - l_act, 1.0),
            );
            draw_circle(
                r1.x,
                r1.y,
                3.0 + r_act * 5.0,
                Color::new(1.0, 1.0 - r_act, 1.0 - r_act, 1.0),
            );
        }

        // Draw CPG Dashboard
        let dash_h = 100.0;
        let dash_y = screen_height() - dash_h;
        draw_rectangle(0.0, dash_y, screen_width(), dash_h, BLACK);
        draw_line(0.0, dash_y, screen_width(), dash_y, 1.0, WHITE);

        for i in 0..lamprey.num_segments {
            let (l_idx, r_idx) = lamprey.neuron_map[i];
            let l_n = &lamprey.cpg.neurons[l_idx];
            let r_n = &lamprey.cpg.neurons[r_idx];

            let x = 50.0 + i as f32 * (screen_width() - 100.0) / lamprey.num_segments as f32;
            let y_l = dash_y + 30.0;
            let y_r = dash_y + 70.0;

            let l_col = if l_n.v > 0.0 {
                YELLOW
            } else {
                Color::new(0.2, 0.2, 0.2, 1.0)
            };
            let r_col = if r_n.v > 0.0 {
                YELLOW
            } else {
                Color::new(0.2, 0.2, 0.2, 1.0)
            };

            draw_circle(x, y_l, 5.0, l_col);
            draw_circle(x, y_r, 5.0, r_col);

            if i < lamprey.num_segments - 1 {
                let next_x =
                    50.0 + (i + 1) as f32 * (screen_width() - 100.0) / lamprey.num_segments as f32;
                draw_line(x, y_l, next_x, y_l, 1.0, DARKGRAY);
                draw_line(x, y_r, next_x, y_r, 1.0, DARKGRAY);
            }
        }

        // Instructions
        draw_text("Controls:", 20.0, 30.0, 20.0, WHITE);
        draw_text("Left Mouse: Drag Body", 20.0, 50.0, 20.0, GRAY);
        draw_text("Space: Turbo Stimulate", 20.0, 70.0, 20.0, GRAY);
        draw_text("Up/Down: Adjust Drive", 20.0, 90.0, 20.0, GRAY);
        draw_text("M: Mutate Weights", 20.0, 110.0, 20.0, GRAY);
        draw_text("R: Reset", 20.0, 130.0, 20.0, GRAY);

        draw_text(
            &format!("Drive: {:.1}", lamprey.base_drive),
            20.0,
            160.0,
            20.0,
            YELLOW,
        );

        next_frame().await
    }
}
