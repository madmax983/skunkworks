use anyhow::Result;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Atom {
    pub position: IVec3,
    pub miller_index: IVec3,
    pub size: u64,
    pub is_dir: bool,
    pub path: PathBuf,
    pub depth: i32,
}

#[derive(Debug, Resource)]
pub struct Crystal {
    pub atoms: Vec<Atom>,
    pub center_of_mass: Vec3,
}

impl Crystal {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            center_of_mass: Vec3::ZERO,
        }
    }

    pub fn build_hopper_crystal(root: &Path) -> Result<Self> {
        let mut atoms = Vec::new();
        let root_depth = root.components().count();

        // 1. Collect all entries
        let mut entries: Vec<_> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        // 2. Sort by depth (folders first) then by size descending
        entries.sort_by(|a, b| {
            let depth_a = a.depth();
            let depth_b = b.depth();
            match depth_a.cmp(&depth_b) {
                Ordering::Equal => {
                    let size_a = a.metadata().map(|m| m.len()).unwrap_or(0);
                    let size_b = b.metadata().map(|m| m.len()).unwrap_or(0);
                    size_b.cmp(&size_a)
                }
                other => other,
            }
        });

        use std::collections::HashMap;
        let mut dir_centers: HashMap<PathBuf, IVec3> = HashMap::new();
        // Insert root. Parent of root entries is root.
        dir_centers.insert(root.to_path_buf(), IVec3::ZERO);

        let mut dir_counts: HashMap<PathBuf, i32> = HashMap::new();

        for entry in entries {
            let path = entry.path().to_path_buf();
            // Parent is root if we are at root, or the actual parent
            let parent = path.parent().unwrap_or(root).to_path_buf();

            // If the parent is not in our map (e.g. because we are strictly below root or sorting issue),
            // fallback to root.
            let parent_center = *dir_centers.get(&parent).unwrap_or(&IVec3::ZERO);

            let is_dir = entry.file_type().is_dir();
            let depth = (path.components().count().saturating_sub(root_depth)) as i32;
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

            // Calculate position relative to parent center using spiral logic
            let count = *dir_counts.entry(parent.clone()).or_insert(0);
            dir_counts.insert(parent.clone(), count + 1);

            let (lx, lz) = spiral_coord(count);

            // Global Position
            // Use directory depth for Y to create layers.
            let global_y = depth * 2; // Spacing
            let global_pos = IVec3::new(
                parent_center.x + lx,
                global_y,
                parent_center.z + lz
            );

            if is_dir {
                dir_centers.insert(path.clone(), global_pos);
            }

            let miller = calculate_miller(lx, lz);

            atoms.push(Atom {
                position: global_pos,
                miller_index: miller,
                size,
                is_dir,
                path,
                depth,
            });
        }

        let sum: Vec3 = atoms.iter().map(|a| a.position.as_vec3()).sum();
        let center_of_mass = if atoms.is_empty() { Vec3::ZERO } else { sum / atoms.len() as f32 };

        Ok(Crystal {
            atoms,
            center_of_mass,
        })
    }
}

// O(1) Mapping from index n to spiral coordinate (x, z)
fn spiral_coord(n: i32) -> (i32, i32) {
    if n == 0 { return (0, 0); }

    // Ring k
    let k = (((n as f32).sqrt() + 1.0) / 2.0).floor() as i32;
    if k == 0 { return (0, 0); } // Should be covered by n=0 check, but safety first

    let start = (2 * k - 1).pow(2);
    let offset = n - start;
    let edge = 2 * k;

    if offset < edge {
        (k, -k + 1 + offset) // Right side, moving Up
    } else if offset < 2 * edge {
        (k - (offset - edge + 1), k) // Top side, moving Left
    } else if offset < 3 * edge {
        (-k, k - (offset - 2 * edge + 1)) // Left side, moving Down
    } else {
        (-k + (offset - 3 * edge + 1), -k) // Bottom side, moving Right
    }
}

fn calculate_miller(x: i32, z: i32) -> IVec3 {
    if x.abs() > z.abs() {
        if x > 0 { IVec3::new(1, 0, 0) } else { IVec3::new(-1, 0, 0) }
    } else {
        if z > 0 { IVec3::new(0, 0, 1) } else { IVec3::new(0, 0, -1) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;

    #[test]
    fn test_spiral_coord_uniqueness() {
        let mut seen = HashSet::new();
        for n in 0..100 {
            let pos = spiral_coord(n);
            assert!(seen.insert(pos), "Duplicate position found at n={}: {:?}", n, pos);
        }
    }

    #[test]
    fn test_spiral_coord_values() {
        // n=0 -> (0,0)
        assert_eq!(spiral_coord(0), (0, 0));

        // n=1 -> k=1 -> (1, 0)
        assert_eq!(spiral_coord(1), (1, 0));

        // n=2 -> k=1 -> (1, 1)
        assert_eq!(spiral_coord(2), (1, 1));

        // n=3 -> (0, 1)
        assert_eq!(spiral_coord(3), (0, 1));

        // n=8 -> (1, -1) (closing ring 1)
        assert_eq!(spiral_coord(8), (1, -1));

        // n=9 -> (2, -1) (start ring 2)
        assert_eq!(spiral_coord(9), (2, -1));
    }

    #[test]
    fn test_crystal_build() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("bismuth_fs_test");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        // Root file
        fs::write(temp_dir.join("root.txt"), "root")?;

        // Subdir A
        let dir_a = temp_dir.join("A");
        fs::create_dir(&dir_a)?;
        fs::write(dir_a.join("a1.txt"), "a1")?;
        fs::write(dir_a.join("a2.txt"), "a2")?;

        // Subdir B
        let dir_b = temp_dir.join("B");
        fs::create_dir(&dir_b)?;
        fs::write(dir_b.join("b1.txt"), "b1")?;

        let crystal = Crystal::build_hopper_crystal(&temp_dir)?;

        // Root + root.txt + A + a1 + a2 + B + b1 = 7 atoms?
        assert_eq!(crystal.atoms.len(), 7);

        // Check uniqueness of positions
        let mut positions = HashSet::new();
        for atom in &crystal.atoms {
            assert!(positions.insert(atom.position), "Duplicate position: {:?}", atom.position);
        }

        // Check depths
        let root_atom = crystal.atoms.iter().find(|a| a.path == temp_dir).unwrap();
        assert_eq!(root_atom.depth, 0);

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;
        Ok(())
    }
}
