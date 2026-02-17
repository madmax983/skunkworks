use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::prelude::*;
use crate::phonology::{TerrainPoint, char_to_terrain, terrain_to_char};

pub const AGENT_COUNT: usize = 20_000;
pub const WORLD_WIDTH: f32 = 1200.0;
pub const WORLD_HEIGHT: f32 = 600.0;
pub const TERRAIN_SCALE_Y: f32 = 400.0;
pub const BASELINE_Y: f32 = 550.0;
pub const POINTS_PER_CHAR: usize = 10;
pub const CHAR_WIDTH: f32 = 40.0;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Flying, 1: Eating/Landed
}

pub struct World {
    pub agents: Vec<Locust>,
    pub terrain: Vec<TerrainPoint>,
    pub original_text: String,
    pub current_text: String,
    pub pheromones: Vec<f32>, // 2D grid for flocking
    pub grid_w: usize,
    pub grid_h: usize,
    pub time: f32,
}

impl World {
    pub fn new(text: &str) -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        // Spawn agents in the sky
        for _ in 0..AGENT_COUNT {
            agents.push(Locust {
                pos: vec2(
                    rng.gen_range(0.0..WORLD_WIDTH),
                    rng.gen_range(0.0..WORLD_HEIGHT * 0.5), // Upper half
                ),
                vel: vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                state: 0,
            });
        }

        let grid_scale = 10;
        let grid_w = (WORLD_WIDTH as usize) / grid_scale;
        let grid_h = (WORLD_HEIGHT as usize) / grid_scale;

        let mut world = World {
            agents,
            terrain: Vec::new(),
            original_text: text.to_string(),
            current_text: text.to_string(),
            pheromones: vec![0.0; grid_w * grid_h],
            grid_w,
            grid_h,
            time: 0.0,
        };

