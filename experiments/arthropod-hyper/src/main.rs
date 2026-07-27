use arthropod::Button;
use hyper_system::{SystemMonitor, Vec4};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Hyper".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Ensure the headless bypass is checked synchronously before `Window::from_config`
fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode enabled. Bypassing macroquad initialization.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut monitor = SystemMonitor::new();

    // UI controls for rotating the 4D space
    let btn_rot_xw =
        Button::new("Rot XW", 10.0, 10.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_rot_yw =
        Button::new("Rot YW", 10.0, 60.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);

    let mut rot_xw_angle = 0.0;
    let mut rot_yw_angle = 0.0;

    let points = vec![
        Vec4::new(-1.0, -1.0, -1.0, -1.0),
        Vec4::new(1.0, -1.0, -1.0, -1.0),
        Vec4::new(1.0, 1.0, -1.0, -1.0),
        Vec4::new(-1.0, 1.0, -1.0, -1.0),
        Vec4::new(-1.0, -1.0, 1.0, -1.0),
        Vec4::new(1.0, -1.0, 1.0, -1.0),
        Vec4::new(1.0, 1.0, 1.0, -1.0),
        Vec4::new(-1.0, 1.0, 1.0, -1.0),
        Vec4::new(-1.0, -1.0, -1.0, 1.0),
        Vec4::new(1.0, -1.0, -1.0, 1.0),
        Vec4::new(1.0, 1.0, -1.0, 1.0),
        Vec4::new(-1.0, 1.0, -1.0, 1.0),
        Vec4::new(-1.0, -1.0, 1.0, 1.0),
        Vec4::new(1.0, -1.0, 1.0, 1.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Vec4::new(-1.0, 1.0, 1.0, 1.0),
    ];

    loop {
        clear_background(BLACK);
        monitor.update();

        // The CPU usage naturally drives the rotation, but the UI buttons can accelerate it
        let mut cpu_stress = monitor.cpu_usage;

        if btn_rot_xw.draw() {
            cpu_stress += 5.0; // Artificial stress injection
        }
        if btn_rot_yw.draw() {
            rot_yw_angle += 0.1;
        }

        rot_xw_angle += cpu_stress * 0.05;

        // Draw hypercube
        for p in &points {
            let p_rot = p.rotate_xw(rot_xw_angle).rotate_yw(rot_yw_angle);
            // Camera dist 3.0
            let p3 = p_rot.project_to_3d(3.0);

            // Map 3D to 2D screen
            // Simple orthographic projection
            let screen_x = p3.x * 100.0 + screen_width() / 2.0;
            let screen_y = p3.y * 100.0 + screen_height() / 2.0;

            draw_circle(screen_x, screen_y, 4.0, RED);
        }

        next_frame().await;
    }
}
