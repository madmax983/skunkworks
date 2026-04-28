//! 🧬 Splice: Cross git-associates × poincare-disk
//!
//! **Concept**: Hyperbolic Codebase Morphogenesis.
//!
//! **Lineage**:
//! - Parent A (git-associates): Provides native Git repository parsing, returning structured metadata on commits, insertions, and deletions.
//! - Parent B (poincare-disk): Provides the non-Euclidean mathematical environment for hyperbolic coordinate projection and geometry constraints.
//!
//! **Novel trait**: Projecting the codebase commit metadata directly onto a Poincaré disk layout. High-churn files/commits push outward toward infinity (the boundary), while stable files reside in the Euclidean center.
//!
//! **Predicted Phenotype**: An organic, relativistic mapping of a codebase. The most actively developed components expand exponentially toward the boundary, showing incredibly dense chaotic clusters of commits, while the stable core remains sparse and central.

use anyhow::Result;
use git_associates::GitModel;
use poincare_disk::{Point, mobius_add};
use rand::Rng;
use std::env;

fn main() -> Result<()> {
    println!("🧬 git-poincare: Hyperbolic Codebase Morphogenesis starting...\n");

    // Initialize the Git Model on the current repository
    let current_dir = env::current_dir()?;
    let git_model = GitModel::open(current_dir)?;

    // Fetch the recent history with diff stats to get the size of commits
    let history = git_model.history_with_diffs(50).unwrap_or_default();

    if history.is_empty() {
        println!("No git history found to process.");
        return Ok(());
    }

    println!("Mapping {} commits into the Poincaré Disk:\n", history.len());

    // We will start mapping from the center
    let mut current_point = Point::new(0.0, 0.0);
    let mut rng = rand::thread_rng();

    for commit in history.iter().rev() {
        let (insertions, deletions) = if let Some(stats) = &commit.stats {
            (stats.insertions, stats.deletions)
        } else {
            (0, 0)
        };

        // Total "mass" or "churn" of the commit
        let churn = (insertions + deletions) as f64;

        // If it's an empty commit, we skip projecting it to save mathematical ops
        if churn == 0.0 {
            continue;
        }

        // We want massive commits to push further towards the boundary.
        // We'll calculate a step size that pushes the point based on the churn size.
        // Step size must be < 1.0 to stay within the unit disk.
        // We use a logarithmic scale to prevent massive commits from instantly hitting the boundary.
        let step_magnitude = (1.0 - (1.0 / (1.0 + (churn / 100.0).ln()))).max(0.01).min(0.99);

        // We pick a random angle to spread the commits out, creating an organic cluster
        let angle = rng.gen_range(0.0..std::f64::consts::TAU);

        let step_point = Point::new(
            step_magnitude * angle.cos(),
            step_magnitude * angle.sin(),
        );

        // Hyperbolic displacement: Instead of standard vector addition, we use mobius_add.
        // This ensures the point will *never* exceed the boundary |z| < 1, but will get exponentially compressed.
        current_point = mobius_add(current_point, step_point);

        // Display the mapping
        println!(
            "[{}] {} (+{}, -{})",
            commit.short_hash,
            truncate(&commit.message, 40),
            insertions,
            deletions
        );
        println!(
            "   -> Hyperbolic Coordinates: (x: {:.4}, y: {:.4}), Magnitude: {:.4}",
            current_point.re,
            current_point.im,
            current_point.norm()
        );
    }

    println!("\n🧬 Morphogenesis complete. The repository history is now contained within the unit disk.");

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    let s = s.split('\n').next().unwrap_or(""); // Get just the first line
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}