        world.generate_terrain(text);
        world
    }

    pub fn generate_terrain(&mut self, text: &str) {
        self.terrain.clear();

        // Padding
        for _ in 0..POINTS_PER_CHAR {
            self.terrain.push(TerrainPoint { height: 0.0, hardness: 0.0 });
        }

        for c in text.chars() {
            let center = char_to_terrain(c);

            // Interpolate to create a smooth hill/valley
            for i in 0..POINTS_PER_CHAR {
                let t = i as f32 / POINTS_PER_CHAR as f32;
                // Parabolic shape for the phoneme
                // height = center.height * (1 - (2*t - 1)^2)
                // This makes a hump: 0 -> 1 -> 0
                let hump = 1.0 - (2.0 * t - 1.0).powi(2);

                self.terrain.push(TerrainPoint {
                    height: center.height * hump,
                    hardness: center.hardness,
                });
            }
        }

        // Padding
        for _ in 0..POINTS_PER_CHAR {
            self.terrain.push(TerrainPoint { height: 0.0, hardness: 0.0 });
        }
    }

    pub fn update(&mut self) {
        self.time += 0.016;
        let terrain_len = self.terrain.len();
        let terrain_width_px = terrain_len as f32 * (CHAR_WIDTH / POINTS_PER_CHAR as f32);

        // Update Pheromones (Decay)
        self.pheromones.par_iter_mut().for_each(|p| *p *= 0.95);

        // Agents Logic
        // We use a read-only reference to terrain for parallel updates
        // BUT we need to modify terrain (erosion).
        // Solution: Calculate erosion separately? Or do it serially.
        // Or atomic operations?
        // Let's do parallel movement, collect "erosion events", then apply them.

        let agents_state: Vec<(Vec2, Vec2, u8, Option<usize>)> = self.agents.par_iter().map(|agent| {
            let mut pos = agent.pos;
            let mut vel = agent.vel;
            let mut state = agent.state;
            let mut erosion_idx = None;

            // Flocking / Pheromone attraction
            // (Simplified for performance)

            // Attraction to Terrain Peaks
            // Find nearest peak in x-range
            let terrain_idx = ((pos.x / WORLD_WIDTH) * terrain_len as f32) as usize;

            // Seek high ground (Consonants)
            // Look ahead/around
            let look_range = 50;
            let start = terrain_idx.saturating_sub(look_range);
            let end = (terrain_idx + look_range).min(terrain_len - 1);

            // Just a simple gravity towards peaks?
            // Actually, they should be attracted to high `height` values.

            // Force towards "food" (high points)
            // But only if they are flying
            if state == 0 {
                // Gravity
                vel.y += 0.1;

                // Wind/Noise
                let mut rng = ::rand::thread_rng();
                vel.x += rng.gen_range(-0.5..0.5);

                // Check collision with terrain
                if terrain_idx < terrain_len {
                    // Map terrain height to Y
                    // terrain[i].height is 0..1
                    // Y = BASELINE_Y - height * SCALE
                    // But we generated terrain with POINTS_PER_CHAR scaling implicitly in X
                    // Let's assume World Width maps to Terrain Length

                    // Re-calculate x index based on screen mapping
                    // We assume the terrain fits in WORLD_WIDTH for now?
                    // No, `terrain_width_px` might be larger.
                    // Let's map pos.x to terrain index.

                    let idx = ((pos.x / WORLD_WIDTH) * terrain_len as f32) as usize;
                    if idx < terrain_len {
                        let ground_y = BASELINE_Y - self.terrain[idx].height * TERRAIN_SCALE_Y;

                        // Seek the ground if it is high (food)
                        if self.terrain[idx].height > 0.5 {
                            // Dive!
                            vel.y += 0.2;
                        }

                        if pos.y >= ground_y {
                            // Landed / Eating
                            pos.y = ground_y;
                            vel = vec2(0.0, 0.0);
                            state = 1;
                            erosion_idx = Some(idx);
                        }
                    }
                }
            } else {
                // Eating state
                // Fly away after a bit?
                let mut rng = ::rand::thread_rng();
                if rng.gen_bool(0.1) {
                    state = 0;
                    vel.y = -5.0; // Jump up
                    vel.x = rng.gen_range(-2.0..2.0);
                } else {
                    erosion_idx = Some(terrain_idx.min(terrain_len - 1));
                }
            }

            // Bounds
            if pos.x < 0.0 { pos.x = WORLD_WIDTH; }
            if pos.x > WORLD_WIDTH { pos.x = 0.0; }
            if pos.y < 0.0 { pos.y = 0.0; vel.y *= -0.5; }
            if pos.y > WORLD_HEIGHT { pos.y = WORLD_HEIGHT; vel.y = 0.0; state = 0; vel.y = -5.0; } // Reset from bottom

            pos += vel;
            vel *= 0.98; // Drag

            (pos, vel, state, erosion_idx)
        }).collect();

        // Apply updates
        for (i, (pos, vel, state, erosion_idx)) in agents_state.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;

            if let Some(idx) = erosion_idx {
                if idx < self.terrain.len() {
                    // Erode
                    // Harder phonemes erode slower
                    let hardness = self.terrain[idx].hardness;
                    let erosion_amount = 0.005 * (1.1 - hardness); // 0.1 to 1.0 hardness
                    self.terrain[idx].height = (self.terrain[idx].height - erosion_amount).max(0.0);
                }
            }
        }

        // Reconstruct Text periodically
        if (self.time * 60.0) as usize % 60 == 0 {
            self.reconstruct_text();
        }
    }

    fn reconstruct_text(&mut self) {
        let mut s = String::new();
        let terrain_len = self.terrain.len();

        let mut i = POINTS_PER_CHAR; // Skip padding
        while i < terrain_len.saturating_sub(POINTS_PER_CHAR) {
            // Sample the "center" of the char
            // Since we expanded each char to POINTS_PER_CHAR points
            // The center is roughly at i + POINTS_PER_CHAR / 2

            let center_idx = i + POINTS_PER_CHAR / 2;
            if center_idx < terrain_len {
                let point = &self.terrain[center_idx];
                let c = terrain_to_char(point);
                if c != ' ' {
                    s.push(c);
                } else {
                    // Maybe stick a space if it was a space before?
                    // For now just skip
                    s.push(' ');
                }
            }
            i += POINTS_PER_CHAR;
        }
        self.current_text = s.trim().to_string();
    }
}
