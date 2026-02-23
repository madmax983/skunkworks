mod logic;
mod network;

use logic::{assign_target_angles, calculate_strip_transforms, parse_trace, SegmentType};
use macroquad::prelude::*;
use nalgebra::Point3;
use network::Network;

#[macroquad::main("Synaptic Trace")]
async fn main() {
    let raw_trace = "
stack backtrace:
   0: std::sys::unix::process::process_common::Command::spawn
             at /rustc/std/src/sys/unix/process/process_common.rs:100
   1: std::process::Command::spawn
             at /rustc/std/src/process.rs:800
   2: chimera_lang::vm::nova::spawn_process
             at experiments/chimera-lang/src/vm/nova.rs:205
   3: chimera_lang::vm::nova::OpCode::Mitosis
             at experiments/chimera-lang/src/vm/nova.rs:150
   4: chimera_lang::vm::execute_strand
             at experiments/chimera-lang/src/vm/mod.rs:500
   5: chimera_lang::vm::tick
             at experiments/chimera-lang/src/vm/mod.rs:450
   6: chimera_app::main_loop
             at experiments/chimera-lang/src/main.rs:120
   7: macroquad::miniquad::window::order_draw
             at /cargo/registry/miniquad/src/window.rs:300
   8: core::ops::function::FnOnce::call_once
             at /rustc/core/src/ops/function.rs:250
    ";

    let segments = parse_trace(raw_trace);
    let angles = assign_target_angles(&segments);

    let mut network = Network::new(segments.len());
    network.connect_trace();

    let mut fold_progress = 0.0f32;
    let mut auto_fold = true;

    let mut cam_yaw = 0.0f32;
    let mut cam_pitch = 0.5f32;
    let mut cam_dist = 20.0f32;

    let mut inputs = vec![0.0; segments.len()];

    loop {
        clear_background(BLACK);

        // Input
        if is_key_down(KeyCode::Left) {
            fold_progress = (fold_progress - 0.01).max(0.0);
            auto_fold = false;
        }
        if is_key_down(KeyCode::Right) {
            fold_progress = (fold_progress + 0.01).min(1.0);
            auto_fold = false;
        }
        if is_key_pressed(KeyCode::Space) {
            // Trigger root neuron spike
            inputs[0] = 50.0;
        } else {
            inputs[0] = 0.0;
        }

        if auto_fold {
            fold_progress = (fold_progress + 0.005).min(1.0);
        }

        // Camera
        if is_key_down(KeyCode::A) {
            cam_yaw += 0.03;
        }
        if is_key_down(KeyCode::D) {
            cam_yaw -= 0.03;
        }
        if is_key_down(KeyCode::W) {
            cam_pitch += 0.03;
        }
        if is_key_down(KeyCode::S) {
            cam_pitch -= 0.03;
        }
        cam_dist = (cam_dist + mouse_wheel().1 * -0.5).clamp(5.0, 50.0);

        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., 5., 0.),
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        // Update Physics
        // Run multiple ticks per frame for speed
        for _ in 0..5 {
            network.step(1.0, &inputs);
            // Reset input after one tick
            if inputs[0] > 0.0 {
                inputs[0] = 0.0;
            }
        }

        // Draw Trace
        let transforms = calculate_strip_transforms(&segments, &angles, fold_progress);

        for (i, transform) in transforms.iter().enumerate() {
            let segment = &segments[i];
            let neuron = &network.neurons[i];

            // Color based on voltage
            // Resting ~ -65, Spike ~ 30. Range ~ 100.
            // Map -65..30 to Color
            let v_norm = ((neuron.v + 70.0) / 100.0).clamp(0.0, 1.0);

            let base_color = match segment.segment_type {
                SegmentType::User => BLUE,
                SegmentType::System => DARKGRAY,
            };

            // Flash color
            let flash_color = if neuron.v > 0.0 { YELLOW } else { RED };

            let color = Color::new(
                base_color.r + (flash_color.r - base_color.r) * v_norm,
                base_color.g + (flash_color.g - base_color.g) * v_norm,
                base_color.b + (flash_color.b - base_color.b) * v_norm,
                1.0,
            );

            // Draw Wireframe Box
            let w = 1.5;
            let h = 0.8;
            let d = 0.2;

            // Local corners (centered on X, base on Y=0, center Z)
            let corners_local = [
                Point3::new(-w / 2., 0., -d / 2.),
                Point3::new(w / 2., 0., -d / 2.),
                Point3::new(w / 2., 0., d / 2.),
                Point3::new(-w / 2., 0., d / 2.),
                Point3::new(-w / 2., h, -d / 2.),
                Point3::new(w / 2., h, -d / 2.),
                Point3::new(w / 2., h, d / 2.),
                Point3::new(-w / 2., h, d / 2.),
            ];

            let c: Vec<Vec3> = corners_local
                .iter()
                .map(|p| {
                    let world = transform * p;
                    vec3(world.x, world.y, world.z)
                })
                .collect();

            // Draw box lines
            // Macroquad lines are constant thickness usually but let's try.
            // Actually draw_line_3d doesn't take thickness easily without custom geometry, color is enough.

            draw_line_3d(c[0], c[1], color);
            draw_line_3d(c[1], c[2], color);
            draw_line_3d(c[2], c[3], color);
            draw_line_3d(c[3], c[0], color);
            draw_line_3d(c[4], c[5], color);
            draw_line_3d(c[5], c[6], color);
            draw_line_3d(c[6], c[7], color);
            draw_line_3d(c[7], c[4], color);
            draw_line_3d(c[0], c[4], color);
            draw_line_3d(c[1], c[5], color);
            draw_line_3d(c[2], c[6], color);
            draw_line_3d(c[3], c[7], color);
        }

        set_default_camera();

        draw_text(
            &format!("Fold: {:.2}", fold_progress),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Controls: SPACE to Spike Root, Arrows to Fold, WASD+Wheel to Move Camera",
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "Neurons Active: {}",
                network.neurons.iter().filter(|n| n.v > -50.0).count()
            ),
            10.0,
            60.0,
            20.0,
            YELLOW,
        );

        next_frame().await
    }
}
