use std::collections::{HashMap, HashSet};
use git_associates::{GitModel, Commit};
use crate::map::{HilbertCurve, WorldMap};

pub struct History {
    pub commits: Vec<Commit>,
    pub file_map: HashMap<String, usize>, // File Path -> Grid Index
    pub files: Vec<String>,               // Sorted list of files
    pub authors: HashMap<String, u8>,     // Author Name -> ID
}

impl History {
    pub fn new(path: &str, map: &mut WorldMap) -> anyhow::Result<Self> {
        // Open the repo
        let model = GitModel::open(path)?;

        // Fetch last 2000 commits (or more if needed)
        // We use history_with_diffs to get file changes
        let mut commits = model.history_with_diffs(2000)?;

        // Reverse to replay from past to present
        commits.reverse();

        let mut unique_files = HashSet::new();
        let mut authors = HashMap::new();
        let mut next_author_id: u8 = 1;

        // Scan history to build maps
        for commit in &commits {
            // Map author
            if !authors.contains_key(&commit.author) {
                // Use wrapping to cycle colors if more than 255 authors
                authors.insert(commit.author.clone(), next_author_id);
                next_author_id = next_author_id.wrapping_add(1);
                if next_author_id == 0 { next_author_id = 1; }
            }

            // Map files
            for file in &commit.files {
                unique_files.insert(file.path.clone());
            }
        }

        // Sort files for stable mapping
        let mut sorted_files: Vec<String> = unique_files.into_iter().collect();
        sorted_files.sort();

        // Map to Hilbert Curve
        // Use the max dimension for the curve order
        let n = map.width.max(map.height).next_power_of_two();
        let curve = HilbertCurve::new(n);
        let mut file_map = HashMap::new();

        for (i, file_path) in sorted_files.iter().enumerate() {
            // Get coordinates from curve
            let (x, y) = curve.d2xy(i);

            // If within map bounds, register it
            if let Some(idx) = map.get_index(x, y) {
                map.file_indices[idx] = Some(i); // Store index into sorted_files
                file_map.insert(file_path.clone(), idx);
            }
        }

        Ok(Self {
            commits,
            file_map,
            files: sorted_files,
            authors,
        })
    }
}
