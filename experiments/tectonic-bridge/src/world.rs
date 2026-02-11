use crate::git::CommitData;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Terrain {
    Empty,
    Solid { commit_idx: usize }, // Store which commit this belongs to for color
    Gap,
    Bridge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntState {
    Foraging,
    Bridging,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub state: AntState,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Terrain>,
    pub ants: Vec<Ant>,
    pub commits: Vec<CommitData>,
    pub scroll_y: f64,
}

impl World {
    pub fn new(width: usize, height: usize, commits: Vec<CommitData>) -> Self {
        let terrain = Self::generate_terrain_data(width, height, &commits);

        let mut world = Self {
            width,
            height,
            terrain,
            ants: Vec::new(),
            commits,
            scroll_y: 0.0,
        };
        world.spawn_ants(100);
        world
    }

    fn generate_terrain_data(width: usize, height: usize, commits: &[CommitData]) -> Vec<Terrain> {
        let mut terrain = vec![Terrain::Empty; width * height];
        let mut rng = rand::thread_rng();
        let mut current_y = 0;

        for (idx, commit) in commits.iter().enumerate() {
            let thickness = 2;

            for _ in 0..thickness {
                if current_y >= height {
                    break;
                }

                // Fill row with Solid
                for x in 0..width {
                    terrain[current_y * width + x] = Terrain::Solid { commit_idx: idx };
                }

                // Create Gaps based on stress
                if commit.stress_level > 0.0 {
                    let gap_width = (commit.stress_level * 2.0).min(width as f64 / 2.0) as usize;
                    let num_gaps = (commit.stress_level as usize).max(1).min(5);

                    for _ in 0..num_gaps {
                        let center_x = rng.gen_range(0..width);
                        let start_x = center_x.saturating_sub(gap_width / 2);
                        let end_x = (center_x + gap_width / 2).min(width);

                        for x in start_x..end_x {
                             if x < width {
                                terrain[current_y * width + x] = Terrain::Gap;
                             }
                        }
                    }
                }

                current_y += 1;
            }
        }
        terrain
    }

    pub fn spawn_ants(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen_range(0..self.width as i32);
            self.ants.push(Ant {
                x,
                y: 0, // Start at bottom
                state: AntState::Foraging,
            });
        }
    }

    pub fn set_terrain(&mut self, x: usize, y: usize, t: Terrain) {
        if x < self.width && y < self.height {
            self.terrain[y * self.width + x] = t;
        }
    }

    pub fn get_terrain(&self, x: i32, y: i32) -> Terrain {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            if y < 0 { return Terrain::Solid { commit_idx: 0 }; } // Bedrock
            return Terrain::Empty;
        }
        self.terrain[(y as usize) * self.width + (x as usize)]
    }

    pub fn set_terrain_safe(&mut self, x: i32, y: i32, t: Terrain) {
         if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.terrain[(y as usize) * self.width + (x as usize)] = t;
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        let width = self.width;
        let height = self.height;

        // Build ant grid for neighbor counts
        let mut ant_grid = vec![0; width * height];
        for ant in &self.ants {
             if ant.x >= 0 && ant.y >= 0 && (ant.x as usize) < width && (ant.y as usize) < height {
                ant_grid[(ant.y as usize) * width + (ant.x as usize)] += 1;
            }
        }

        // Helper closure to avoid borrow checker issues with self
        let terrain = &self.terrain;
        let get_terrain_local = |x: i32, y: i32| -> Terrain {
            if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                if y < 0 { return Terrain::Solid { commit_idx: 0 }; }
                return Terrain::Empty;
            }
            terrain[(y as usize) * width + (x as usize)]
        };

        let mut updates = Vec::new();

        for (i, ant) in self.ants.iter().enumerate() {
            match ant.state {
                AntState::Bridging => {
                    // Check neighbors
                    let mut neighbors = 0;
                     for dx in -2..=2 {
                        for dy in -2..=2 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = ant.x + dx;
                            let ny = ant.y + dy;
                             if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                                 neighbors += ant_grid[(ny as usize) * width + (nx as usize)];
                             }
                        }
                    }

                    if neighbors < 2 && rng.gen_bool(0.05) { // Unbridge if lonely
                         updates.push((i, ant.x, ant.y, AntState::Foraging, Some((ant.x, ant.y, Terrain::Gap))));
                    } else {
                         updates.push((i, ant.x, ant.y, AntState::Bridging, Some((ant.x, ant.y, Terrain::Bridge))));
                    }
                },
                AntState::Foraging => {
                    let moves = [
                        (0, 1), // Up
                        (-1, 1), // Up-Left
                        (1, 1), // Up-Right
                        (-1, 0), // Left
                        (1, 0), // Right
                         (0, -1), // Down (fallback)
                    ];

                    let mut chosen_move = None;

                    for (dx, dy) in moves {
                        let nx = ant.x + dx;
                        let ny = ant.y + dy;
                        let t = get_terrain_local(nx, ny);

                        match t {
                            Terrain::Solid { .. } | Terrain::Bridge => {
                                chosen_move = Some((nx, ny, false));
                                break;
                            },
                            Terrain::Gap => {
                                // Check crowding
                                let mut neighbors = 0;
                                 for ndx in -2..=2 {
                                    for ndy in -2..=2 {
                                        if ndx == 0 && ndy == 0 { continue; }
                                        let nnx = ant.x + ndx;
                                        let nny = ant.y + ndy;
                                         if nnx >= 0 && nny >= 0 && (nnx as usize) < width && (nny as usize) < height {
                                             neighbors += ant_grid[(nny as usize) * width + (nnx as usize)];
                                         }
                                    }
                                }

                                if neighbors >= 3 {
                                     chosen_move = Some((nx, ny, true));
                                     break;
                                }
                            },
                            Terrain::Empty => {
                                // Sky?
                                chosen_move = Some((nx, ny, false));
                                if dx == 0 && dy == 1 {
                                    break;
                                }
                            }
                        }
                    }

                    if let Some((nx, ny, bridging)) = chosen_move {
                        if bridging {
                            updates.push((i, nx, ny, AntState::Bridging, Some((nx, ny, Terrain::Bridge))));
                        } else {
                            updates.push((i, nx, ny, AntState::Foraging, None));
                        }
                    } else {
                        // Random walk if stuck
                        if rng.gen_bool(0.1) {
                             let dx = rng.gen_range(-1..=1);
                             let dy = rng.gen_range(-1..=1);
                             let nx = ant.x + dx;
                             let ny = ant.y + dy;
                             if matches!(get_terrain_local(nx, ny), Terrain::Solid{..} | Terrain::Bridge | Terrain::Empty) {
                                 updates.push((i, nx, ny, AntState::Foraging, None));
                             }
                        }
                    }
                }
            }
        }

        for (i, nx, ny, state, terrain_change) in updates {
            self.ants[i].x = nx;
            self.ants[i].y = ny;
            self.ants[i].state = state;
            if let Some((tx, ty, t)) = terrain_change {
                self.set_terrain_safe(tx, ty, t);
            }
        }

        if self.ants.len() < 200 {
            self.spawn_ants(5);
        }
    }
}
