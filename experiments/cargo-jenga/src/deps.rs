use anyhow::Result;
use cargo_metadata::MetadataCommand;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct CrateBlock {
    pub name: String,
    pub width: f32,  // Based on name length or deps count
    pub height: f32, // Fixed
    pub color: (u8, u8, u8),
}

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
