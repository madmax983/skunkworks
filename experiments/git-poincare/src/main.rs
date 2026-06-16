use git_associates::GitModel;
use poincare_disk::Point;
use macroquad::prelude::*;
use std::env;

#[macroquad::main("Git Poincaré")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    // Simulate some logic even in headless mode to not block CI
    let model = GitModel::open(".").unwrap();
    let history = model.history_with_diffs(50).unwrap_or_default();

    // Map commits to hyperbolic space
    // Let's say age pushes them out to boundary
    let mut commit_points = Vec::new();
    for (i, commit) in history.iter().enumerate() {
        let r = 1.0 - (1.0 / (1.0 + i as f64 * 0.1)); // distance from center based on age
        let theta = (commit.short_hash.chars().next().unwrap() as u32 as f64) * std::f64::consts::PI / 8.0;

        let point = Point::new(r * theta.cos(), r * theta.sin());
        commit_points.push(point);
    }

    if headless {
        println!("git-poincare running. Headless: {}", headless);
        println!("Loaded {} commits", history.len());
        println!("First mapped point: {:?}", commit_points.first());
        return; // exit early for headless
    }

    loop {
        clear_background(BLACK);

        // Draw Poincare Disk boundary
        draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, 300.0, 2.0, WHITE);

        // Draw commits
        for point in &commit_points {
            let x = screen_width() / 2.0 + point.re as f32 * 300.0;
            let y = screen_height() / 2.0 + point.im as f32 * 300.0;
            draw_circle(x, y, 3.0, RED);
        }

        next_frame().await;
    }
}
