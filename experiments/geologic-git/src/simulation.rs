use crate::git_source::fetch_history;
use crate::layout::{layout_treemap, scan_repo, collect_leaves, Node};
use crate::terrain::Terrain;
use git_associates::model::Commit;
use macroquad::math::Rect;
use std::path::Path;

pub struct Simulation {
    pub terrain: Terrain,
    pub commits: Vec<Commit>,
    pub _layout_root: Node,
    pub layout_leaves: Vec<Node>,
    pub current_commit_index: usize,
    pub playing: bool,
    pub erosion_drops_per_frame: usize,
    pub total_uplift: f32,
}

impl Simulation {
    pub fn new(repo_path: &Path) -> Self {
        println!("Scanning repo at {:?}", repo_path);
        let mut root = scan_repo(repo_path);

        println!("Building layout...");
        // Map to 0..256 coordinates roughly?
        // Terrain is 256x256.
        // Let's use 0.0..256.0 range for layout area.
        let terrain_size = 256;
        layout_treemap(&mut root, Rect::new(0.0, 0.0, terrain_size as f32, terrain_size as f32));

        let leaves = collect_leaves(&root);
        println!("Layout complete. {} files mapped.", leaves.len());

        println!("Fetching git history...");
        // Fetch last 1000 commits? Or all?
        let commits = fetch_history(repo_path, 1000).unwrap_or_else(|e| {
            eprintln!("Failed to fetch history: {}", e);
            Vec::new()
        });
        // Reverse commits to replay from past to present?
        // fetch_history returns most recent first.
        // So we reverse it.
        let commits: Vec<Commit> = commits.into_iter().rev().collect();
        println!("Fetched {} commits.", commits.len());

        Self {
            terrain: Terrain::new(terrain_size, terrain_size),
            commits,
            _layout_root: root,
            layout_leaves: leaves,
            current_commit_index: 0,
            playing: false,
            erosion_drops_per_frame: 2000,
            total_uplift: 0.0,
        }
    }

    pub fn step(&mut self) {
        if self.playing && self.current_commit_index < self.commits.len() {
            let commit = &self.commits[self.current_commit_index];

            // Apply Uplift
            if let Some(_stats) = &commit.stats {
                // We use files list if available
                if !commit.files.is_empty() {
                    for file in &commit.files {
                        // Find file in layout
                        // Git path: "src/main.rs"
                        // Layout path: "./src/main.rs" or "src/main.rs" depending on how scan_repo was called.
                        // We check if layout path ends with git path.

                        let file_path = Path::new(&file.path);

                        // Heuristic matching
                        if let Some(node) = self.layout_leaves.iter().find(|n| {
                            n.path.ends_with(file_path)
                        }) {
                            // Uplift center of rect
                            let cx = node.rect.x + node.rect.w * 0.5;
                            let cy = node.rect.y + node.rect.h * 0.5;

                            // Uplift amount proportional to insertions
                            // Cap it?
                            // insertions can be huge.
                            // log scale?
                            let amount = (file.insertions as f32).ln().max(1.0) * 2.0;
                            // Also Radius proportional to rect size?
                            let radius = (node.rect.w.min(node.rect.h) * 0.5).max(2.0);

                            self.terrain.uplift(cx, cy, amount, radius);
                            self.total_uplift += amount;
                        }
                    }
                }
            }

            self.current_commit_index += 1;
        }

        // Apply Erosion (Constant time weathering)
        // Only if there is something to erode
        if self.total_uplift > 0.0 {
            self.terrain.erode(self.erosion_drops_per_frame);
        }
    }

    pub fn reset(&mut self) {
        let w = self.terrain.width;
        let h = self.terrain.height;
        self.terrain = Terrain::new(w, h);
        self.current_commit_index = 0;
        self.total_uplift = 0.0;
    }
}
