mod lsystem;
mod monitor;
mod simulation;
mod tree;

use macroquad::prelude::*;
use monitor::fetch_processes;
use simulation::{ScheduleMode, Sun};
use sysinfo::System;
use tree::Tree;

#[macroquad::main("Process Canopy")]
async fn main() {
    let mut sys = System::new_all();
    let mut trees: Vec<Tree> = Vec::new();
    let mut last_update = 0.0;
    let mut sun = Sun::new();

    // Initial fetch
    repopulate_forest(&mut sys, &mut trees);

    loop {
        let dt = get_frame_time();
        let time = get_time();

        // Sun Logic
        if is_key_pressed(KeyCode::Space) {
            sun.mode = match sun.mode {
                ScheduleMode::RoundRobin => ScheduleMode::Priority,
                ScheduleMode::Priority => ScheduleMode::RoundRobin,
            };
        }

        sun.update(dt * 2.0); // Speed up a bit
        if sun.position.x > screen_width() {
            sun.position.x = 0.0;
        }

        // Process Update
        if time - last_update > 2.0 {
            repopulate_forest(&mut sys, &mut trees);
            last_update = time;
        }

        // Priority Logic: Snap Sun to Highest CPU if mode is Priority
        if sun.mode == ScheduleMode::Priority && !trees.is_empty() {
            // Find tree with highest CPU
            if let Some(target) = trees
                .iter()
                .max_by(|a, b| a.stats.cpu_usage.partial_cmp(&b.stats.cpu_usage).unwrap())
            {
                // Move sun towards target
                let diff = target.position.x - sun.position.x;
                if diff.abs() > 5.0 {
                    sun.position.x += diff.signum() * 200.0 * dt;
                } else {
                    sun.position.x = target.position.x;
                }
            }
        }

        clear_background(SKYBLUE);

        // Draw Ground / Soil
        draw_rectangle(0.0, screen_height() - 20.0, screen_width(), 20.0, BROWN);

        // Draw Trees
        for tree in &trees {
            let is_scheduled = sun.is_shining_on(tree);
            tree.draw(is_scheduled);

            // Highlight active trees
            if is_scheduled {
                draw_circle(tree.position.x, tree.position.y, 5.0, GOLD);
            }
        }

        sun.draw();

        // Draw UI
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, BLACK);
        draw_text("Top 20 Processes by CPU", 20.0, 50.0, 20.0, BLACK);
        draw_text(
            &format!(
                "Scheduler Mode: {} (Space)",
                match sun.mode {
                    ScheduleMode::RoundRobin => "Round Robin",
                    ScheduleMode::Priority => "Priority",
                }
            ),
            20.0,
            80.0,
            20.0,
            BLACK,
        );

        // Instructions
        draw_text(
            "Sun highlights active process (simulated)",
            20.0,
            screen_height() - 40.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

fn repopulate_forest(sys: &mut System, trees: &mut Vec<Tree>) {
    let processes = fetch_processes(sys);
    let top = processes.into_iter().take(20).collect::<Vec<_>>();

    trees.clear();
    let spacing = screen_width() / (top.len() as f32 + 1.0);
    for (i, p) in top.iter().enumerate() {
        let x = (i as f32 + 1.0) * spacing;
        let y = screen_height() - 20.0;
        let tree = Tree::new(p.clone(), vec2(x, y));
        trees.push(tree);
    }
}
