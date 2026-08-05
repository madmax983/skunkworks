//! # Hyper-Platter
//!
//! ## Concept
//! A 4D structural topology visualization combined with a fading thermal heat field.
//!
//! The experiment projects an interactive 4-dimensional hypercube down into a 2D scalar field (`platter`). As the structural constraints of the 4D geometry rotate across higher-dimensional planes (like XW or YW) based on real-time system metrics (CPU load), its vertices scrape against the 2D thermodynamic slice, injecting scalar heat into the field. The result is a fading, glowing topological heatmap visualizing the hidden non-Euclidean structural volume passing through our slice of reality.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p hyper-platter
//! ```
//!
//! Headless bypass for CI:
//! ```bash
//! cargo run -p hyper-platter -- --headless
//! ```
//!
use hyper_system::{SystemMonitor, Vec4};
use macroquad::prelude::*;
use platter::Platter;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyper Platter 4D Heatmap".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode enabled. Bypassing macroquad initialization.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut monitor = SystemMonitor::new();
    let mut platter = Platter::new(800, 600);
    let mut image = Image::gen_image_color(800, 600, BLACK);
    let texture = Texture2D::from_image(&image);

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
        monitor.update();

        let dt = get_frame_time();
        platter.decay(0.95);

        let cpu_stress = monitor.cpu_usage;
        rot_xw_angle += (0.5 + cpu_stress * 2.0) * dt;
        rot_yw_angle += (0.3 + cpu_stress * 1.5) * dt;

        for p in &points {
            let p_rot = p.rotate_xw(rot_xw_angle).rotate_yw(rot_yw_angle);
            let p3 = p_rot.project_to_3d(3.0);

            let screen_x = p3.x * 150.0 + 400.0;
            let screen_y = p3.y * 150.0 + 300.0;

            let px = screen_x as isize;
            let py = screen_y as isize;

            if px >= 0 && px < 800 && py >= 0 && py < 600 {
                for dx in -3..=3 {
                    for dy in -3..=3 {
                        let nx = px + dx;
                        let ny = py + dy;
                        if nx >= 0 && nx < 800 && ny >= 0 && ny < 600 {
                            let dist_sq = (dx * dx + dy * dy) as f64;
                            let heat = if dist_sq == 0.0 { 1.0 } else { 1.0 / dist_sq };
                            platter.accumulate(nx as usize, ny as usize, heat * 0.1);
                        }
                    }
                }
            }
        }

        for y in 0..600 {
            for x in 0..800 {
                let heat = platter.get(x, y) as f32;

                let r = (heat * 255.0).min(255.0) as u8;
                let g = ((heat - 1.0).max(0.0) * 255.0).min(255.0) as u8;
                let b = ((heat - 2.0).max(0.0) * 255.0).min(255.0) as u8;

                let idx = (y * 800 + x) * 4;
                image.bytes[idx] = r;
                image.bytes[idx + 1] = g;
                image.bytes[idx + 2] = b;
                image.bytes[idx + 3] = 255;
            }
        }

        texture.update(&image);
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
