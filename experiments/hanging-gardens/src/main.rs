use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

mod lsystem;
mod sexagesimal;
mod sim;
mod turtle;

use git_associates::{Commit, FileChange, GitModel};
use sim::Garden;
use tui_shared::Tui;

fn main() -> Result<()> {
    // Terminal setup
    let mut tui = Tui::init()?;

    // Load History
    let mut commits = match GitModel::open(".") {
        Ok(model) => {
            match model.history_with_diffs(usize::MAX) {
                Ok(mut c) => {
                    c.reverse(); // Oldest first
                    c
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };

    if commits.is_empty() {
        // Demo Data
        commits.push(Commit {
            hash: "DEMO1".to_string(),
            short_hash: "DEMO1".to_string(),
            author: "Demo".to_string(),
            message: "Seed".to_string(),
            timestamp: chrono::Utc::now(),
            parents: vec![],
            stats: None,
            files: vec![
                FileChange {
                    path: "garden.rs".to_string(),
                    extension: "rs".to_string(),
                    insertions: 100,
                    deletions: 0,
                    is_binary: false,
                    hunks: vec![],
                },
            ],
        });
        commits.push(Commit {
            hash: "DEMO2".to_string(),
            short_hash: "DEMO2".to_string(),
            author: "Demo".to_string(),
            message: "Refactor".to_string(),
            timestamp: chrono::Utc::now(),
            parents: vec![],
            stats: None,
            files: vec![
                FileChange {
                    path: "garden.rs".to_string(),
                    extension: "rs".to_string(),
                    insertions: 10,
                    deletions: 50,
                    is_binary: false,
                    hunks: vec![],
                },
            ],
        });
    }

    let size = tui.terminal.size()?;
    let width = size.width as f64;
    let height = (size.height.saturating_sub(4)) as f64 * 4.0; // Scale up for canvas precision

    let mut garden = Garden::new(width, height);

    let mut commit_idx = 0;
    let mut speed = 1;
    let mut paused = false;
    let mut should_quit = false;

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        if !paused && commit_idx < commits.len() {
            // Process speed number of commits
            // Or maybe speed is just frames skip? Let's process 1 commit every `speed` frames?
            // No, standard is process `speed` commits per frame? That's too fast.
            // Let's do: process 1 commit per frame, but skip `speed` frames if speed < 0?
            // Let's implement: speed > 0 means process `speed` commits per frame.

            let end_idx = (commit_idx + speed).min(commits.len());
            for i in commit_idx..end_idx {
                let commit = &commits[i];
                for change in &commit.files {
                    let x = map_path(&change.path, width as usize) as f64;

                    if change.insertions > 0 {
                        let (axiom, rules) = hash_to_rules(&commit.hash);
                        garden.spawn_plant(x, &axiom, rules);
                    }

                    if change.deletions > 0 {
                        // Deletions create rain
                        garden.spawn_rain(x, change.deletions.min(50));
                    }
                }
            }
            commit_idx = end_idx;
        }

        garden.update(0.1);

        tui.terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(area);

            // Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Hanging Gardens of Entropy "))
                .x_bounds([0.0, width])
                .y_bounds([0.0, height]) // 0 is top visually if we map it right, but canvas 0,0 is bottom-left usually.
                // Wait, standard canvas: (0,0) is bottom-left.
                // Our plants start at y=0. If we want them hanging from top, we should start them at `height`.
                // But `sim.rs` assumes y=0 is top?
                // `sim.rs`: `p.y += p.vy * dt`. `p.y` increases. So y=0 is top, y=height is bottom.
                // Canvas: (0,0) is bottom-left. y increases UP.
                // So we need to flip Y coordinate when drawing.
                // Draw Y = Height - SimY.
                .paint(|ctx| {
                     // Draw Plants
                    for plant in &garden.plants {
                         let color = Color::Rgb(plant.color.0, plant.color.1, plant.color.2);
                         for line in &plant.lines {
                             let y1 = height - line.y1;
                             let y2 = height - line.y2;

                             // Don't draw if out of bounds (Canvas panics?)
                             ctx.draw(&CanvasLine {
                                 x1: line.x1,
                                 y1: y1,
                                 x2: line.x2,
                                 y2: y2,
                                 color,
                             });
                         }
                    }

                    // Draw Rain
                    for p in &garden.particles {
                        let y = height - p.y;
                        ctx.draw(&CanvasLine {
                            x1: p.x,
                            y1: y,
                            x2: p.x,
                            y2: y - 1.0, // Short tail
                            color: Color::Red,
                        });
                    }
                });

            f.render_widget(canvas, chunks[0]);

            // Status
            let status = if commit_idx < commits.len() {
                let c = &commits[commit_idx.saturating_sub(1)];
                format!("Commit: {} | Plants: {} | Rain: {} | Speed: {} | [SPACE] Pause [q] Quit",
                    c.short_hash, garden.plants.len(), garden.particles.len(), speed)
            } else {
                format!("DONE | Plants: {} | Rain: {} | [q] Quit", garden.plants.len(), garden.particles.len())
            };
            f.render_widget(Paragraph::new(status).block(Block::default().borders(Borders::ALL)), chunks[1]);

        })?;

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                 if key.kind == KeyEventKind::Press {
                     match key.code {
                         KeyCode::Char('q') => should_quit = true,
                         KeyCode::Char(' ') => paused = !paused,
                         KeyCode::Char('+') => speed += 1,
                         KeyCode::Char('-') => speed = speed.saturating_sub(1).max(1),
                         _ => {}
                     }
                 }
            }
        }

        if last_tick.elapsed() < tick_rate {
            std::thread::sleep(tick_rate - last_tick.elapsed());
        }
        last_tick = Instant::now();

        if should_quit {
            break;
        }
    }

    Ok(())
}

fn map_path(path: &str, width: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let h = hasher.finish();
    (h as usize) % width
}

fn hash_to_rules(hash: &str) -> (String, Vec<(char, &str)>) {
    let mut rules = Vec::new();
    let axiom = "F".to_string();

    // Use the first char of hash to determine rule set
    let c = hash.chars().next().unwrap_or('a');
    match (c as u32) % 4 {
        0 => rules.push(('F', "F[+F]F[-F]F")),
        1 => rules.push(('F', "FF-[-F+F+F]+[+F-F-F]")),
        2 => rules.push(('F', "F[+F]F[-F][F]")),
        _ => rules.push(('F', "FF+[+F-F-F]-[-F+F+F]")),
    }

    (axiom, rules)
}
