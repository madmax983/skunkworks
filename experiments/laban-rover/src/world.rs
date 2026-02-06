use anyhow::{Context, Result};
use std::f64::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};
use tui_shared::math::Vec2;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum EntityType {
    Directory,
    File { size: u64 },
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub path: PathBuf,
    pub pos: Vec2,
    pub kind: EntityType,
    pub modified: SystemTime,
}

pub struct World {
    pub current_path: PathBuf,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::from("."),
            entities: Vec::new(),
        }
    }

    pub fn scan_path(&mut self, path: &Path) -> Result<()> {
        let canonical_path = fs::canonicalize(path).unwrap_or(path.to_path_buf());
        self.current_path = canonical_path.clone();
        self.entities.clear();

        let entries = fs::read_dir(&self.current_path)
            .context(format!("Failed to read directory {:?}", self.current_path))?;

        for (index, entry) in entries.enumerate() {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().to_string();
            let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

            let kind = if metadata.is_dir() {
                EntityType::Directory
            } else {
                EntityType::File {
                    size: metadata.len(),
                }
            };

            // Calculate position using Fermat's Spiral for deterministic, scattered placement
            // r = c * sqrt(n), theta = n * 137.5 degrees (Golden Angle)
            let n = index as f64;
            let c = 5.0; // Spacing factor
            let golden_angle = PI * (3.0 - (5.0f64).sqrt()); // ~2.399 radians
            let r = c * (n + 1.0).sqrt();
            let theta = n * golden_angle;

            let x = r * theta.cos();
            let y = r * theta.sin();

            self.entities.push(Entity {
                name,
                path,
                pos: Vec2::new(x, y),
                kind,
                modified,
            });
        }

        Ok(())
    }
}
