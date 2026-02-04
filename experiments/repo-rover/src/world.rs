use anyhow::{Context, Result};
use std::f64::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};
use tui_shared::math::Vec2;

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
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_scan_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create some files and folders
        fs::create_dir(dir_path.join("subdir"))?;
        File::create(dir_path.join("file1.txt"))?;
        File::create(dir_path.join("file2.txt"))?;

        let mut world = World::new();
        world.scan_path(dir_path)?;

        assert_eq!(world.entities.len(), 3);

        let file_names: Vec<String> = world.entities.iter().map(|e| e.name.clone()).collect();
        assert!(file_names.contains(&"subdir".to_string()));
        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(file_names.contains(&"file2.txt".to_string()));

        // Check positions are distinct
        let p1 = world.entities[0].pos;
        let p2 = world.entities[1].pos;
        assert!(p1.distance(p2) > 0.1);

        Ok(())
    }
}
