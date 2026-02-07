use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum State {
    Foraging,
    Bridging,
    Returning,
    Panicking, // New state: when bridge breaks due to waves
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub state: State,
    pub energy: f32, // New: Ants have energy, bridging consumes it?
}

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            state: State::Foraging,
            energy: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Terrain {
    Solid,  // Ground
    Gap,    // Fluid (impassable without bridge)
    Bridge, // Ant acting as structure
}

pub struct AntColony {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Terrain>,
    pub pheromones: Vec<f32>, // 0.0 to 1.0
    pub ants: Vec<Ant>,
}

impl AntColony {
    pub fn new(width: usize, height: usize) -> Self {
        let mut ants = Vec::new();
        // Spawn some initial ants
        for _ in 0..200 {
            ants.push(Ant::new(width as i32 / 2, height as i32 / 2));
        }

        Self {
            width,
            height,
            terrain: vec![Terrain::Solid; width * height],
            pheromones: vec![0.0; width * height],
            ants,
        }
    }

    #[allow(dead_code)]
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

    pub fn set_terrain_safe(&mut self, x: i32, y: i32, t: Terrain) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.terrain[(y as usize) * self.width + (x as usize)] = t;
        }
    }

    // Sync terrain with fluid density.
    // Fluid > threshold -> Gap (unless Bridge)
    pub fn sync_with_fluid(&mut self, density: &[f32], threshold: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                match self.terrain[idx] {
                    Terrain::Bridge => {
                        // Bridge persists unless broken explicitly
                    }
                    _ => {
                        if density[idx] > threshold {
                            self.terrain[idx] = Terrain::Gap;
                        } else {
                            self.terrain[idx] = Terrain::Solid;
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, audio_energy: f32) {
        let mut rng = rand::thread_rng();

        // 1. Evaporate pheromones
        for p in &mut self.pheromones {
            *p *= 0.98;
        }

        // 2. Build Ant Grid for O(1) neighbor checks
        let mut ant_grid = vec![0; self.width * self.height];
        for ant in &self.ants {
             if ant.x >= 0 && ant.y >= 0 && (ant.x as usize) < self.width && (ant.y as usize) < self.height {
                 ant_grid[(ant.y as usize) * self.width + (ant.x as usize)] += 1;
             }
        }

        let width = self.width;
        let height = self.height;
        // let ant_grid_ref = &ant_grid; // Not needed if we clone or access direct

        // 3. Determine updates
        // We collect changes to apply them after to avoid borrow conflicts
        struct Update {
            index: usize,
            nx: i32,
            ny: i32,
            nstate: State,
            terrain_change: Option<(i32, i32, Terrain)>,
        }

        let mut updates = Vec::new();

        // Helper to check neighbors in grid
        let count_neighbors = |x: i32, y: i32, grid: &[i32]| -> i32 {
            let mut count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                         count += grid[(ny as usize) * width + (nx as usize)];
                    }
                }
            }
            if count > 0 { count - 1 } else { 0 }
        };

        for (i, ant) in self.ants.iter().enumerate() {
            let x = ant.x;
            let y = ant.y;
            let state = ant.state;

            // Audio energy can break bridges (Panic)
            if state == State::Bridging && rng.gen::<f32>() < (audio_energy * 0.1) {
                // Bridge collapse!
                updates.push(Update {
                    index: i,
                    nx: x,
                    ny: y,
                    nstate: State::Panicking,
                    terrain_change: Some((x, y, Terrain::Gap)), // Falls into fluid
                });
                continue;
            }

            match state {
                State::Bridging => {
                    let neighbors = count_neighbors(x, y, &ant_grid);
                    // Unbridge rule: If neighbors < 2 (lonely bridge), unbridge.
                    if neighbors < 2 {
                        // Fall back to gap
                        updates.push(Update {
                            index: i,
                            nx: x,
                            ny: y,
                            nstate: State::Foraging,
                            terrain_change: Some((x, y, Terrain::Gap)),
                        });
                    } else {
                         // Stay as bridge
                    }
                }
                State::Panicking => {
                    // Random walk to find solid ground
                     let dx = rng.gen_range(-1..=1);
                     let dy = rng.gen_range(-1..=1);
                     let nx = x + dx;
                     let ny = y + dy;

                     let t = self.get_terrain(nx, ny);
                     if t == Terrain::Solid {
                         updates.push(Update {
                            index: i,
                            nx,
                            ny,
                            nstate: State::Foraging,
                            terrain_change: None,
                        });
                     } else {
                         // Still panicking (swimming?)
                         updates.push(Update {
                            index: i,
                            nx,
                            ny,
                            nstate: State::Panicking,
                            terrain_change: None,
                        });
                     }
                }
                State::Foraging | State::Returning => {
                    // Decide where to move
                    let mut possible_moves = Vec::new();

                    // Look at neighbors
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x + dx;
                            let ny = y + dy;

                            let t = self.get_terrain(nx, ny);

                            if t != Terrain::Gap {
                                // Prefer Pheromones?
                                let mut weight = 1.0;
                                if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                                    weight += self.pheromones[(ny as usize) * width + (nx as usize)] * 10.0;
                                }
                                possible_moves.push((nx, ny, false, weight));
                            } else {
                                // Gap. Can we bridge?
                                let neighbors = count_neighbors(x, y, &ant_grid);
                                // Higher density required to start bridging
                                if neighbors >= 4 {
                                    possible_moves.push((nx, ny, true, 5.0)); // High weight to bridge
                                }
                            }
                        }
                    }

                    if !possible_moves.is_empty() {
                        // Weighted random choice
                        let total_weight: f32 = possible_moves.iter().map(|(_,_,_,w)| w).sum();
                        let mut r = rng.gen::<f32>() * total_weight;
                        let mut selected = possible_moves[0];

                        for m in possible_moves {
                            r -= m.3;
                            if r <= 0.0 {
                                selected = m;
                                break;
                            }
                        }

                        let (nx, ny, bridging, _) = selected;

                        if bridging {
                             updates.push(Update {
                                index: i,
                                nx,
                                ny,
                                nstate: State::Bridging,
                                terrain_change: Some((nx, ny, Terrain::Bridge)),
                            });
                        } else {
                             updates.push(Update {
                                index: i,
                                nx,
                                ny,
                                nstate: state,
                                terrain_change: None,
                            });
                        }
                    }
                }
            }
        }

        // 4. Apply updates
        for update in updates {
            self.ants[update.index].x = update.nx;
            self.ants[update.index].y = update.ny;
            self.ants[update.index].state = update.nstate;

            if let Some((tx, ty, t)) = update.terrain_change {
                 self.set_terrain_safe(tx, ty, t);
            }

            // Apply pheromone at new position
            if update.nx >= 0 && update.ny >= 0 && (update.nx as usize) < width && (update.ny as usize) < height {
                let idx = (update.ny as usize) * width + (update.nx as usize);
                self.pheromones[idx] = (self.pheromones[idx] + 0.1).min(1.0);
            }
        }
    }
}
