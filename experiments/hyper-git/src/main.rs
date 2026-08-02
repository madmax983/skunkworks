//! 🧬 Splice: hyper-git
//!
//! Lineage:
//! - Parent A (git-associates): Provides chronological git commit history and metadata (hashes, message lengths).
//! - Parent B (hyper-system): Provides 4D math primitives (`Vec4`) for higher-dimensional spatial projection.
//!
//! Emergent Phenotype: Codebase metrics drive hyper-dimensional geometric torque, visualizing git history as 4D rotation.

use git_associates::GitModel;
use hyper_system::Vec4;
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Hyper Git".to_owned(),
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
    // Parent A allele: Git metadata parsing
    let model = GitModel::open(".").expect("Failed to open git repo");
    let commits = model.history(100).unwrap_or_default();

    let mut rotation_xw = 0.0;
    let mut rotation_yw = 0.0;

    let mut commit_idx = 0;

    loop {
        clear_background(BLACK);

        let mut string_len_metric = 0.0;

        if !commits.is_empty() {
            let commit = &commits[commit_idx % commits.len()];
            string_len_metric = commit.message.len() as f32;

            draw_text(
                &format!("Commit: {}", commit.short_hash),
                20.0,
                30.0,
                30.0,
                WHITE,
            );
            draw_text(
                &format!("Message Len: {}", string_len_metric),
                20.0,
                70.0,
                30.0,
                GREEN,
            );
        }

        commit_idx = (commit_idx + 1) % commits.len().max(1);

        // Novel trait: Using codebase metrics to drive 4D rotation torque
        rotation_xw += string_len_metric * 0.001 + 0.01;
        rotation_yw += string_len_metric * 0.002 + 0.01;

        // Parent B allele: 4D vector primitive
        let mut v = Vec4::new(100.0, 0.0, 0.0, 100.0);

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
        let distance = 200.0;
        let w_factor = 1.0 / (distance - v.w).max(0.1);

        let proj_x = v.x * w_factor * 200.0;
        let proj_y = v.y * w_factor * 200.0;

        draw_circle(
            400.0 + proj_x,
            300.0 + proj_y,
            10.0 * w_factor * 50.0, // Scale radius based on 4D depth
            WHITE,
        );

        next_frame().await;
    }
}
