//! 🧬 Splice: Cross git-associates × platter
//!
//! **Concept**: Git History Heatmap.
//!
//! **Lineage**:
//! - Parent A (git-associates): Provides native Git repository parsing, returning structured metadata on commits, insertions, and deletions.
//! - Parent B (platter): Provides the continuous 2D scalar field for accumulating heat/mass.
//!
//! **Novel trait**: The discrete git commits (from `git-associates`) are mapped onto a 2D scalar field (`platter`). The file changes act as massive heat pulses that saturate the continuous scalar field at specific regions representing the files.
//!
//! **Predicted Phenotype**: A visual "codebase radar" or commit heatmap. Discrete code changes (Insertions and Deletions) leave continuous, fading trails in the 2D grid. Areas of high development churn will show up as bright, persistent hotspots, while stable code will remain dark.

use anyhow::Result;
use git_associates::GitModel;
use platter::Platter;
use std::env;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() -> Result<()> {
    println!("🧬 git-platter: Git History Heatmap starting...\n");

    let current_dir = env::current_dir()?;
    let git_model = GitModel::open(current_dir)?;

    let history = git_model.history_with_diffs(50).unwrap_or_default();

    if history.is_empty() {
        println!("No git history found to process.");
        return Ok(());
    }

    println!("Mapping {} commits into the Platter field:\n", history.len());

    let width = 64;
    let height = 32;
    let mut platter = Platter::new(width, height);

    for commit in history.iter().rev() {
        if let Some(stats) = &commit.stats {
            let churn = (stats.insertions + stats.deletions) as f64;
            if churn == 0.0 {
                continue;
            }

            // Map each commit's author + message to a specific x, y region
            let hash = calculate_hash(&(&commit.author, &commit.message));

            // simple mapping to grid coordinates
            let base_x = (hash % width as u64) as usize;
            let base_y = ((hash / width as u64) % height as u64) as usize;

            // Apply a "pulse" of heat around the epicenter based on churn amount
            let intensity = (churn / 50.0).min(1.0); // Normalize churn

            for dy in -2..=2 {
                for dx in -2..=2 {
                    let px = base_x as isize + dx;
                    let py = base_y as isize + dy;

                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        let distance = ((dx*dx + dy*dy) as f64).sqrt();
                        if distance <= 2.0 {
                            let falloff = 1.0 - (distance / 2.0);
                            platter.accumulate(px as usize, py as usize, intensity * falloff);
                        }
                    }
                }
            }

            println!(
                "[{}] {} (+{}, -{}) => Heat pulse at ({}, {}) with max intensity {:.2}",
                commit.short_hash,
                truncate(&commit.message, 40),
                stats.insertions,
                stats.deletions,
                base_x,
                base_y,
                intensity
            );
        }

        // The field decays slightly after each commit, simulating the passing of time
        platter.decay(0.8);
    }

    // Print a rudimentary visual representation of the final heatmap
    println!("\nFinal Codebase Heatmap (Dense/Bright = Hot, Sparse/Dark = Cold):\n");
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    for y in 0..height {
        for x in 0..width {
            let val = platter.get_magnetism(x, y);
            let char_idx = ((val * 10.0) as usize).min(9);
            print!("{}", chars[char_idx]);
        }
        println!();
    }

    println!("\n🧬 Heatmap generation complete.");

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    let s = s.split('\n').next().unwrap_or("");
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}
