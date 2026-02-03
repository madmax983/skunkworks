//! # Dependency Fetcher
//!
//! This module is responsible for analyzing the Cargo workspace and converting crates into
//! physical "blocks" for the Jenga tower.
//!
//! It uses `cargo_metadata` to retrieve the list of workspace members and maps their properties
//! (like name length) to physical dimensions. If the metadata command fails (e.g., in a CI
//! environment without `cargo`), it gracefully falls back to a hardcoded set of "dummy" crates.
//!
//! ## Example
//!
//! ```rust,ignore
//! use crate::deps::{fetch_workspace_crates, CrateBlock};
//!
//! // Fetch crates (or dummies if outside a workspace)
//! let blocks = fetch_workspace_crates().unwrap();
//! assert!(!blocks.is_empty());
//!
//! // Inspect a block
//! let block = &blocks[0];
//! println!("Block: {} ({}x{})", block.name, block.width, block.height);
//! ```

use anyhow::Result;
use cargo_metadata::MetadataCommand;
use rand::Rng;

/// Represents a single block in the Jenga tower, corresponding to a Rust crate.
///
/// Each block has physical dimensions and a color. The width is derived from the crate's
/// name length (or other metrics in future versions), while the height is fixed.
///
/// # Examples
///
/// ```rust,ignore
/// use crate::deps::CrateBlock;
///
/// let block = CrateBlock {
///     name: "tokio".to_string(),
///     width: 5.0,
///     height: 1.0,
///     color: (255, 100, 0),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CrateBlock {
    /// The name of the crate (e.g., "serde").
    pub name: String,
    /// The width of the block in physical units.
    pub width: f32,
    /// The height of the block in physical units (usually fixed at 1.0).
    pub height: f32,
    /// The display color of the block (R, G, B).
    pub color: (u8, u8, u8),
}

/// Fetches workspace members and converts them into `CrateBlock`s.
///
/// This function attempts to run `cargo metadata` to discover crates in the current workspace.
///
/// # Logic
///
/// 1. Runs `cargo metadata`.
/// 2. Iterates over `workspace_members`.
/// 3. Calculates block width based on name length (clamped between 2.0 and 6.0).
/// 4. Assigns a random RGB color.
///
/// # Fallback
///
/// If `cargo metadata` fails (e.g., `cargo` is not installed or not in a workspace),
/// it returns a predefined list of "dummy" crates (core, std, ratatui, etc.) via [`dummy_crates`].
///
/// # Errors
///
/// Returns an `anyhow::Result` which is `Ok` even if `cargo metadata` fails (due to fallback).
/// It essentially never returns `Err` unless something catastrophic happens during fallback construction.
pub fn fetch_workspace_crates() -> Result<Vec<CrateBlock>> {
    // Try to run cargo metadata
    match MetadataCommand::new().exec() {
        Ok(metadata) => {
            let mut blocks = Vec::new();
            let mut rng = rand::thread_rng();

            for pkg in metadata.packages {
                // Filter for workspace members only?
                // Actually, let's include all deps in the tree if we were doing a full tree.
                // But for Jenga, let's just grab the workspace members or top-level deps.

                // Let's use workspace members for now as they are the "Experiment" blocks.
                if metadata.workspace_members.contains(&pkg.id) {
                    let name = pkg.name;
                    let width = (name.len() as f32 * 0.5).clamp(2.0, 6.0);

                    // Random color for variety
                    let r = rng.gen_range(50..255);
                    let g = rng.gen_range(50..255);
                    let b = rng.gen_range(50..255);

                    blocks.push(CrateBlock {
                        name,
                        width,
                        height: 1.0,
                        color: (r, g, b),
                    });
                }
            }

            // If empty (shouldn't be), return dummy
            if blocks.is_empty() {
                Ok(dummy_crates())
            } else {
                Ok(blocks)
            }
        }
        Err(_) => {
            // Fallback
            Ok(dummy_crates())
        }
    }
}

/// Generates a list of dummy crates for fallback purposes.
///
/// This ensures the application is playable even without a valid Cargo workspace context.
fn dummy_crates() -> Vec<CrateBlock> {
    vec![
        block("core", 4.0, (255, 0, 0)),
        block("std", 4.0, (0, 255, 0)),
        block("alloc", 4.0, (0, 0, 255)),
        block("ratatui", 5.0, (255, 255, 0)),
        block("syn", 3.0, (0, 255, 255)),
        block("quote", 3.0, (255, 0, 255)),
        block("serde", 4.0, (100, 100, 100)),
        block("tokio", 4.0, (200, 100, 50)),
        block("anyhow", 4.0, (50, 200, 100)),
        block("thiserror", 5.0, (100, 50, 200)),
        block("clap", 3.0, (200, 200, 50)),
        block("log", 3.0, (50, 50, 50)),
    ]
}

/// Helper to create a `CrateBlock` with less boilerplate.
fn block(name: &str, width: f32, color: (u8, u8, u8)) -> CrateBlock {
    CrateBlock {
        name: name.to_string(),
        width,
        height: 1.0,
        color,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_or_fallback() {
        let crates = fetch_workspace_crates();
        assert!(crates.is_ok());
        let list = crates.unwrap();
        assert!(!list.is_empty());
        // Check fields
        for c in list {
            assert!(c.width > 0.0);
            assert!(c.height > 0.0);
        }
    }
}
