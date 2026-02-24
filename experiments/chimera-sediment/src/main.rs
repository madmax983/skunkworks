mod detritivore;
mod lsystem;
mod monitor;
mod sediment;
mod simulation;
mod tree;

use detritivore::Detritivore;
use macroquad::prelude::*;
use monitor::fetch_processes;
use sediment::SedimentParticle;
use simulation::{ScheduleMode, Sun};
use sysinfo::System;
use tree::Tree;

#[macroquad::main("Chimera Sediment")]
async fn main() {
    let mut sys = System::new_all();
    let mut trees: Vec<Tree> = Vec::new();
    let mut detritivores: Vec<Detritivore> = Vec::new();
    let mut sediment: Vec<SedimentParticle> = Vec::new();
    let mut last_update = 0.0;
    let mut sun = Sun::new();

    // Initial fetch
    update_forest(&mut sys, &mut trees, &mut sediment);

    // Initial Spawning
    for _ in 0..10 {
        let x = rand::gen_range(0.0, screen_width());
        detritivores.push(Detritivore::new(vec2(x, screen_height() - 20.0)));
    }

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

        sun.update(dt * 2.0);
        if sun.position.x > screen_width() {
            sun.position.x = 0.0;
        }

        // Process Update (Forest Cycle)
        if time - last_update > 2.0 {
            update_forest(&mut sys, &mut trees, &mut sediment);
            last_update = time;
        }

        // Priority Logic
        if sun.mode == ScheduleMode::Priority && !trees.is_empty() {
            if let Some(target) = trees
                .iter()
                .max_by(|a, b| a.stats.cpu_usage.partial_cmp(&b.stats.cpu_usage).unwrap())
            {
                let diff = target.position.x - sun.position.x;
                if diff.abs() > 5.0 {
                    sun.position.x += diff.signum() * 200.0 * dt;
                } else {
                    sun.position.x = target.position.x;
                }
            }
        }

        // UPDATE SEDIMENT
        // Ground level increases as sediment piles up?
        // For simplicity, ground is fixed, but particles pile up visually if we implement collision.
        // For now, let's just use fixed ground.
        let ground_level = screen_height() - 20.0;

        for p in &mut sediment {
            p.update(dt, ground_level);
        }

        // UPDATE DETRITIVORES
        // Spawn more if low
        if detritivores.len() < 5 {
            let x = rand::gen_range(0.0, screen_width());
            detritivores.push(Detritivore::new(vec2(x, ground_level)));
        }

        let mut new_detritivores = Vec::new();
        for detritivore in &mut detritivores {
            if let Some(child) = detritivore.update(dt, &mut sediment, ground_level) {
                new_detritivores.push(child);
            }
        }
        detritivores.extend(new_detritivores);

        // Limit sediment
        if sediment.len() > 500 {
            sediment.drain(0..sediment.len() - 500);
        }

        // DRAWING
        draw_sky_gradient();

        // Draw Ground
        draw_rectangle(
            0.0,
            screen_height() - 20.0,
            screen_width(),
            20.0,
            Color::new(0.3, 0.2, 0.1, 1.0),
        );

        // Draw Sediment (Behind trees?)
        for p in &sediment {
            p.draw();
        }

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

            if is_scheduled {
                draw_circle(tree.position.x, tree.position.y, 5.0, GOLD);
            }
            if is_hovered {
                draw_circle_lines(tree.position.x, tree.position.y, 30.0, 2.0, WHITE);
            }
        }

        // DRAW DETRITIVORES
        for d in &detritivores {
            d.draw();
        }

        sun.draw();

        // Draw UI
        draw_hud(
            &sys,
            sun.mode,
            hovered_tree,
            detritivores.len(),
            sediment.len(),
        );

        next_frame().await
    }
}

fn update_forest(sys: &mut System, trees: &mut Vec<Tree>, sediment: &mut Vec<SedimentParticle>) {
    let processes = fetch_processes(sys);

    // Store old trees to check for dead ones
    let old_trees: Vec<Tree> = trees.drain(..).collect();

    // Build new trees (Top 20)
    let top_20 = processes.into_iter().take(20).collect::<Vec<_>>();
    let spacing = screen_width() / (top_20.len() as f32 + 1.0);

    for (i, p) in top_20.iter().enumerate() {
        let x = (i as f32 + 1.0) * spacing;
        let y = screen_height() - 20.0;
        trees.push(Tree::new(p.clone(), vec2(x, y)));
    }

    // Check for fallen trees (Present in old but not in new)
    let new_pids: Vec<_> = trees.iter().map(|t| t.stats.pid).collect();

    for old_tree in old_trees {
        if !new_pids.contains(&old_tree.stats.pid) {
            // It fell out of canopy
            sediment.extend(old_tree.collapse());
        }
    }
}

fn draw_hud(
    sys: &System,
    mode: ScheduleMode,
    hovered_tree: Option<&Tree>,
    detritivore_count: usize,
    sediment_count: usize,
) {
    draw_rectangle(10.0, 10.0, 260.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_rectangle_lines(10.0, 10.0, 260.0, 120.0, 2.0, WHITE);

    draw_text(&format!("FPS: {}", get_fps()), 20.0, 35.0, 20.0, WHITE);

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

    draw_text(
        &format!("Detritivores: {}", detritivore_count),
        20.0,
        105.0,
        20.0,
        GREEN,
    );
    draw_text(
        &format!("Sediment: {}", sediment_count),
        150.0,
        105.0,
        20.0,
        BROWN,
    );

    let mode_str = match mode {
        ScheduleMode::RoundRobin => "Round Robin",
        ScheduleMode::Priority => "Priority",
    };
    draw_text(
        &format!("Mode: {} (Space)", mode_str),
        20.0,
        150.0,
        20.0,
        WHITE,
    );

    if let Some(tree) = hovered_tree {
        let tooltip_x = tree.position.x + 20.0;
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
        draw_text("Top 20 Processes (Canopy)", 20.0, 175.0, 20.0, LIGHTGRAY);
    }
}

fn draw_sky_gradient() {
    let top_color = Color::new(0.1, 0.1, 0.1, 1.0); // Darker sky for necrotic theme
    let bottom_color = Color::new(0.3, 0.3, 0.3, 1.0); // Grey fog

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
