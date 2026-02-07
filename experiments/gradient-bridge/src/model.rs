use rand::Rng;
use crate::landscape::ObjectiveFunction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Foraging,
    Bridging,
    Returning, // Maybe used if they find "food" (Peak)
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub state: State,
    pub last_height: f32, // To track progress
}

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            state: State::Foraging,
            last_height: -999.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Terrain {
    Solid,
    Gap,
    Bridge,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Terrain>,
    pub height_map: Vec<f32>,
    pub pheromones: Vec<f32>, // 0.0 to 1.0 (Trail)
    pub ants: Vec<Ant>,
    pub gap_threshold: f32,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            terrain: vec![Terrain::Solid; width * height],
            height_map: vec![0.0; width * height],
            pheromones: vec![0.0; width * height],
            ants: Vec::new(),
            gap_threshold: -1.0, // Default, can be tweaked
        }
    }

    pub fn generate_terrain(&mut self, func: &dyn ObjectiveFunction) {
        // Map grid (0..width, 0..height) to world space (-10..10, -10..10)
        let world_min = -10.0;
        let world_max = 10.0;
        let scale_x = (world_max - world_min) / self.width as f32;
        let scale_y = (world_max - world_min) / self.height as f32;

        for y in 0..self.height {
            for x in 0..self.width {
                let wx = world_min + x as f32 * scale_x;
                let wy = world_min + y as f32 * scale_y;
                let val = func.value(wx, wy);

                let idx = y * self.width + x;
                self.height_map[idx] = val;

                if val < self.gap_threshold {
                    self.terrain[idx] = Terrain::Gap;
                } else {
                    self.terrain[idx] = Terrain::Solid;
                }

                // Reset pheromones on map change
                self.pheromones[idx] = 0.0;
            }
        }

        // Reset ants to start or respawn?
        // Let's clear ants on new terrain in main loop, or here.
        // Better to let main handle ant spawning.
        self.ants.clear();
    }

    pub fn set_terrain(&mut self, x: usize, y: usize, t: Terrain) {
        if x < self.width && y < self.height {
            self.terrain[y * self.width + x] = t;
        }
    }

    pub fn add_ant(&mut self, x: i32, y: i32) {
        self.ants.push(Ant::new(x, y));
    }

    pub fn get_terrain(&self, x: i32, y: i32) -> Terrain {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return Terrain::Gap; // Treat out of bounds as Gap (void)
        }
        self.terrain[(y as usize) * self.width + (x as usize)]
    }

    pub fn get_height(&self, x: i32, y: i32) -> f32 {
         if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return -9999.0;
        }
        self.height_map[(y as usize) * self.width + (x as usize)]
    }

    pub fn set_terrain_safe(&mut self, x: i32, y: i32, t: Terrain) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.terrain[(y as usize) * self.width + (x as usize)] = t;
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Evaporate pheromones
        for p in &mut self.pheromones {
            *p *= 0.98;
        }

        // 2. Build Ant Grid for O(1) neighbor checks (crowd density)
        let mut ant_grid = vec![0; self.width * self.height];
        for ant in &self.ants {
             if ant.x >= 0 && ant.y >= 0 && (ant.x as usize) < self.width && (ant.y as usize) < self.height {
                 ant_grid[(ant.y as usize) * self.width + (ant.x as usize)] += 1;
             }
        }

        let width = self.width;
        let height = self.height;
        let ant_grid_ref = &ant_grid;

        // Capture read-only data for the loop to avoid borrow checker issues
        let height_map = &self.height_map;
        let terrain = &self.terrain;
        let pheromones = &self.pheromones;

        let count_neighbors = |x: i32, y: i32| -> i32 {
            let mut count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                         count += ant_grid_ref[(ny as usize) * width + (nx as usize)];
                    }
                }
            }
            // Subtract self from count
            if count > 0 { count - 1 } else { 0 }
        };

        let get_height_local = |x: i32, y: i32| -> f32 {
             if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                return -9999.0;
            }
            height_map[(y as usize) * width + (x as usize)]
        };

        let get_terrain_local = |x: i32, y: i32| -> Terrain {
            if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                return Terrain::Gap;
            }
            terrain[(y as usize) * width + (x as usize)]
        };

        // 3. Determine updates
        let mut updates = Vec::new();

        for (i, ant) in self.ants.iter().enumerate() {
            let x = ant.x;
            let y = ant.y;
            let state = ant.state;

            match state {
                State::Bridging => {
                    let neighbors = count_neighbors(x, y);
                    // Unbridge rule: If neighbors < 2 (lonely bridge), unbridge.
                    if neighbors < 2 {
                        updates.push((i, x, y, State::Foraging, Some((x, y, Terrain::Gap))));
                    } else {
                         // Just stay there.
                    }
                }
                State::Foraging | State::Returning => {
                    // Decide where to move based on Gradient Ascent
                    let mut possible_moves = Vec::new();
                    let current_h = get_height_local(x, y);

                    // Look at neighbors
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x + dx;
                            let ny = y + dy;

                            // Check terrain
                            let t = get_terrain_local(nx, ny);

                            if t != Terrain::Gap {
                                // Calculate score
                                let h = get_height_local(nx, ny);
                                let h_diff = h - current_h;

                                // Pheromone attraction
                                let p = if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                                    pheromones[(ny as usize) * width + (nx as usize)]
                                } else { 0.0 };

                                // Score: Height Diff + Pheromone + Random Noise
                                let score = h_diff * 5.0 + p * 2.0 + rng.gen_range(0.0..1.0);

                                possible_moves.push((nx, ny, false, score));
                            } else {
                                // Gap. Can we bridge?
                                let neighbors = count_neighbors(x, y);
                                if neighbors >= 3 {
                                    // High score for bridging if crowded
                                    possible_moves.push((nx, ny, true, 100.0));
                                }
                            }
                        }
                    }

                    if !possible_moves.is_empty() {
                        // Let's pick best.
                        possible_moves.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

                        // Pick top 1, or top 3 random?
                        let choice_idx = if rng.gen_bool(0.8) { 0 } else { rng.gen_range(0..possible_moves.len()) };
                        let (nx, ny, bridging, _) = possible_moves[choice_idx];

                        if bridging {
                            // Become bridge
                             updates.push((i, nx, ny, State::Bridging, Some((nx, ny, Terrain::Bridge))));
                        } else {
                             // Move
                             updates.push((i, nx, ny, state, None));
                        }
                    }
                }
            }
        }

        // 4. Apply updates
        for (i, nx, ny, nstate, terrain_change) in updates {
            self.ants[i].x = nx;
            self.ants[i].y = ny;
            self.ants[i].state = nstate;

            if let Some((tx, ty, t)) = terrain_change {
                 self.set_terrain_safe(tx, ty, t);
            }

            // Apply pheromone at new position
            if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                let idx = (ny as usize) * width + (nx as usize);
                // Deposit pheromone
                self.pheromones[idx] = (self.pheromones[idx] + 0.2).min(1.0);
            }
        }
    }
}
