use crate::map::WorldMap;
use crate::history::History;
use git_associates::Commit;

pub fn process_commit(commit: &Commit, history: &History, map: &mut WorldMap) {
    if let Some(author_id) = history.authors.get(&commit.author) {
        let author_id = *author_id;

        for file_change in &commit.files {
            if let Some(idx) = history.file_map.get(&file_change.path) {
                let idx = *idx;

                // Get current pixel state
                let current_owner = map.owner[idx];
                let current_strength = map.strength[idx];

                // Tectonic interaction
                if current_owner == 0 {
                    // Claim unclaimed land
                    map.owner[idx] = author_id;
                    map.strength[idx] = 0.5;
                    map.heightmap[idx] += 0.5; // Initial deposit
                } else if current_owner == author_id {
                    // Reinforce ownership
                    map.strength[idx] = (current_strength + 0.1).min(1.0);
                    map.heightmap[idx] += 0.1; // Deposition from activity
                } else {
                    // CONFLICT!
                    // Uplift based on conflict intensity
                    // More established owner = harder fight = higher mountain
                    let uplift = 1.0 * current_strength;
                    map.heightmap[idx] += uplift;

                    // Reduce strength of current owner
                    // If the attacker is persistent, they will eventually take over
                    map.strength[idx] -= 0.2;

                    if map.strength[idx] <= 0.0 {
                        // Coup!
                        map.owner[idx] = author_id;
                        map.strength[idx] = 0.2;
                    }
                }
            }
        }
    }
}

/// Simple thermal erosion (smoothing) to prevent infinite spikes
pub fn thermal_erosion(map: &mut WorldMap) {
    let w = map.width;
    let h = map.height;
    // We modify in place with a bias to avoid allocating a full buffer every frame?
    // No, strictly correct erosion needs double buffering or carefully ordered updates.
    // For visual purposes, we can just clamp spikes.

    // Let's identify unstable slopes.
    // Talus angle = 4.0 height units per pixel.
    let talus = 1.5;

    for i in 0..map.heightmap.len() {
        let x = i % w;
        let y = i / w;
        let h_val = map.heightmap[i];

        // Check 4 neighbors
        let neighbors = [
            if x > 0 { Some(i - 1) } else { None },
            if x < w - 1 { Some(i + 1) } else { None },
            if y > 0 { Some(i - w) } else { None },
            if y < h - 1 { Some(i + w) } else { None },
        ];

        for n_idx in neighbors.into_iter().flatten() {
            let n_h = map.heightmap[n_idx];
            let diff = h_val - n_h;
            if diff > talus {
                // Slump
                let transfer = (diff - talus) * 0.5;
                map.heightmap[i] -= transfer;
                map.heightmap[n_idx] += transfer;
            }
        }
    }
}
