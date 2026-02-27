use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;

fn generate_tesseract_base() -> (Vec<Vec4>, Vec<(usize, usize)>) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { 1.0 } else { -1.0 };
        let y = if i & 2 != 0 { 1.0 } else { -1.0 };
        let z = if i & 4 != 0 { 1.0 } else { -1.0 };
        let w = if i & 8 != 0 { 1.0 } else { -1.0 };
        verts.push(Vec4::new(x, y, z, w));
    }

    let mut edges = Vec::new();
    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                edges.push((i, j));
            }
        }
    }
    (verts, edges)
}

#[macroquad::main("Tesseract Ops")]
async fn main() {
    let (base_verts, edges) = generate_tesseract_base();
    let mut monitor = SystemMonitor::new();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let angle_zw = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Rotate based on load?
        // Let's make the rotation speed depend on load
        let base_speed = 0.2;
        let speed_mult = 1.0 + monitor.load_avg * 2.0;
        angle_xw += dt * base_speed * speed_mult;
        angle_yw += dt * base_speed * 0.7 * speed_mult;

        // Input Camera
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

        // Deform and Transform Vertices
        // X-axis scale: CPU
        // Y-axis scale: Memory
        // Z-axis scale: Swap
        // W-axis scale: Load (Breathing)

        let sx = 1.0 + monitor.cpu_usage;
        let sy = 1.0 + monitor.mem_usage;
        let sz = 1.0 + monitor.swap_usage;
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.2;

        let transform = |v: Vec4| -> Vec3 {
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v.project_to_3d(3.0)
        };

        // Draw Edges
        for &(i, j) in &edges {
            let v1 = base_verts[i];
            let v2 = base_verts[j];
            let p1 = transform(v1);
            let p2 = transform(v2);

            // Color based on metrics
            // High CPU -> Red
            // High Mem -> Blue
            // Low Load -> Green
            let r = monitor.cpu_usage;
            let b = monitor.mem_usage;
            let g = 1.0 - monitor.load_avg;

            draw_line_3d(p1, p2, Color::new(r, g, b, 0.8));
        }

        // Draw Vertices as spheres
        for v in &base_verts {
            let p = transform(*v);
            // Size based on load
            let size = 0.05 + monitor.load_avg * 0.1;
            draw_sphere(p, size, None, WHITE);
        }

        set_default_camera();
        draw_text("Tesseract Ops", 10.0, 20.0, 30.0, WHITE);

        draw_text(
            &format!("CPU (X-Scale): {:.0}%", monitor.cpu_usage * 100.0),
            10.0,
            50.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM (Y-Scale): {:.0}%", monitor.mem_usage * 100.0),
            10.0,
            70.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("SWP (Z-Scale): {:.0}%", monitor.swap_usage * 100.0),
            10.0,
            90.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!("LOD (Rotation): {:.2}", monitor.load_avg * 4.0),
            10.0,
            110.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}
