//! # Miller Lattice
//!
//! A procedural generator for visualizing hierarchical data structures (like a file system)
//! as a 3D crystalline lattice.
//!
//! This crate maps directories and files into 3D discrete space using Miller indices concepts.
//! Directories represent structural branch points that alter the normal vector of the growth plane,
//! while files and subdirectories are placed as `Atom`s around their parent node in a spiral pattern.
//!
//! ## Core Concepts
//!
//! - **[`LatticePoint`]**: A discrete integer coordinate `(x, y, z)` in 3D space.
//! - **[`Atom`]**: A single node in the crystal representing a file or directory.
//! - **[`Crystal`]**: The entire generated structure containing all atoms and their connectivity (bonds).

use anyhow::Result;
use cgmath::Vector3;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// A discrete coordinate in 3D integer space.
///
/// This serves as the fundamental positioning system for atoms in the crystal lattice.
/// It uses `i32` to prevent floating-point inaccuracies during structural generation.
///
/// # Examples
///
/// ```
/// use miller_lattice::LatticePoint;
///
/// let pt = LatticePoint::new(1, 2, -3);
/// assert_eq!(pt.x, 1);
/// assert_eq!(pt.z, -3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LatticePoint {
    /// The X coordinate.
    pub x: i32,
    /// The Y coordinate.
    pub y: i32,
    /// The Z coordinate.
    pub z: i32,
}

