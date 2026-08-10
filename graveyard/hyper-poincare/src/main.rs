//! 🧬 Splice: hyper-poincare
//!
//! Lineage:
//! - Parent A (hyper-system): Provides 4D math primitives (`Vec4`) for higher-dimensional spatial projection.
//! - Parent B (poincare-disk): Provides the continuous non-Euclidean coordinate space (`Mobius`, `Point`).
//!
//! Emergent Phenotype: Hyperbolic 4D Rotation. A 4-dimensional hypercube is rotated, projected down to 3D, and then squashed into the non-Euclidean boundary of the Poincaré disk, demonstrating exponential compression of higher dimensions towards a geometric boundary.

use hyper_system::Vec4;
use macroquad::prelude::*;
use poincare_disk::{Mobius, Point};

fn conf() -> Conf {
    Conf {
        window_title: "Hyper Poincare".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad initialization.");
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        run_headless();
        return;
    }

    macroquad::Window::from_config(conf(), async_main());
}

async fn async_main() {
    let mut rotation_xw: f32 = 0.0;
    let mut rotation_yw: f32 = 0.0;
    let mut transform = Mobius::rotation(0.0);

    loop {
        clear_background(BLACK);

        let rot_step = Mobius::rotation(0.01);
        transform = transform.then(&rot_step);

        rotation_xw += 0.02;
        rotation_yw += 0.03;

        // Draw Disk boundary
        for i in 0..100 {
            let angle = (i as f32) / 100.0 * 2.0 * std::f32::consts::PI;
            let cx = 400.0 + angle.cos() * 200.0;
            let cy = 300.0 + angle.sin() * 200.0;
            draw_circle(cx, cy, 2.0, DARKGRAY);
        }

        // Vertices of a hypercube
        let mut vertices = vec![];
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    for w in [-1.0, 1.0] {
                        vertices.push(Vec4::new(x as f32, y as f32, z as f32, w as f32));
                    }
                }
            }
        }

        for v_orig in vertices {
            let mut v = v_orig;

            // Apply 4D rotation (XW plane)
            let cos_xw = rotation_xw.cos();
            let sin_xw = rotation_xw.sin();
            let x1 = v.x * cos_xw - v.w * sin_xw;
            let w1 = v.x * sin_xw + v.w * cos_xw;
            v.x = x1;
            v.w = w1;

            // Apply 4D rotation (YW plane)
            let cos_yw = rotation_yw.cos();
            let sin_yw = rotation_yw.sin();
            let y1 = v.y * cos_yw - v.w * sin_yw;
            let w2 = v.y * sin_yw + v.w * cos_yw;
            v.y = y1;
            v.w = w2;

            // Project 4D to 2D
            let distance = 3.0;
            let w_factor = 1.0 / (distance - v.w).max(0.1);
            let proj_x = v.x * w_factor;
            let proj_y = v.y * w_factor;

            // Map the Euclidean grid point using the current Mobius transform
            let mut p = Point::new(proj_x as f64, proj_y as f64);

            // Shrink it so it's strictly inside the disk
            p = Point::new(p.re * 0.5, p.im * 0.5);
            p = transform.apply(p);

            draw_circle(
                400.0 + p.re as f32 * 200.0,
                300.0 + p.im as f32 * 200.0,
                3.0 * w_factor,
                WHITE,
            );
        }

        next_frame().await;
    }
}
