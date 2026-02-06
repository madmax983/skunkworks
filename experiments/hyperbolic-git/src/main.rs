use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use tui_shared::Tui;

mod graph;
mod ui;

use graph::CommitGraph;
use ui::draw_ui;
// use poincare_disk::Point; // Not directly used except in type inference

fn main() -> Result<()> {
    // Find git repo
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());

    // Check if we can load graph before initializing TUI to avoid messing up terminal on error
    let mut graph = match CommitGraph::new(&path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error loading git repo at '{}': {}", path, e);
            eprintln!("Usage: hyperbolic-git <path-to-repo>");
            return Ok(());
        }
    };

    let mut tui = Tui::init()?;

    let mut focus_oid = graph.head_oid;
    let mut selected_oid = focus_oid;

    loop {
        // Layout
        // If layout fails (e.g. git error), we should probably exit or show error.
        // For now unwrap/expect inside Tui loop is risky, better handle it.
        let layout_res = graph.layout(focus_oid, 4);

        match layout_res {
            Ok(layout) => {
                 tui.terminal.draw(|f| {
                    draw_ui(f, &graph, &layout, focus_oid, Some(selected_oid));
                })?;

                if event::poll(Duration::from_millis(50))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => break,
                                KeyCode::Enter => {
                                    focus_oid = selected_oid;
                                }
                                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                                    // Navigation logic
                                    let current_pos = layout.iter().find(|(id, _)| *id == selected_oid)
                                        .map(|(_, p)| *p).unwrap_or(num_complex::Complex::new(0.0, 0.0));

                                    let dir = match key.code {
                                        KeyCode::Up => num_complex::Complex::new(0.0, 1.0),
                                        KeyCode::Down => num_complex::Complex::new(0.0, -1.0),
                                        KeyCode::Left => num_complex::Complex::new(-1.0, 0.0),
                                        KeyCode::Right => num_complex::Complex::new(1.0, 0.0),
                                        _ => num_complex::Complex::new(0.0, 0.0),
                                    };

                                    // Find best candidate
                                    let mut best = None;
                                    let mut max_score = -f64::INFINITY;

                                    for (oid, pos) in &layout {
                                        if *oid == selected_oid { continue; }

                                        let diff = pos - current_pos;
                                        let dist = diff.norm();
                                        if dist < 0.01 { continue; }

                                        // Cosine similarity
                                        let dir_score = (diff.re * dir.re + diff.im * dir.im) / dist;

                                        // Filter by cone (> 45 degrees approx)
                                        if dir_score > 0.5 {
                                            // Heuristic: Prefer nodes closer in distance among those in direction
                                            // Score = Alignment - Distance_Penalty
                                            let score = dir_score - dist * 0.5;

                                            if score > max_score {
                                                max_score = score;
                                                best = Some(*oid);
                                            }
                                        }
                                    }

                                    if let Some(oid) = best {
                                        selected_oid = oid;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
            Err(e) => {
                // If layout fails, just break loop and print error
                tui.exit()?;
                eprintln!("Graph layout error: {}", e);
                break;
            }
        }
    }

    tui.exit()?;
    Ok(())
}
