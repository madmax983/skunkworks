use arthropod::Button;
use macroquad::prelude::*;
use poincare_disk::{Mobius, Point};
use std::f64::consts::PI;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Poincare (Hybrid)".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Ensure early return BEFORE macroquad main loops start if in headless mode
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: Exiting immediately to avoid X11 panics.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let btn_forward = Button::new("Forward", 10.0, 10.0, 120.0, 40.0)
        .with_colors(Color::new(0.2, 0.4, 0.8, 1.0), Color::new(0.3, 0.5, 0.9, 1.0), Color::new(0.1, 0.3, 0.7, 1.0));
    let btn_backward = Button::new("Backward", 10.0, 60.0, 120.0, 40.0)
        .with_colors(Color::new(0.2, 0.4, 0.8, 1.0), Color::new(0.3, 0.5, 0.9, 1.0), Color::new(0.1, 0.3, 0.7, 1.0));
    let btn_left = Button::new("Rot Left", 10.0, 110.0, 120.0, 40.0)
        .with_colors(Color::new(0.8, 0.4, 0.2, 1.0), Color::new(0.9, 0.5, 0.3, 1.0), Color::new(0.7, 0.3, 0.1, 1.0));
    let btn_right = Button::new("Rot Right", 10.0, 160.0, 120.0, 40.0)
        .with_colors(Color::new(0.8, 0.4, 0.2, 1.0), Color::new(0.9, 0.5, 0.3, 1.0), Color::new(0.7, 0.3, 0.1, 1.0));

    let mut current_transform = Mobius::identity();
    let mut grid_points: Vec<Point> = Vec::new();
    for angle in 0..12 {
        let theta = (angle as f64) * PI / 6.0;
        for r_step in 1..9 {
            let r = (r_step as f64) * 0.1;
            grid_points.push(num_complex::Complex::from_polar(r, theta));
        }
    }

    loop {
        clear_background(BLACK);

        let sw = screen_width();
        let sh = screen_height();
        let center_x = sw / 2.0;
        let center_y = sh / 2.0;
        let disk_radius = sw.min(sh) * 0.45;

        draw_circle_lines(center_x, center_y, disk_radius, 2.0, DARKGRAY);

        if btn_forward.draw() {
            let step = Mobius::translation(num_complex::Complex::new(0.05, 0.0));
            current_transform = step.then(&current_transform);
        }
        if btn_backward.draw() {
            let step = Mobius::translation(num_complex::Complex::new(-0.05, 0.0));
            current_transform = step.then(&current_transform);
        }
        if btn_left.draw() {
            let rot = Mobius::rotation(-0.05);
            current_transform = rot.then(&current_transform);
        }
        if btn_right.draw() {
            let rot = Mobius::rotation(0.05);
            current_transform = rot.then(&current_transform);
        }

        for p in &grid_points {
            let mapped = current_transform.apply(*p);
            let px = center_x + (mapped.re as f32) * disk_radius;
            let py = center_y + (mapped.im as f32) * disk_radius;
            let d_from_center = mapped.norm();
            let size = 3.0 * (1.0 - (d_from_center as f32)).max(0.1);
            let color_intensity = (1.0 - (d_from_center as f32)).max(0.2);
            let color = Color::new(color_intensity, color_intensity * 0.8, color_intensity * 1.5, 1.0);
            draw_circle(px, py, size, color);
        }

        draw_text("Hyperbolic Space Navigation", 140.0, 30.0, 20.0, WHITE);
        next_frame().await;
    }
}
