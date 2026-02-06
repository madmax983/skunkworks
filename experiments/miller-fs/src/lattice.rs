use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LatticePoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl LatticePoint {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn neighbors(&self) -> Vec<LatticePoint> {
        vec![
            LatticePoint::new(self.x + 1, self.y, self.z),
            LatticePoint::new(self.x - 1, self.y, self.z),
            LatticePoint::new(self.x, self.y + 1, self.z),
            LatticePoint::new(self.x, self.y - 1, self.z),
            LatticePoint::new(self.x, self.y, self.z + 1),
            LatticePoint::new(self.x, self.y, self.z - 1),
        ]
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Atom {
    pub position: LatticePoint,
    pub is_dir: bool,
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Crystal {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<(usize, usize)>, // indices into atoms
    pub lookup: HashMap<LatticePoint, usize>,
}

impl Crystal {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn build_from_path(root: &Path) -> Result<Self> {
        let mut crystal = Crystal::new();
        let mut occupied = HashSet::new();

        // Add root
        let root_pos = LatticePoint::new(0, 0, 0);
        crystal.atoms.push(Atom {
            position: root_pos,
            is_dir: root.is_dir(),
            name: root
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: root.to_path_buf(),
        });
        occupied.insert(root_pos);
        crystal.lookup.insert(root_pos, 0);

        let mut queue = VecDeque::new();
        if root.is_dir() {
            queue.push_back((root.to_path_buf(), 0usize));
        }

        while let Some((path, parent_idx)) = queue.pop_front() {
            // Read children
            let mut entries = Vec::new();
            match std::fs::read_dir(&path) {
                Ok(read_dir) => {
                    for entry in read_dir.flatten() {
                        entries.push(entry);
                    }
                }
                Err(_) => continue,
            }

            // Sort for determinism
            entries.sort_by_key(|e| e.file_name());

            let parent_pos = crystal.atoms[parent_idx].position;

            for entry in entries {
                // Find nearest free spot
                let mut search_queue = VecDeque::new();
                search_queue.push_back(parent_pos);
                let mut visited_search = HashSet::new();
                visited_search.insert(parent_pos);

                let mut found_pos = None;

                while let Some(pos) = search_queue.pop_front() {
                    // Check neighbors in a deterministic order
                    let neighbors = pos.neighbors();
                    for n in neighbors {
                        if !occupied.contains(&n) {
                            found_pos = Some(n);
                            break;
                        }
                        if visited_search.insert(n) {
                            search_queue.push_back(n);
                        }
                    }
                    if found_pos.is_some() {
                        break;
                    }
                }

                if let Some(pos) = found_pos {
                    occupied.insert(pos);
                    let idx = crystal.atoms.len();
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                    crystal.atoms.push(Atom {
                        position: pos,
                        is_dir,
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry.path(),
                    });
                    crystal.bonds.push((parent_idx, idx));
                    crystal.lookup.insert(pos, idx);

                    if is_dir {
                        queue.push_back((entry.path(), idx));
                    }
                }
            }
        }

        Ok(crystal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_crystal_growth() -> Result<()> {
        // Create a temporary directory structure
        let temp_dir = std::env::temp_dir().join("miller_fs_test_123");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir(&temp_dir)?;
        fs::create_dir(temp_dir.join("A"))?;
        fs::create_dir(temp_dir.join("B"))?;
        fs::write(temp_dir.join("file1.txt"), "hello")?;
        fs::write(temp_dir.join("A").join("file2.txt"), "world")?;

        let crystal = Crystal::build_from_path(&temp_dir)?;

        // Assertions
        // Expect: Root + A + B + file1 + file2 = 5 atoms
        assert_eq!(
            crystal.atoms.len(),
            5,
            "Expected 5 atoms, found {}",
            crystal.atoms.len()
        );

        // Check uniqueness of positions
        let mut positions = HashSet::new();
        for atom in &crystal.atoms {
            assert!(
                positions.insert(atom.position),
                "Duplicate position found: {:?}",
                atom.position
            );
        }

        // Check connectivity (every atom except root has a bond)
        // Note: Bonds store (parent, child). Total bonds should be atoms.len() - 1
        assert_eq!(crystal.bonds.len(), 4);

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;
        Ok(())
    }
}
