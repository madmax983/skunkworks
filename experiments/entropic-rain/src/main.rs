use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod sim;
mod terrain;

use sim::Simulation;
use terrain::Terrain;
use tui_shared::Tui;
use git_associates::{GitModel, Commit, FileChange};

fn main() -> Result<()> {
    // Terminal setup
    let mut tui = Tui::init()?;

    // App state
    // We try to load history. If it fails (e.g. not a git repo), we fail gracefully-ish.
    let mut commits = match GitModel::open(".") {
        Ok(model) => {
            match model.history_with_diffs(usize::MAX) {
                Ok(mut c) => {
                    c.reverse(); // Sort Oldest to Newest
                    c
                },
                Err(_) => vec![],
            }
        },
        Err(_) => vec![],
    };

    if commits.is_empty() {
        // Create dummy data for demo if no git history found
        commits.push(Commit {
            hash: "DEMO01".to_string(),
            short_hash: "DEMO01".to_string(),
            author: "Demo".to_string(),
            message: "Demo".to_string(),
            timestamp: chrono::Utc::now(),
            parents: vec![],
            stats: None,
            files: vec![
                FileChange {
                    path: "mountain.rs".to_string(),
                    extension: "rs".to_string(),
                    insertions: 50,
                    deletions: 0,
                    is_binary: false,
                    hunks: vec![],
                },
                FileChange {
                    path: "valley.rs".to_string(),
                    extension: "rs".to_string(),
                    insertions: 20,
                    deletions: 0,
                    is_binary: false,
                    hunks: vec![],
                },
            ],
        });
        commits.push(Commit {
            hash: "DEMO02".to_string(),
            short_hash: "DEMO02".to_string(),
            author: "Demo".to_string(),
            message: "Demo".to_string(),
            timestamp: chrono::Utc::now(),
            parents: vec![],
            stats: None,
            files: vec![FileChange {
                path: "mountain.rs".to_string(),
                extension: "rs".to_string(),
                insertions: 0,
                deletions: 20,
                is_binary: false,
                hunks: vec![],
            }],
        });
    }

    let size = tui.terminal.size()?;
    let width = size.width as usize;
    let height = (size.height.saturating_sub(4)) as usize;

    let mut terrain = Terrain::new(width);
    let mut sim = Simulation::new(width, height);

    let mut commit_idx = 0;
    let mut speed = 5; // Default speed
    let mut paused = false;
    let mut should_quit = false;

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        // Update logic
        if !paused && commit_idx < commits.len() {
            let end_idx = (commit_idx + speed).min(commits.len());
            for i in commit_idx..end_idx {
                let commit = &commits[i];
                for change in &commit.files {
                    let path_str = &change.path;
                    let x = map_path(path_str, width);

                    // Additions -> Uplift
                    if change.insertions > 0 {
                        // Limit height growth scaling
                        terrain.uplift(x, (change.insertions as f64 * 0.2).min(5.0));
                    }

                    // Deletions -> Rain
                    if change.deletions > 0 {
                        // Cap spawn count to avoid explosion
                        sim.spawn(x, change.deletions.min(20));
                    }
                }
            }
            commit_idx = end_idx;
        }

        // Physics Update
        sim.update(0.05, &mut terrain);

        // Draw
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            // Info
            let info_text = if commit_idx < commits.len() {
                let c = &commits[commit_idx.saturating_sub(1)];
                format!(
                    "Commit: {} | Rain: {} | Speed: {} | [SPACE] Pause [+/-] Speed [q] Quit",
                    &c.short_hash,
                    sim.particles.len(),
                    speed
                )
            } else {
                format!("DONE | Final Particles: {} | [q] Quit", sim.particles.len())
            };

            f.render_widget(
                Paragraph::new(info_text).block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );

            // Canvas
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Entropic Rain "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    // Draw Terrain
                    for x in 0..width {
                        let h = terrain.get_height(x);
                        if h > 0.1 {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: 0.0,
                                width: 1.0,
                                height: h,
                                color: Color::Green,
                            });
                        }
                    }

                    // Draw Particles
                    for p in &sim.particles {
                        let cy = height as f64 - p.y;
                        if cy >= 0.0 && cy <= height as f64 {
                            ctx.draw(&Rectangle {
                                x: p.x,
                                y: cy,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Red,
                            });
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);
        })?;

        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => should_quit = true,
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

    drop(tui);

    Ok(())
}

fn map_path(path: &str, width: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let h = hasher.finish();
    (h as usize) % width
}
