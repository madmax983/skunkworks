use rand::Rng;
use crate::phonology::{Word, RuleType, evolve, parse_identifier};

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
    pub word: Word,
    pub age: u32,
}

const PROTO_WORDS: &[&str] = &[
    "pater", "mater", "frater", "soror",
    "domus", "via", "terra", "aqua",
    "ignis", "ventus", "caelum", "mare",
    "corpus", "mens", "spiritus", "anima",
    "rex", "lex", "grex", "pes",
    "manus", "caput", "oculus", "dens",
];

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let proto = PROTO_WORDS[rng.gen_range(0..PROTO_WORDS.len())];
        Self {
            x,
            y,
            state: State::Foraging,
            word: parse_identifier(proto),
            age: 0,
        }
    }
}

pub fn levenshtein_distance(w1: &Word, w2: &Word) -> usize {
    let v1 = &w1.phonemes;
    let v2 = &w2.phonemes;
    let n = v1.len();
    let m = v2.len();

    if n == 0 { return m; }
    if m == 0 { return n; }

    let mut matrix = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n { matrix[i][0] = i; }
    for j in 0..=m { matrix[0][j] = j; }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1) // deletion
                .min(matrix[i][j - 1] + 1)     // insertion
                .min(matrix[i - 1][j - 1] + cost); // substitution
        }
    }

    matrix[n][m]
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

    pub fn evolve_all(&mut self, chance: f64) {
        let mut rng = rand::thread_rng();
        for ant in &mut self.ants {
            if rng.gen_bool(chance) {
                let rule = match rng.gen_range(0..4) {
                    0 => RuleType::Grimm,
                    1 => RuleType::VowelShift,
                    2 => RuleType::Lenition,
                    _ => RuleType::Assimilation,
                };
                evolve(&mut ant.word, rule);
                ant.age += 1;
            }
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Evaporate pheromones
        for p in &mut self.pheromones {
            *p *= 0.99;
        }

        // 2. Build Ant Grid for O(1) neighbor checks (stores index of ant)
        let mut ant_map: Vec<Vec<usize>> = vec![Vec::new(); self.width * self.height];
        for (i, ant) in self.ants.iter().enumerate() {
            if ant.x >= 0
                && ant.y >= 0
                && (ant.x as usize) < self.width
                && (ant.y as usize) < self.height
            {
                ant_map[(ant.y as usize) * self.width + (ant.x as usize)].push(i);
            }
        }

        let width = self.width;
        let height = self.height;

        // Threshold for phonetic distance
        const PHONETIC_THRESHOLD: usize = 2;

        let mut updates = Vec::new();

        for (i, ant) in self.ants.iter().enumerate() {
            let x = ant.x;
            let y = ant.y;
            let state = ant.state;

            // Helper to check valid supports
            let mut valid_supports = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                        let idx = (ny as usize) * width + (nx as usize);

                        // Check Solid Terrain
                        if self.terrain[idx] == Terrain::Solid {
                            valid_supports += 1;
                        } else {
                            // Check Ants
                            let neighbors = &ant_map[idx];
                            for &n_idx in neighbors {
                                if n_idx == i { continue; }
                                let neighbor_ant = &self.ants[n_idx];

                                // Must be bridging to provide support
                                if neighbor_ant.state == State::Bridging {
                                    let d = levenshtein_distance(&ant.word, &neighbor_ant.word);
                                    if d <= PHONETIC_THRESHOLD {
                                        valid_supports += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match state {
                State::Bridging => {
                    // Collapse condition:
                    // Not enough valid supports (< 2)
                    if valid_supports < 2 {
                        updates.push((i, x, y, State::Foraging, Some((x, y, Terrain::Gap))));
                    } else {
                        // Reinforce bridge
                        updates.push((i, x, y, State::Bridging, Some((x, y, Terrain::Bridge))));
                    }
                }
                State::Foraging | State::Returning => {
                    // Decide where to move
                    let mut possible_moves = Vec::new();

                    // Look at neighbors for movement
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x + dx;
                            let ny = y + dy;

                            // Check terrain
                            let t = if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                                self.terrain[(ny as usize) * width + (nx as usize)]
                            } else {
                                Terrain::Gap
                            };

                            if t != Terrain::Gap {
                                possible_moves.push((nx, ny, false)); // false = not bridging
                            } else {
                                // Gap. Can we bridge?
                                // Only bridge if we have enough valid supports at CURRENT position?
                                // No, usually you step into the gap and attach to supports.
                                // But here, the logic is "falling into gap to bridge it".
                                // The supports must be at (x,y) (where we are coming FROM) or around the new position?
                                // The new position (nx, ny) is the gap. Supports are around (nx, ny).
                                // So we need to check supports around (nx, ny).

                                // Let's check supports around (nx, ny)
                                let mut future_supports = 0;
                                for fdx in -1..=1 {
                                    for fdy in -1..=1 {
                                        if fdx == 0 && fdy == 0 { continue; }
                                        let fnx = nx + fdx;
                                        let fny = ny + fdy;
                                        if fnx >= 0 && fny >= 0 && (fnx as usize) < width && (fny as usize) < height {
                                            let fidx = (fny as usize) * width + (fnx as usize);
                                            if self.terrain[fidx] == Terrain::Solid {
                                                future_supports += 1;
                                            } else {
                                                for &n_idx in &ant_map[fidx] {
                                                    if n_idx == i { continue; }
                                                    let neighbor = &self.ants[n_idx];
                                                    if neighbor.state == State::Bridging {
                                                        if levenshtein_distance(&ant.word, &neighbor.word) <= PHONETIC_THRESHOLD {
                                                            future_supports += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if future_supports >= 2 {
                                    possible_moves.push((nx, ny, true)); // true = bridging attempt
                                }
                            }
                        }
                    }

                    if !possible_moves.is_empty() {
                        let idx = rng.gen_range(0..possible_moves.len());
                        let (nx, ny, bridging) = possible_moves[idx];

                        if bridging {
                            // Become bridge at NEW location
                            updates.push((
                                i,
                                nx,
                                ny,
                                State::Bridging,
                                Some((nx, ny, Terrain::Bridge)),
                            ));
                        } else {
                            // Just move
                            updates.push((i, nx, ny, state, None));
                        }
                    } else {
                        // Stay put
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
                self.pheromones[idx] = (self.pheromones[idx] + 0.1).min(1.0);
            }
        }
    }
}
