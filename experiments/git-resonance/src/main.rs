//! # 🧬 Splice: `git-resonance`
//!
//! **Lineage:** `crates/git-associates` × `crates/resonance-audio`
//!
//! **Concept:** Acoustic Git History. The discrete, structured data of a git repository's
//! commit history acts as a sequence of physical plucks within a 2D continuous acoustic wave tank.
//!
//! **Novel Trait:** Continuous Acoustic Translation of Git Intent.
//! By parsing the actual repository (insertions, deletions), we drop acoustic "plucks"
//! into the resonant wave tank. The size of the commit and the type of change determine
//! the strength, location, and polarity of the acoustic pressure wave, resulting in a
//! visual and auditory representation of the repository's evolution over time.

use crossbeam_channel::bounded;
use git_associates::GitModel;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::env;

const WIDTH: usize = 120;
const HEIGHT: usize = 120;

#[macroquad::main("Acoustic Git History")]
async fn main() -> anyhow::Result<()> {
    // Determine which repository to analyze
    let current_dir = env::current_dir().unwrap_or_else(|_| ".".into());
    let repo_path = std::env::args()
        .nth(1)
        .map(|p| std::path::PathBuf::from(p))
        .unwrap_or(current_dir);

    // Initialize the Audio Model
    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, snap_rx) = bounded(1);
    let mut audio_model = AudioModel::new(WIDTH, HEIGHT, cmd_rx, snap_tx, None);

    // Parse Git History
    let git_model = GitModel::open(&repo_path)?;
    let mut history = git_model.history_with_diffs(50)?;
    history.reverse(); // Play chronologically

    let mut commit_idx = 0;
    let mut last_commit_time = get_time();
    let commit_interval = 0.5; // seconds between processing commits

    let mut audio_buffer = vec![0.0; 1024];

    loop {
        let current_time = get_time();

        // Step 1: Process Audio Logic
        audio_model.process(&mut audio_buffer);

        // Step 2: Extract snapshot for rendering
        if let Ok(snapshot) = snap_rx.try_recv() {
            clear_background(BLACK);

            let cell_w = screen_width() / WIDTH as f32;
            let cell_h = screen_height() / HEIGHT as f32;

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let idx = y * WIDTH + x;
                    let pressure = snapshot.pressure[idx];

                    if pressure.abs() > 0.01 {
                        let val = (pressure.abs() * 5.0).clamp(0.0, 1.0);
                        let color = if pressure > 0.0 {
                            Color::new(0.0, val, val, 1.0)
                        } else {
                            Color::new(val, 0.0, val, 1.0)
                        };
                        draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, color);
                    }
                }
            }

            // Display current commit info
            if commit_idx < history.len() {
                let commit = &history[commit_idx];
                draw_text(
                    &format!("Commit: {} - {}", commit.short_hash, commit.message),
                    10.0,
                    20.0,
                    20.0,
                    WHITE,
                );
            } else {
                draw_text("End of history", 10.0, 20.0, 20.0, WHITE);
            }
        }

        // Step 3: Trigger acoustic events from git history
        if current_time - last_commit_time > commit_interval {
            if commit_idx < history.len() {
                let commit = &history[commit_idx];

                if let Some(stats) = &commit.stats {
                    let total_changes = stats.insertions + stats.deletions;
                    if total_changes > 0 {
                        // Use commit hash bytes to pick a location
                        let hash_bytes = commit.hash.as_bytes();
                        let loc_x = (hash_bytes[0] as usize % (WIDTH - 10)) + 5;
                        let loc_y = (hash_bytes[1] as usize % (HEIGHT - 10)) + 5;

                        // Calculate strength based on lines changed
                        let base_strength = (total_changes as f32).sqrt().clamp(0.1, 5.0);

                        // Insertions create positive pressure, deletions negative pressure
                        let net_change = stats.insertions as f32 - stats.deletions as f32;
                        let polarity = if net_change >= 0.0 { 1.0 } else { -1.0 };

                        let pluck_strength = base_strength * polarity;

                        let _ = cmd_tx.send(AudioCommand::Pluck {
                            x: loc_x,
                            y: loc_y,
                            strength: pluck_strength,
                        });
                    }
                }

                commit_idx += 1;
            }
            last_commit_time = current_time;
        }

        next_frame().await;
    }
}
