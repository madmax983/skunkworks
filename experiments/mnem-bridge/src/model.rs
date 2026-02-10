use rand::Rng;
use crate::glitch::TextGlitcher;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Foraging,
    Bridging,
    Returning,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub state: State,
    pub payload: String,
    pub health: f32,
}

const MEMORIES: &[&str] = &[
    "The bridge is memory.",
    "We cross the void.",
    "Do not forget.",
    "Hold the line.",
    "Structure is semantic.",
    "Entropy is coming.",
    "Data rot is real.",
    "Bit flip detected.",
    "Signal to noise.",
    "Connecting the dots.",
];

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let payload = MEMORIES[rng.gen_range(0..MEMORIES.len())].to_string();
        Self {
            x,
            y,
            state: State::Foraging,
            payload,
            health: 1.0,
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
    pub pheromones: Vec<f32>, // 0.0 to 1.0
    pub ants: Vec<Ant>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            terrain: vec![Terrain::Solid; width * height],
            pheromones: vec![0.0; width * height],
            ants: Vec::new(),
        }
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

    pub fn set_terrain_safe(&mut self, x: i32, y: i32, t: Terrain) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.terrain[(y as usize) * self.width + (x as usize)] = t;
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Evaporate pheromones
        for p in &mut self.pheromones {
            *p *= 0.99;
        }

        // 2. Build Ant Grid for O(1) neighbor checks
        let mut ant_grid = vec![0; self.width * self.height];
        for ant in &self.ants {
            if ant.x >= 0
                && ant.y >= 0
                && (ant.x as usize) < self.width
                && (ant.y as usize) < self.height
            {
                ant_grid[(ant.y as usize) * self.width + (ant.x as usize)] += 1;
            }
        }

        let width = self.width;
        let height = self.height;
        let ant_grid_ref = &ant_grid;

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
            if count > 0 {
                count - 1
            } else {
                0
            }
        };

        // 3. Determine updates
        // We collect changes to apply them after to avoid borrow conflicts
        // (index, new_x, new_y, new_state, new_terrain_at_pos, reset_ant)

        let mut updates = Vec::new();

        for (i, ant) in self.ants.iter_mut().enumerate() {
            let x = ant.x;
            let y = ant.y;
            let state = ant.state;

            // Decay logic
            if state == State::Bridging {
                ant.health -= 0.002; // Slow decay

                // Corrupt payload
                if rng.gen::<f32>() < 0.05 {
                     ant.payload = TextGlitcher::corrupt(&ant.payload, 1.0 - ant.health);
                }

                if ant.health <= 0.0 {
                    // Bridge collapse!
                    updates.push((i, 0, 0, State::Foraging, Some((x, y, Terrain::Gap)), true));
                    continue;
                }
            }

            match state {
                State::Bridging => {
                    let neighbors = count_neighbors(x, y);
                    // Unbridge rule: If neighbors < 2 (lonely bridge), unbridge.
                    if neighbors < 2 {
                        updates.push((i, x, y, State::Foraging, Some((x, y, Terrain::Gap)), false));
                    } else {
                        // Reinforce bridge
                        updates.push((i, x, y, State::Bridging, Some((x, y, Terrain::Bridge)), false));
                    }
                }
                State::Foraging | State::Returning => {
                    // Decide where to move
                    let mut possible_moves = Vec::new();

                    // Look at neighbors
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = x + dx;
                            let ny = y + dy;

                            // Check terrain
                            let t = if nx >= 0
                                && ny >= 0
                                && (nx as usize) < width
                                && (ny as usize) < height
                            {
                                self.terrain[(ny as usize) * width + (nx as usize)]
                            } else {
                                Terrain::Gap
                            };

                            if t != Terrain::Gap {
                                possible_moves.push((nx, ny, false)); // false = not bridging
                            } else {
                                // Gap. Can we bridge?
                                let neighbors = count_neighbors(x, y); // Check crowding at CURRENT position
                                if neighbors >= 3 {
                                    possible_moves.push((nx, ny, true)); // true = bridging attempt
                                }
                            }
                        }
                    }

                    if !possible_moves.is_empty() {
                        let idx = rng.gen_range(0..possible_moves.len());
                        let (nx, ny, bridging) = possible_moves[idx];

                        if bridging {
                            // Become bridge at NEW location (falling into gap to bridge it)
                            updates.push((
                                i,
                                nx,
                                ny,
                                State::Bridging,
                                Some((nx, ny, Terrain::Bridge)),
                                false
                            ));
                        } else {
                            // Just move
                            updates.push((i, nx, ny, state, None, false));
                        }
                    } else {
                        // Stay put
                    }
                }
            }
        }

        // 4. Apply updates
        for (i, nx, ny, nstate, terrain_change, reset) in updates {
            if reset {
                // Respawn logic
                self.ants[i].x = rng.gen_range(5..20);
                self.ants[i].y = rng.gen_range(5..self.height as i32 - 5);
                self.ants[i].state = State::Foraging;
                self.ants[i].health = 1.0;
                self.ants[i].payload = MEMORIES[rng.gen_range(0..MEMORIES.len())].to_string();
            } else {
                self.ants[i].x = nx;
                self.ants[i].y = ny;
                self.ants[i].state = nstate;
            }

            if let Some((tx, ty, t)) = terrain_change {
                self.set_terrain_safe(tx, ty, t);
            }

            // Apply pheromone at new position
            let ax = self.ants[i].x;
            let ay = self.ants[i].y;
            if ax >= 0 && ay >= 0 && (ax as usize) < width && (ay as usize) < height {
                let idx = (ay as usize) * width + (ax as usize);
                self.pheromones[idx] = (self.pheromones[idx] + 0.1).min(1.0);
            }
        }
    }
}
