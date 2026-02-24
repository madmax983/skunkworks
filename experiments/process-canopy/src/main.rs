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

        draw_sky_gradient();

        // Draw Ground / Soil
        draw_rectangle(
            0.0,
            screen_height() - 20.0,
            screen_width(),
            20.0,
            Color::new(0.4, 0.3, 0.2, 1.0),
        );
        draw_rectangle(
            0.0,
            screen_height() - 20.0,
            screen_width(),
            5.0,
            Color::new(0.2, 0.5, 0.2, 1.0),
        );

        // Mouse Interaction
        let mouse_pos = Vec2::from(mouse_position());
        let hovered_tree = trees.iter().find(|t| t.contains(mouse_pos));

        // Draw Trees
        for tree in &trees {
            let is_scheduled = sun.is_shining_on(tree);
            let is_hovered = hovered_tree
                .map(|t| t.stats.pid == tree.stats.pid)
                .unwrap_or(false);

            tree.draw(is_scheduled);

            // Highlight active trees
            if is_scheduled {
                draw_circle(tree.position.x, tree.position.y, 5.0, GOLD);
            }

            // Highlight hovered tree
            if is_hovered {
                draw_circle_lines(tree.position.x, tree.position.y, 30.0, 2.0, WHITE);
            }
        }

        sun.draw();

        // Draw UI
        draw_hud(&sys, sun.mode, hovered_tree);

        next_frame().await
    }
}

fn draw_hud(sys: &System, mode: ScheduleMode, hovered_tree: Option<&Tree>) {
    // Background Panel
    draw_rectangle(10.0, 10.0, 260.0, 100.0, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_rectangle_lines(10.0, 10.0, 260.0, 100.0, 2.0, WHITE);

    // FPS
    draw_text(&format!("FPS: {}", get_fps()), 20.0, 35.0, 20.0, WHITE);

    // Global Stats
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let global_cpu = sys.global_cpu_info().cpu_usage();

    draw_text(
        &format!("Global CPU: {:.1}%", global_cpu),
        20.0,
        60.0,
        20.0,
        if global_cpu > 50.0 { RED } else { GOLD },
    );
    draw_text(
        &format!("RAM: {:.1}GB / {:.1}GB", used_mem, total_mem),
        20.0,
        85.0,
        20.0,
        if used_mem / total_mem > 0.8 {
            RED
        } else {
            GOLD
        },
    );

    // Scheduler Mode
    let mode_str = match mode {
        ScheduleMode::RoundRobin => "Round Robin",
        ScheduleMode::Priority => "Priority",
    };
    draw_text(
        &format!("Mode: {} (Space)", mode_str),
        20.0,
        130.0,
        20.0,
        WHITE,
    );

    // Tooltip for Hovered Tree
    if let Some(tree) = hovered_tree {
        let tooltip_x = tree.position.x + 20.0;
        // Ensure tooltip stays on screen
        let tooltip_x = if tooltip_x + 200.0 > screen_width() {
            tree.position.x - 220.0
        } else {
            tooltip_x
        };

        let tooltip_y = tree.position.y - 120.0;

        draw_rectangle(
            tooltip_x,
            tooltip_y,
            200.0,
            100.0,
            Color::new(0.0, 0.0, 0.0, 0.8),
        );
        draw_rectangle_lines(tooltip_x, tooltip_y, 200.0, 100.0, 1.0, WHITE);

        draw_text(
            &tree.stats.name,
            tooltip_x + 10.0,
            tooltip_y + 20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("PID: {}", tree.stats.pid),
            tooltip_x + 10.0,
            tooltip_y + 40.0,
            16.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("CPU: {:.1}%", tree.stats.cpu_usage),
            tooltip_x + 10.0,
            tooltip_y + 60.0,
            16.0,
            GOLD,
        );
        draw_text(
            &format!("MEM: {} MB", tree.stats.memory / 1024 / 1024),
            tooltip_x + 10.0,
            tooltip_y + 80.0,
            16.0,
            SKYBLUE,
        );
    } else {
        draw_text(
            "Top 20 Processes (Hover to Inspect)",
            20.0,
            155.0,
            20.0,
            LIGHTGRAY,
        );
    }
}

fn draw_sky_gradient() {
    let top_color = Color::new(0.05, 0.1, 0.2, 1.0); // Deep Dark Blue
    let bottom_color = Color::new(0.4, 0.6, 0.8, 1.0); // Horizon Blue

    let steps = 20;
    let step_height = screen_height() / steps as f32;

    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let r = top_color.r + (bottom_color.r - top_color.r) * t;
        let g = top_color.g + (bottom_color.g - top_color.g) * t;
        let b = top_color.b + (bottom_color.b - top_color.b) * t;

        draw_rectangle(
            0.0,
            i as f32 * step_height,
            screen_width(),
            step_height + 1.0,
            Color::new(r, g, b, 1.0),
        );
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
