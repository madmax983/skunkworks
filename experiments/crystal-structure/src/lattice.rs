use std::path::Path;
use ratatui::style::Color;
use nalgebra::Point3;
use walkdir::WalkDir;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Atom {
    pub position: Point3<f64>,
    #[allow(dead_code)]
    pub original_index: Point3<i32>, // The integer lattice coordinates
    #[allow(dead_code)]
    pub file_path: String,
    #[allow(dead_code)]
    pub is_dir: bool,
    pub color: Color,
}

pub struct Lattice {
    pub atoms: Vec<Atom>,
    #[allow(dead_code)]
    pub center: Point3<f64>,
    #[allow(dead_code)]
    pub scale: f64,
}

impl Lattice {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            center: Point3::origin(),
            scale: 1.0,
        }
    }

    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut atoms = Vec::new();
        let entries: Vec<_> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().to_string_lossy().contains("/.")) // Skip hidden files roughly
            .collect();

        let count = entries.len();
        if count == 0 {
            return Ok(Self::new());
        }

        let s = (count as f64).powf(1.0/3.0).ceil() as i32;
        // Ensure s is at least 1
        let s = if s < 1 { 1 } else { s };

        let offset = -(s as f64) / 2.0;

        for (i, entry) in entries.iter().enumerate() {
            let idx = i as i32;
            let x_int = idx % s;
            let y_int = (idx / s) % s;
            let z_int = idx / (s * s);

            let x = x_int as f64 + offset;
            let y = y_int as f64 + offset;
            let z = z_int as f64 + offset;

            let is_dir = entry.file_type().is_dir();
            let path_str = entry.path().to_string_lossy().to_string();

            let color = if is_dir {
                Color::Blue
            } else if path_str.ends_with(".rs") {
                Color::Red
            } else if path_str.ends_with(".md") {
                Color::Yellow
            } else if path_str.ends_with(".toml") {
                Color::Green
            } else {
                Color::White
            };

            atoms.push(Atom {
                position: Point3::new(x, y, z),
                original_index: Point3::new(x_int, y_int, z_int),
                file_path: path_str,
                is_dir,
                color,
            });
        }

        // Calculate center (should be near 0,0,0 due to offset, but let's be precise if needed)
        // Actually, let's keep the center at 0,0,0 for rotation.

        Ok(Self {
            atoms,
            center: Point3::origin(),
            scale: 1.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_lattice_generation() -> Result<()> {
        // Create a temp directory with some files
        let temp_dir = tempfile::Builder::new().prefix("crystal_test").tempdir()?;
        let root = temp_dir.path();

        fs::write(root.join("a.txt"), "content")?;
        fs::write(root.join("b.rs"), "content")?;
        fs::create_dir(root.join("subdir"))?;
        fs::write(root.join("subdir/c.md"), "content")?;

        // root, a.txt, b.rs, subdir, c.md => 5 items

        let lattice = Lattice::from_dir(root)?;

        assert_eq!(lattice.atoms.len(), 5);

        // Check if atoms are arranged in a lattice (not all at 0,0,0)
        let unique_positions = lattice.atoms.iter()
            .map(|a| (a.original_index.x, a.original_index.y, a.original_index.z))
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(unique_positions.len(), 5);

        Ok(())
    }
}