impl LatticePoint {
    /// Creates a new `LatticePoint` from x, y, z coordinates.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Converts the integer coordinates into a floating-point `Vector3`.
    ///
    /// This is useful for passing positions to rendering pipelines or physics engines.
    ///
    /// # Examples
    ///
    /// ```
    /// use miller_lattice::LatticePoint;
    ///
    /// let pt = LatticePoint::new(5, -2, 0);
    /// let vec = pt.to_vec3();
    /// assert_eq!(vec.x, 5.0);
    /// assert_eq!(vec.y, -2.0);
    /// ```
    pub fn to_vec3(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// Represents a single node (file or directory) within the crystal lattice.
///
/// An atom carries metadata about the file system entry it represents, including
/// its position in the 3D grid and the normal vector that defined its placement plane.
#[derive(Debug, Clone)]
pub struct Atom {
    /// The discrete 3D position of this atom in the crystal.
    pub position: LatticePoint,
    /// Whether this atom represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,
    /// The base name of the file or directory.
    pub name: String,
    /// The full path to the file system entry.
    pub path: PathBuf,
    /// The normal vector of the growth plane this atom was placed on.
    ///
    /// For directories, this normal dictates the orientation of its children.
    pub normal: Vector3<i32>,
}

/// A generated 3D crystalline lattice representing a file system hierarchy.
///
/// `Crystal` contains the list of all [`Atom`]s (files/directories) and the structural
/// `bonds` connecting parents to children.
///
/// # Examples
///
/// ```no_run
/// use miller_lattice::Crystal;
/// use std::path::Path;
///
/// let crystal = Crystal::build_from_path(Path::new("./src")).unwrap();
/// println!("Generated {} atoms with {} bonds.", crystal.atoms.len(), crystal.bonds.len());
/// ```
#[derive(Debug)]
pub struct Crystal {
    /// The flattened list of all atoms in the crystal.
    pub atoms: Vec<Atom>,
    /// Connections between atoms, representing parent-child relationships.
    ///
    /// Each tuple `(parent_idx, child_idx)` contains indices pointing into the `atoms` vector.
    pub bonds: Vec<(usize, usize)>,
    /// Fast spatial lookup mapping a discrete coordinate to an index in the `atoms` vector.
    pub lookup: HashMap<LatticePoint, usize>,
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new()
    }
}

impl Crystal {
    /// Creates a new, empty Crystal.
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    /// Recursively builds a crystal lattice from a given root path.
    ///
    /// This function performs a breadth-first traversal of the file system.
    ///
    /// - **Root Node**: Placed at `(0, 0, 0)`.
    /// - **Subdirectories**: Each new directory gets a semi-deterministic,
    ///   hash-based normal vector assigned to it, changing the "growth plane"
    ///   for its children.
    /// - **Files & Children**: Placed in an outward-expanding spiral around the parent,
    ///   respecting the parent's growth plane to prevent collisions.
    ///
    /// # Errors
    ///
    /// Returns an error if the root path cannot be read.
    pub fn build_from_path(root: &Path) -> Result<Self> {
        let mut crystal = Crystal::new();
        let mut occupied = HashSet::new();

        // Add root at origin
        let root_pos = LatticePoint::new(0, 0, 0);
        let root_normal = Vector3::new(0, 0, 1); // Default normal
        crystal.atoms.push(Atom {
            position: root_pos,
            is_dir: root.is_dir(),
            name: root
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: root.to_path_buf(),
            normal: root_normal,
        });
        occupied.insert(root_pos);
        crystal.lookup.insert(root_pos, 0);

        let mut queue = VecDeque::new();
        if root.is_dir() {
            queue.push_back((root.to_path_buf(), 0usize, root_normal));
        }

        while let Some((path, parent_idx, parent_normal)) = queue.pop_front() {
            let entries = match std::fs::read_dir(&path) {
                Ok(read_dir) => {
                    let mut entries: Vec<_> = read_dir.flatten().collect();
                    entries.sort_by_key(|e| e.file_name());
                    entries
                }
                Err(_) => continue,
            };

            let parent_pos = crystal.atoms[parent_idx].position;
            let (u, v) = get_basis_vectors(parent_normal);

            // Generate potential coordinates in a spiral
            let mut coords = VecDeque::new();
            // Max radius depends on number of entries, roughly sqrt(N)
            let max_r = ((entries.len() as f32).sqrt() as i32) + 2;

            // First point (0,0) is skipped because it's the parent itself?
            // Actually, the parent is already placed. We want to place children AROUND it.
            // So start r=1.
            for r in 1..=max_r + 5 {
                // Add padding
                // Perimeter of square radius r
                // Top: y=r, x from -r to r
                for dx in -r..=r {
                    coords.push_back((dx, r));
                }
                // Right: x=r, y from r-1 down to -r
                for dy in (-r..r).rev() {
                    coords.push_back((r, dy));
                }
                // Bottom: y=-r, x from r-1 down to -r
                for dx in (-r..r).rev() {
                    coords.push_back((dx, -r));
                }
                // Left: x=-r, y from -r+1 up to r-1
                for dy in (-r + 1)..r {
                    coords.push_back((-r, dy));
                }
            }

            for entry in entries {
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let name = entry.file_name().to_string_lossy().to_string();

                let mut found_pos = None;

                // Search for next free spot
                while let Some((du, dv)) = coords.pop_front() {
                    let pos = LatticePoint::new(
                        parent_pos.x + du * u.x + dv * v.x,
                        parent_pos.y + du * u.y + dv * v.y,
                        parent_pos.z + du * u.z + dv * v.z,
                    );

                    if !occupied.contains(&pos) {
                        found_pos = Some(pos);
                        break;
                    }
                }

                if let Some(pos) = found_pos {
                    occupied.insert(pos);
                    let idx = crystal.atoms.len();

                    // Determine new normal for directory
                    let new_normal = if is_dir {
                        get_dir_normal(&name, parent_normal)
                    } else {
                        parent_normal
                    };

                    crystal.atoms.push(Atom {
                        position: pos,
                        is_dir,
                        name: name.clone(),
                        path: entry.path(),
                        normal: new_normal,
                    });
                    crystal.bonds.push((parent_idx, idx));
                    crystal.lookup.insert(pos, idx);

                    if is_dir {
                        queue.push_back((entry.path(), idx, new_normal));
                    }
                }
            }
        }

        Ok(crystal)
    }
}

fn get_basis_vectors(n: Vector3<i32>) -> (Vector3<i32>, Vector3<i32>) {
    // Hardcoded basis for known normals to ensure integer arithmetic
    match (n.x, n.y, n.z) {
        (1, 0, 0) | (-1, 0, 0) => (Vector3::new(0, 1, 0), Vector3::new(0, 0, 1)),
        (0, 1, 0) | (0, -1, 0) => (Vector3::new(1, 0, 0), Vector3::new(0, 0, 1)),
        (0, 0, 1) | (0, 0, -1) => (Vector3::new(1, 0, 0), Vector3::new(0, 1, 0)),

        (1, 1, 0) | (-1, -1, 0) => (Vector3::new(0, 0, 1), Vector3::new(1, -1, 0)),
        (1, -1, 0) | (-1, 1, 0) => (Vector3::new(0, 0, 1), Vector3::new(1, 1, 0)),

        (1, 0, 1) | (-1, 0, -1) => (Vector3::new(0, 1, 0), Vector3::new(1, 0, -1)),
        (1, 0, -1) | (-1, 0, 1) => (Vector3::new(0, 1, 0), Vector3::new(1, 0, 1)),

        (0, 1, 1) | (0, -1, -1) => (Vector3::new(1, 0, 0), Vector3::new(0, 1, -1)),
        (0, 1, -1) | (0, -1, 1) => (Vector3::new(1, 0, 0), Vector3::new(0, 1, 1)),

        (1, 1, 1) | (-1, -1, -1) => (Vector3::new(1, -1, 0), Vector3::new(1, 1, -2)),
        // Fallback for unknown normals (should be covered by get_dir_normal)
        _ => (Vector3::new(1, 0, 0), Vector3::new(0, 1, 0)),
    }
}

fn get_dir_normal(name: &str, parent_normal: Vector3<i32>) -> Vector3<i32> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hasher);
    let h = hasher.finish();

    // Set of allowed normals
    let normals = vec![
        Vector3::new(1, 0, 0),
        Vector3::new(0, 1, 0),
        Vector3::new(0, 0, 1),
        Vector3::new(1, 1, 0),
        Vector3::new(1, -1, 0),
        Vector3::new(1, 0, 1),
        Vector3::new(1, 0, -1),
        Vector3::new(0, 1, 1),
        Vector3::new(0, 1, -1),
        Vector3::new(1, 1, 1),
    ];

    // Deterministic selection
    // We want to avoid parent_normal and -parent_normal to create branching
    for i in 0..normals.len() {
        let idx = ((h as usize) + i) % normals.len();
        let n = normals[idx];
        if n != parent_normal && n != -parent_normal {
            return n;
        }
    }

    normals[0] // Fallback
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_crystal_growth() -> Result<()> {
        let temp_dir = std::env::temp_dir().join("miller_fs_test_789");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir(&temp_dir)?;
        fs::create_dir(temp_dir.join("A"))?;
        fs::create_dir(temp_dir.join("B"))?;
        fs::write(temp_dir.join("file1.txt"), "hello")?;
        fs::write(temp_dir.join("A").join("file2.txt"), "world")?;

        let crystal = Crystal::build_from_path(&temp_dir)?;

        assert_eq!(crystal.atoms.len(), 5);
        assert_eq!(crystal.bonds.len(), 4);

        let mut positions = HashSet::new();
        for atom in &crystal.atoms {
            assert!(
                positions.insert(atom.position),
                "Duplicate position: {:?}",
                atom.position
            );
        }

        fs::remove_dir_all(&temp_dir)?;
        Ok(())
    }

    #[cfg(test)]
    mod havoc_tests {
        use super::super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            #[should_panic(expected = "attempt to negate with overflow")]
            fn test_havoc_get_dir_normal_neg_overflow(name in ".*") {
                let normal = Vector3::new(std::i32::MIN, 0, 0);
                let _ = get_dir_normal(&name, normal);
            }
        }
    }
}
