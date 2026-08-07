//! # git-locus
//!
//! A hybrid experiment crossing `git-associates` with `locus`.
//!
//! **Lineage:**
//! - From `git-associates`: Discrete Git commit metadata parsing, measuring churn and developer intent over time.
//! - From `locus`: Continuous non-Euclidean boundary wrapping (Torus, Klein Bottle, Mobius, etc.)
//!
//! **Phenotype:**
//! The chronological commit history is projected onto a continuous 2D boundary grid governed by topological rules. As codebase activity expands past visual Euclidean boundaries, the history naturally wraps and intersects itself. This creates overlapping "ghost timelines" of the repository's evolution, allowing observers to see how deeply coupled files interact with each other even when separated by immense linear time.
//!
//! This hybrid was created by **The Splice Surgeon 🧬**.
//!
use git_associates::GitModel;
use locus::Topology;
use macroquad::prelude::*;
use std::env;

fn window_conf() -> Conf {
    Conf {
        window_title: "Git Locus Morphogenesis".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn run_sim() {
    let model = match GitModel::open("../../") {
        Ok(model) => model,
        Err(_) => {
            println!("No git repo found. Exiting gracefully.");
            return;
        }
    };
    let commits = model.history(50).unwrap_or_default();

    let mut current_topology = Topology::Torus;
    let width = 800;
    let height = 600;

    // Initial simple spawn of points representing commits
    let mut commit_points = vec![];
    let mut cursor = Vec2::new(400.0, 300.0);

    for commit in commits {
        // use file length or string hash to generate initial perturbation
        let perturbation = commit.message.len() as f32 * 10.0;
        cursor.x += perturbation;
        cursor.y += perturbation * 0.5;

        // Wrap the cursor using locus topology
        if let Some((y, x)) =
            current_topology.normalize(cursor.y as i64, cursor.x as i64, width, height)
        {
            cursor.y = y as f32;
            cursor.x = x as f32;
        }
        commit_points.push(cursor);
    }

    // Default points if repo metadata is empty
    if commit_points.is_empty() {
        for i in 0..100 {
            let mut p = Vec2::new(400.0 + (i as f32 * 15.0), 300.0 + (i as f32 * 10.0));
            if let Some((y, x)) = current_topology.normalize(p.y as i64, p.x as i64, width, height)
            {
                p.y = y as f32;
                p.x = x as f32;
            }
            commit_points.push(p);
        }
    }

    let mut time: f32 = 0.0;

    loop {
        clear_background(BLACK);

        time += get_frame_time();

        // Dynamically wrap points and draw
        for point in commit_points.iter_mut() {
            // Apply drift to simulate time
            point.x += (time.sin() * 5.0) * get_frame_time() * 10.0;
            point.y += (time.cos() * 5.0) * get_frame_time() * 10.0;

            if let Some((y, x)) =
                current_topology.normalize(point.y as i64, point.x as i64, width, height)
            {
                point.y = y as f32;
                point.x = x as f32;
            }
            draw_circle(point.x, point.y, 4.0, Color::new(0.2, 0.8, 0.4, 0.8));
        }

        draw_text(
            "Git Locus: Topological Repository History",
            20.0,
            30.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Topology: {:?}", current_topology),
            20.0,
            50.0,
            20.0,
            GRAY,
        );

        // Cycle topology
        if is_key_pressed(KeyCode::T) {
            current_topology = match current_topology {
                Topology::Plane => Topology::CylinderH,
                Topology::CylinderH => Topology::Torus,
                Topology::Torus => Topology::Klein,
                Topology::Klein => Topology::Projective,
                Topology::Projective => Topology::Sphere,
                Topology::Sphere => Topology::Plane,
                _ => Topology::Torus,
            };
        }

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing simulation.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
