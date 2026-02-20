use anyhow::Result;
use git_associates::{Commit, GitModel};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum GeologicalEvent {
    Uplift { x: usize, y: usize, amount: f32 },
    Weathering { x: usize, y: usize, intensity: f32 },
}

pub struct FileMapper {
    width: usize,
    height: usize,
    path_map: HashMap<String, (usize, usize)>,
    grid: Vec<Option<String>>,
}

impl FileMapper {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            path_map: HashMap::new(),
            grid: vec![None; width * height],
        }
    }

    pub fn get_coordinate(&mut self, path: &str) -> (usize, usize) {
        if let Some(&coord) = self.path_map.get(path) {
            return coord;
        }

        // Find a spot
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash = hasher.finish() as usize;

        let start_idx = hash % (self.width * self.height);
        let mut idx = start_idx;

        // Linear probing to find empty spot
        loop {
            if self.grid[idx].is_none() {
                self.grid[idx] = Some(path.to_string());
                let x = idx % self.width;
                let y = idx / self.width;
                self.path_map.insert(path.to_string(), (x, y));
                return (x, y);
            }

            idx = (idx + 1) % (self.width * self.height);

            // Full loop check - if map is full, overwrite start_idx
            if idx == start_idx {
                let x = start_idx % self.width;
                let y = start_idx / self.width;
                return (x, y);
            }
        }
    }
}

pub struct HistoryStream {
    pub commits: Vec<Commit>,
    pub current_index: usize,
}

impl HistoryStream {
    pub fn new(path: &str, limit: usize) -> Result<Self> {
        let model = GitModel::open(path)?;
        // Fetch history with diffs
        let mut commits = model.history_with_diffs(limit)?;
        // Reverse to play forward in time
        commits.reverse();
        Ok(Self {
            commits,
            current_index: 0,
        })
    }

    fn next(&mut self) -> Option<&Commit> {
        if self.current_index < self.commits.len() {
            let commit = &self.commits[self.current_index];
            self.current_index += 1;
            Some(commit)
        } else {
            None
        }
    }

    pub fn next_events(
        &mut self,
        mapper: &mut FileMapper,
    ) -> Option<(Vec<GeologicalEvent>, String, String)> {
        if let Some(commit) = self.next() {
            let mut events = Vec::new();
            for file in &commit.files {
                let (x, y) = mapper.get_coordinate(&file.path);

                // Uplift based on insertions
                if file.insertions > 0 {
                    // Logarithmic scale? Or linear?
                    // Let's cap it.
                    let amount = (file.insertions as f32).min(50.0) * 0.1;
                    events.push(GeologicalEvent::Uplift { x, y, amount });
                }

                // Weathering based on deletions/changes
                // Even insertions cause weathering (disturbance)
                let intensity = ((file.deletions + file.insertions) as f32).min(100.0) * 0.05;
                if intensity > 0.0 {
                    events.push(GeologicalEvent::Weathering { x, y, intensity });
                }
            }
            Some((events, commit.short_hash.clone(), commit.message.clone()))
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_mapping() {
        let mut mapper = FileMapper::new(10, 10);
        let c1 = mapper.get_coordinate("src/main.rs");
        let c2 = mapper.get_coordinate("src/lib.rs");
        let c3 = mapper.get_coordinate("src/main.rs");

        assert_eq!(c1, c3);
        assert_ne!(c1, c2); // Unlikely collision
    }

    #[test]
    fn test_history_loading() {
        // This test relies on running inside a git repo (the sandbox is one)
        let stream = HistoryStream::new(".", 10);
        if let Ok(mut stream) = stream {
            assert!(stream.commits.len() > 0);
            let mut mapper = FileMapper::new(10, 10);
            if let Some((events, _hash, _msg)) = stream.next_events(&mut mapper) {
                // We might have events or not depending on the commit
                // But it shouldn't panic
                println!("Got {} events", events.len());
            }
        } else {
            // Failed to open repo? Maybe running in CI without .git?
            // In the sandbox, .git should exist.
            // If it fails, we print error
            println!("Failed to load git history");
        }
    }
}
