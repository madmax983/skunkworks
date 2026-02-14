use crate::map::{Terrain, Voxel, VoxelType};
use anyhow::Result;
use git2::{Repository, DiffOptions, Oid, Sort};
use macroquad::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub struct HistoryLoader {
    repo: Repository,
}

impl HistoryLoader {
    pub fn new(path: &str) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    pub fn load_into_terrain(&self, terrain: &mut Terrain) -> Result<()> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?; // Oldest first

        let mut previous_oid: Option<Oid> = None;

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;
            let tree = commit.tree()?;

            if let Some(parent_oid) = previous_oid {
                let parent_commit = self.repo.find_commit(parent_oid)?;
                let parent_tree = parent_commit.tree()?;

                let mut diff_opts = DiffOptions::new();
                let diff = self.repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_opts))?;

                for delta in diff.deltas() {
                    let path = delta.new_file().path().unwrap_or_else(|| delta.old_file().path().unwrap());
                    let (x, y) = self.hash_path(path, terrain.width, terrain.height);

                    // Determine Voxel Type based on commit message or diff
                    let message = commit.message().unwrap_or("").to_lowercase();
                    let voxel_type = if message.contains("fix") || message.contains("bug") {
                        VoxelType::Bone
                    } else if message.contains("remove") || message.contains("delete") {
                        VoxelType::Fossil
                    } else {
                        VoxelType::Sediment
                    };

                    let color = self.color_from_path(path, voxel_type);

                    // Add voxels. For now, 1 voxel per commit modification.
                    // Ideally we'd add more based on lines changed.
                    terrain.push_voxel(x, y, Voxel::new(voxel_type, color));
                }
            } else {
                // First commit - initialize files
                let mut diff_opts = DiffOptions::new();
                let diff = self.repo.diff_tree_to_tree(None, Some(&tree), Some(&mut diff_opts))?;
                 for delta in diff.deltas() {
                    let path = delta.new_file().path().unwrap();
                    let (x, y) = self.hash_path(path, terrain.width, terrain.height);
                    let color = self.color_from_path(path, VoxelType::Rock);
                    // Give it some base height
                    for _ in 0..5 {
                        terrain.push_voxel(x, y, Voxel::new(VoxelType::Rock, color));
                    }
                 }
            }

            previous_oid = Some(oid);
        }

        Ok(())
    }

    fn hash_path(&self, path: &Path, w: usize, h: usize) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let x = (hash as usize) % w;
        let y = ((hash >> 32) as usize) % h;
        (x, y)
    }

    fn color_from_path(&self, path: &Path, vtype: VoxelType) -> Color {
        match vtype {
            VoxelType::Bone => RED,
            VoxelType::Fossil => WHITE,
            VoxelType::Rock => DARKGRAY,
            VoxelType::Sediment => {
                // Color by extension
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    match ext {
                        "rs" => Color::new(0.8, 0.4, 0.0, 1.0), // Rust/Orange
                        "toml" => Color::new(0.9, 0.9, 0.2, 1.0), // Yellow
                        "md" => Color::new(0.2, 0.4, 0.8, 1.0), // Blue
                        "js" | "ts" => Color::new(0.2, 0.8, 0.2, 1.0), // Green
                        "json" => Color::new(0.8, 0.8, 0.2, 1.0),
                        _ => GRAY,
                    }
                } else {
                    GRAY
                }
            }
        }
    }
}
