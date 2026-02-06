use anyhow::{Context, Result};
use git2::{Oid, Repository};
use poincare_disk::{Mobius, Point};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use num_complex::Complex;

#[derive(Clone, Debug)]
pub struct CommitData {
    pub oid: Oid,
    pub message: String,
    pub author: String,
    pub parents: Vec<Oid>,
    pub children: Vec<Oid>,
}

pub struct CommitGraph {
    pub repo: Repository,
    pub commits: HashMap<Oid, CommitData>,
    pub head_oid: Oid,
}

impl CommitGraph {
    pub fn new(path: &str) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open git repo")?;
        let head_oid = {
            let head = repo.head().context("HEAD not found")?;
            let head_commit = head.peel_to_commit().context("HEAD is not a commit")?;
            head_commit.id()
        };

        let mut graph = Self {
            repo,
            commits: HashMap::new(),
            head_oid,
        };

        graph.load_commit(head_oid)?;
        Ok(graph)
    }

    pub fn load_commit(&mut self, oid: Oid) -> Result<()> {
        if self.commits.contains_key(&oid) {
            return Ok(());
        }

        let commit = self.repo.find_commit(oid)?;
        let message = commit.summary().unwrap_or("").to_string();
        let author = commit.author().name().unwrap_or("").to_string();

        let parents: Vec<Oid> = commit.parents().map(|p| p.id()).collect();

        let data = CommitData {
            oid,
            message,
            author,
            parents: parents.clone(),
            children: Vec::new(),
        };

        self.commits.insert(oid, data);
        Ok(())
    }

    pub fn ensure_neighbors_loaded(&mut self, oid: Oid) -> Result<()> {
        let parents = if let Some(data) = self.commits.get(&oid) {
             data.parents.clone()
        } else {
            self.load_commit(oid)?;
            self.commits.get(&oid).unwrap().parents.clone()
        };

        for p in parents {
            if !self.commits.contains_key(&p) {
                self.load_commit(p)?;
            }
            if let Some(parent_data) = self.commits.get_mut(&p) {
                if !parent_data.children.contains(&oid) {
                    parent_data.children.push(oid);
                }
            }
        }
        Ok(())
    }

    pub fn layout(&mut self, focus_oid: Oid, depth_limit: usize) -> Result<Vec<(Oid, Point)>> {
        self.ensure_neighbors_loaded(focus_oid)?;

        let mut positions = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // (oid, transform_from_origin, depth, parent_angle_local)
        queue.push_back((focus_oid, Mobius::identity(), 0, None));
        visited.insert(focus_oid);

        let step_a = 0.45; // Euclidean distance step size (tanh(d/2))

        while let Some((u, trans, depth, parent_angle)) = queue.pop_front() {
            // Apply transform to origin to get position
            let pos = trans.apply(Complex::new(0.0, 0.0));
            positions.push((u, pos));

            if depth >= depth_limit {
                continue;
            }

            if let Err(_) = self.ensure_neighbors_loaded(u) {
                continue;
            }

            let data = self.commits.get(&u).unwrap();
            let mut neighbors = Vec::new();
            // Prioritize parents then children? Mixed is fine.
            for p in &data.parents { if !visited.contains(p) { neighbors.push(*p); } }
            for c in &data.children { if !visited.contains(c) { neighbors.push(*c); } }

            if neighbors.is_empty() {
                continue;
            }

            let count = neighbors.len();

            let angles: Vec<f64> = if let Some(p_angle) = parent_angle {
                // Distribute away from parent
                // Wedge of exclusion: 90 degrees (PI/2)
                let spread = 2.0 * PI - (PI / 2.0); // 270 degrees available
                let start_angle = p_angle + PI - (spread / 2.0);

                let step = if count > 1 { spread / (count as f64 - 1.0) } else { 0.0 };
                (0..count).map(|i| start_angle + step * (i as f64)).collect()
            } else {
                // Root: 360 degrees
                let step = 2.0 * PI / (count as f64);
                (0..count).map(|i| step * (i as f64)).collect()
            };

            for (i, &neighbor) in neighbors.iter().enumerate() {
                visited.insert(neighbor);
                let theta = angles[i];

                let local_pt = Complex::from_polar(step_a, theta);
                let move_trans = Mobius::translation(local_pt);
                // The new transform maps origin -> u -> neighbor
                // But `trans` maps origin -> u.
                // We want `new_trans` such that `new_trans(0) = neighbor`.
                // `trans(move_trans(0))` = `trans(local_pt)` = absolute position of neighbor.
                // Yes, `then` composes as `self(other(z))`.
                // So `trans.then(&move_trans)` does `trans(move_trans(z))`.
                let new_trans = trans.then(&move_trans);

                let next_parent_angle = theta + PI;
                queue.push_back((neighbor, new_trans, depth + 1, Some(next_parent_angle)));
            }
        }

        Ok(positions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_graph_init() {
        // Try to open the current repo
        // This test runs in target/debug/deps, so we need to find the repo root.
        // Assuming we are running in the repo root or can find it.
        // But for `cargo test`, the CWD is the package root `experiments/hyperbolic-git` usually, or workspace root.
        // Let's try "../../".

        let path = if Path::new(".git").exists() {
            "."
        } else if Path::new("../../.git").exists() {
            "../.."
        } else {
            // Can't find repo, skip test or panic?
            // In CI environment, repo should be there.
            "."
        };

        if let Ok(graph) = CommitGraph::new(path) {
            println!("Loaded graph with HEAD: {}", graph.head_oid);
            assert!(graph.commits.contains_key(&graph.head_oid));
        } else {
            // Might fail if not running in a git repo (e.g. exported source), but we are in a git repo.
            // If it fails, print why.
            println!("Skipping test: Git repo not found at {}", path);
        }
    }

    #[test]
    fn test_layout() {
        let path = if Path::new("../../.git").exists() { "../.." } else { "." };
        if let Ok(mut graph) = CommitGraph::new(path) {
             let layout = graph.layout(graph.head_oid, 2).unwrap();
             assert!(!layout.is_empty());
             // Head should be at 0,0 (approx)
             let (head_oid, head_pos) = layout[0];
             assert_eq!(head_oid, graph.head_oid);
             assert!(head_pos.norm() < 1e-9);

             if layout.len() > 1 {
                 // Check second node distance
                 let (_, pos) = layout[1];
                 assert!(pos.norm() > 0.3); // Should be around 0.45
             }
        }
    }
}
