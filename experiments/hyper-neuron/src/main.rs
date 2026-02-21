mod math;
mod monitor;
mod network;

use math::Vec4D;
use monitor::SystemMonitor;
use network::Network;

use macroquad::prelude::*;

#[macroquad::main("Hyper-Neuron")]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let mut network = Network::new(100); // 100 Neurons

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        // 1. Update Monitor
        monitor.update();
        let dt = get_frame_time();

        // 2. Update Network
        // We pass smaller time steps to network to ensure stability if frame rate drops
        // But for visual simulation, 1 update per frame is okay if dt is small.
        // Let's cap dt.
        let sim_dt = dt.min(0.1);
        network.update(sim_dt * 1000.0, &monitor); // Network uses ms, dt is seconds.
        // Wait, Izhikevich usually uses ms. dt=1.0 is 1ms.
        // If I pass seconds * 1000, that's ms.
        // If frame time is 0.016s (60fps), that's 16ms per frame.
        // That's a lot of simulation steps if internal is 1ms.
        // Network::update handles integration.
        // If I pass 16.0, it simulates 16ms. This is good for real-time behavior.

        // 3. Rotate 4D Space
        let base_speed = 0.1;
        let speed_mult = 1.0 + monitor.load_avg * 2.0;
        angle_xw += dt * base_speed * speed_mult;
        angle_yw += dt * base_speed * 0.5 * speed_mult;
        angle_zw += dt * base_speed * 0.2;

        // 4. Camera Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // System Distortion Vector (Scaling Factors)
        let sx = 1.0 + monitor.cpu_usage;
        let sy = 1.0 + monitor.mem_usage;
        let sz = 1.0 + monitor.swap_usage;
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.5;

        // Transform Helper (4D -> 3D)
        let transform = |v: Vec4D| -> Vec3 {
            // Apply distortion
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            // Apply Global Rotation
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            // Project
            v.project_to_3d(4.0)
        };

        // Draw Neurons & Synapses
        for (i, neuron) in network.neurons.iter().enumerate() {
            let p1 = transform(neuron.position);

            // Draw Neuron
            // Color based on voltage: -65 (Blue) to 30 (Red)
            let v_norm = ((neuron.state.v + 65.0) / 95.0).clamp(0.0, 1.0);
            let color = Color::new(v_norm, 0.2, 1.0 - v_norm, 1.0);
            draw_sphere(p1, 0.05 + v_norm * 0.05, None, color);

            // Draw Synapses
            if let Some(synapses) = network.synapses.get(i) {
                for synapse in synapses {
                     let p2 = transform(network.neurons[synapse.target_index].position);

                     // Draw Line
                     draw_line_3d(p1, p2, Color::new(0.3, 0.3, 0.3, 0.1));

                     // Draw Spikes
                     let speed = 5.0; // Must match network.rs
                     for (timer, initial_dist) in &synapse.spikes {
                         // t goes from 0.0 (start) to 1.0 (end)
                         // timer goes from duration to 0.0
                         // duration = initial_dist / speed
                         let duration = initial_dist / speed;
                         if duration > 0.0 {
                             let t = (1.0 - (timer / duration)).clamp(0.0, 1.0);
                             let spike_pos = p1 + (p2 - p1) * t;
                             draw_sphere(spike_pos, 0.03, None, YELLOW);
                         }
                     }
                }
            }
        }

        // Draw Tesseract Bounds
        draw_tesseract_wireframe(Vec4D::new(2.0, 2.0, 2.0, 2.0), angle_xw, angle_yw, angle_zw, sx, sy, sz, sw);

        set_default_camera();
        draw_text("Hyper-Neuron", 10.0, 20.0, 30.0, WHITE);
        draw_text("4D SNN driven by System Metrics", 10.0, 40.0, 20.0, GRAY);

        draw_text(&format!("CPU: {:.0}% (X-Stretch)", monitor.cpu_usage * 100.0), 10.0, 70.0, 20.0, RED);
        draw_text(&format!("MEM: {:.0}% (Y-Stretch)", monitor.mem_usage * 100.0), 10.0, 90.0, 20.0, BLUE);
        draw_text(&format!("SWAP: {:.0}% (Z-Stretch)", monitor.swap_usage * 100.0), 10.0, 110.0, 20.0, GREEN);
        draw_text(&format!("LOAD: {:.2} (W-Oscillation)", monitor.load_avg), 10.0, 130.0, 20.0, YELLOW);

        next_frame().await
    }
}

fn draw_tesseract_wireframe(bounds: Vec4D, axw: f32, ayw: f32, azw: f32, sx: f32, sy: f32, sz: f32, sw: f32) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { bounds.x } else { -bounds.x };
        let y = if i & 2 != 0 { bounds.y } else { -bounds.y };
        let z = if i & 4 != 0 { bounds.z } else { -bounds.z };
        let w = if i & 8 != 0 { bounds.w } else { -bounds.w };
        verts.push(Vec4D::new(x, y, z, w));
    }

    let transform = |v: Vec4D| -> Vec3 {
        // Apply same distortion as plants
        let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
        v = v.rotate_xw(axw);
        v = v.rotate_yw(ayw);
        v = v.rotate_zw(azw);
        v.project_to_3d(4.0)
    };

    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                let p1 = transform(verts[i]);
                let p2 = transform(verts[j]);
                draw_line_3d(p1, p2, Color::new(0.5, 0.5, 0.5, 0.2));
            }
        }
    }
}
