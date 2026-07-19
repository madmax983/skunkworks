use arthropod::Button;
use macroquad::prelude::*;
use poincare_disk::{neighbor_transform_a, Mobius, Point, TilingConsts};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Poincare".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let consts = TilingConsts::new_4_5();
    let mut current_transform = Mobius::translation(Point::new(0.0, 0.0));

    let btn_right = Button::new("Right", 20.0, 20.0, 100.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);
    let btn_up = Button::new("Up", 20.0, 70.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_left = Button::new("Left", 20.0, 120.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_down = Button::new("Down", 20.0, 170.0, 100.0, 40.0).with_colors(GRAY, LIGHTGRAY, BLACK);
    let btn_rot = Button::new("Rotate", 20.0, 220.0, 100.0, 40.0).with_colors(YELLOW, GOLD, ORANGE);

    let screen_w = 800.0;
    let screen_h = 800.0;
    let center_x = screen_w / 2.0;
    let center_y = screen_h / 2.0;
    let disk_radius = 350.0;

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_right.draw() {
            let t = Mobius::translation(neighbor_transform_a(0, &consts));
            current_transform = current_transform.then(&t);
        }

        if btn_up.draw() {
            let t = Mobius::translation(neighbor_transform_a(1, &consts));
            current_transform = current_transform.then(&t);
        }

        if btn_left.draw() {
            let t = Mobius::translation(neighbor_transform_a(2, &consts));
            current_transform = current_transform.then(&t);
        }

        if btn_down.draw() {
            let t = Mobius::translation(neighbor_transform_a(3, &consts));
            current_transform = current_transform.then(&t);
        }

        if btn_rot.draw() {
            let t = Mobius::rotation(std::f64::consts::PI / 8.0);
            current_transform = current_transform.then(&t);
        }

        // Draw boundary
        draw_circle_lines(center_x, center_y, disk_radius, 2.0, color_u8!(100, 100, 120, 255));

        // Draw the {4,5} central tile (which is a square).
        // Vertices are at distance `vertex_offset` from origin, angles at PI/4, 3PI/4, 5PI/4, 7PI/4
        let mut base_vertices = vec![];
        for i in 0..4 {
            let angle = std::f64::consts::PI / 4.0 + (i as f64) * std::f64::consts::PI / 2.0;
            base_vertices.push(Point::new(
                consts.vertex_offset * angle.cos(),
                consts.vertex_offset * angle.sin(),
            ));
        }

        // Apply current transform and map to screen
        let mut screen_points = vec![];
        for v in &base_vertices {
            let transformed = current_transform.apply(*v);
            screen_points.push(Vec2::new(
                center_x + (transformed.re as f32 * disk_radius),
                center_y + (transformed.im as f32 * disk_radius),
            ));
        }

        // Draw lines
        for i in 0..4 {
            let p1 = screen_points[i];
            let p2 = screen_points[(i + 1) % 4];
            draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, WHITE);
        }

        next_frame().await;
    }
}
