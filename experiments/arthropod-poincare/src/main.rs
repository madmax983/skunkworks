//! # Arthropod Poincare 🐜💠
//!
//! **Concept:** Interactive Hyperbolic Navigation.
//!
//! This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the continuous non-Euclidean geometry of `poincare-disk`.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
//! - **Parent B (crates/poincare-disk):** Provides the continuous non-Euclidean mathematics and Möbius transformations.
//!
//! ## Novel Trait
//! The abstract continuous geometry is now interactively explorable. Discrete UI button clicks apply continuous Möbius transformations (translations and rotations) to the view center, allowing the user to navigate the hyperbolic plane in real time.
//!
//! ## Predicted Phenotype
//! An interactive non-Euclidean kaleidoscope. As the user clicks the UI buttons to move around, the underlying Euclidean representation squashes and stretches according to hyperbolic metrics, bridging discrete GUI actions to non-Euclidean continuous spaces.

use arthropod::Button;
use macroquad::prelude::*;
use poincare_disk::{Mobius, Point, TilingConsts, neighbor_transform_a};
use std::f64::consts::PI;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Poincare".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Bypass macroquad::main to support headless execution
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
    let mut current_transform = Mobius::identity();

    // Setup interactive buttons
    let btn_up = Button::new("Move Up", 20.0, 20.0, 120.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_down = Button::new("Move Down", 20.0, 70.0, 120.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_left = Button::new("Move Left", 20.0, 120.0, 120.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_right = Button::new("Move Right", 20.0, 170.0, 120.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);

    let btn_rot_cw = Button::new("Rotate CW", 20.0, 240.0, 120.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_rot_ccw = Button::new("Rotate CCW", 20.0, 290.0, 120.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);

    let btn_reset = Button::new("Reset View", 20.0, 360.0, 120.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);

    let screen_center = vec2(400.0, 300.0);
    let disk_radius = 250.0;

    loop {
        clear_background(BLACK);

        // Draw Poincare Disk boundary
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 2.0, DARKGRAY);

        // Handle navigation interactions
        if btn_right.draw() {
            let a = neighbor_transform_a(0, &consts);
            let t = Mobius::translation(a);
            current_transform = t.then(&current_transform);
        }
        if btn_up.draw() {
            let a = neighbor_transform_a(1, &consts);
            let t = Mobius::translation(a);
            current_transform = t.then(&current_transform);
        }
        if btn_left.draw() {
            let a = neighbor_transform_a(2, &consts);
            let t = Mobius::translation(a);
            current_transform = t.then(&current_transform);
        }
        if btn_down.draw() {
            let a = neighbor_transform_a(3, &consts);
            let t = Mobius::translation(a);
            current_transform = t.then(&current_transform);
        }

        if btn_rot_ccw.draw() {
            let t = Mobius::rotation(PI / 8.0);
            current_transform = t.then(&current_transform);
        }
        if btn_rot_cw.draw() {
            let t = Mobius::rotation(-PI / 8.0);
            current_transform = t.then(&current_transform);
        }

        if btn_reset.draw() {
            current_transform = Mobius::identity();
        }

        // Draw an abstract representation of the space
        // We'll draw some points and transform them to show the hyperbolic distortion
        for r in 1..=4 {
            let r_f = r as f64 * 0.2;
            for theta_deg in (0..360).step_by(30) {
                let theta = (theta_deg as f64) * PI / 180.0;
                let z = Point::from_polar(r_f, theta);

                // Apply the inverse transform to map from "world" space to our "view" space
                let view_transform = current_transform.inverse();
                let z_transformed = view_transform.apply(z);

                // Map to screen
                let screen_x = screen_center.x + (z_transformed.re as f32) * disk_radius;
                let screen_y = screen_center.y - (z_transformed.im as f32) * disk_radius; // -y because screen space

                draw_circle(screen_x, screen_y, 3.0, WHITE);
            }
        }

        // Draw the center origin to ground the user
        let origin_transformed = current_transform.inverse().apply(Point::new(0.0, 0.0));
        let screen_x = screen_center.x + (origin_transformed.re as f32) * disk_radius;
        let screen_y = screen_center.y - (origin_transformed.im as f32) * disk_radius;
        draw_circle(screen_x, screen_y, 6.0, YELLOW);

        next_frame().await;
    }
}
