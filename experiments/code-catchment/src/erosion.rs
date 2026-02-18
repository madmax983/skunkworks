use bevy::prelude::*;
use crate::terrain::TerrainMap;
use rand::prelude::*;

#[derive(Resource)]
pub struct ErosionConfig {
    pub gravity: f32,
    pub erosion_rate: f32,
    pub deposition_rate: f32,
    pub evaporation_rate: f32,
    pub min_slope: f32,
    pub inertia: f32,
    pub capacity_factor: f32,
    pub max_steps: usize,
}

impl Default for ErosionConfig {
    fn default() -> Self {
        Self {
            gravity: 4.0,
            erosion_rate: 0.1,
            deposition_rate: 0.1,
            evaporation_rate: 0.02,
            min_slope: 0.05,
            inertia: 0.1,
            capacity_factor: 8.0,
            max_steps: 64, // Lifetime of a droplet
        }
    }
}

#[derive(Clone, Copy)]
pub struct Droplet {
    pub pos: Vec2,
    pub dir: Vec2,
    pub speed: f32,
    pub water: f32,
    pub sediment: f32,
    pub steps: usize,
}

impl Droplet {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            dir: Vec2::ZERO,
            speed: 0.0,
            water: 1.0,
            sediment: 0.0,
            steps: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct ErosionState {
    pub droplets: Vec<Droplet>,
}

pub fn erosion_system(
    mut terrain: ResMut<TerrainMap>,
    mut state: ResMut<ErosionState>,
    config: Res<ErosionConfig>,
) {
    let w = terrain.width as i32;
    let h = terrain.height as i32;

    // We process droplets in chunks or all at once?
    // Let's process all existing droplets for 1 step per frame.
    // Or multiple steps per frame?
    // Usually erosion is fast. Let's do 1 step per droplet per frame for visualization.

    // Clear old water map for visualization
    terrain.water.fill(0.0);

    let mut dead_indices = Vec::new();

    for (i, droplet) in state.droplets.iter_mut().enumerate() {
        if droplet.steps >= config.max_steps || droplet.water < 0.01 {
            // Deposit remaining sediment
            if is_inside(droplet.pos, w, h) {
                 deposit(&mut terrain, droplet.pos, droplet.sediment);
            }
            dead_indices.push(i);
            continue;
        }

        // Mark water for visualization
        if is_inside(droplet.pos, w, h) {
            let idx = (droplet.pos.y as usize) * terrain.width + (droplet.pos.x as usize);
            if idx < terrain.water.len() {
                terrain.water[idx] += droplet.water;
            }
        }

        // 1. Calculate Gradient
        let gradient = calculate_gradient(&terrain, droplet.pos);

        // 2. Update Direction (Inertia)
        let mut new_dir = droplet.dir * config.inertia - gradient * (1.0 - config.inertia);
        if new_dir.length_squared() < 0.0001 {
            let mut rng = rand::thread_rng();
            new_dir = Vec2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5);
        }
        droplet.dir = new_dir.normalize_or_zero();

        // 3. Move
        let old_pos = droplet.pos;
        droplet.pos += droplet.dir;

        // Check bounds
        if !is_inside(droplet.pos, w, h) {
            dead_indices.push(i);
            continue;
        }

        // 4. Height Difference
        let old_height = get_height_interp(&terrain, old_pos);
        let new_height = get_height_interp(&terrain, droplet.pos);
        let diff = old_height - new_height;

        // 5. Capacity
        let capacity = diff.max(config.min_slope) * droplet.speed * droplet.water * config.capacity_factor;

        // 6. Erode/Deposit
        if droplet.sediment > capacity || diff < 0.0 {
            // Deposit
            let amount = (droplet.sediment - capacity) * config.deposition_rate;
            let amount = if diff < 0.0 { amount.max(-diff) } else { amount }; // Fill pit?

            droplet.sediment -= amount;
            deposit(&mut terrain, old_pos, amount);
        } else {
            // Erode
            let amount = (capacity - droplet.sediment) * config.erosion_rate;
            let amount = amount.min(diff); // Don't dig deeper than slope

            droplet.sediment += amount;
            erode(&mut terrain, old_pos, amount);
        }

        // 7. Update Speed & Water
        droplet.speed = (droplet.speed * droplet.speed + diff * config.gravity).abs().sqrt();
        droplet.water *= 1.0 - config.evaporation_rate;
        droplet.steps += 1;
    }

    // Remove dead droplets (in reverse order to keep indices valid, but swap_remove is easier)
    // Actually standard remove pattern:
    // Better: retain.
    // But Mut ref...
    // Let's iterate backwards and swap_remove.
    for i in dead_indices.iter().rev() {
        state.droplets.swap_remove(*i);
    }
}

fn is_inside(pos: Vec2, w: i32, h: i32) -> bool {
    pos.x >= 0.0 && pos.x < (w as f32 - 1.0) && pos.y >= 0.0 && pos.y < (h as f32 - 1.0)
}

fn get_height_interp(terrain: &TerrainMap, pos: Vec2) -> f32 {
    let x = pos.x.floor() as usize;
    let y = pos.y.floor() as usize;
    let dx = pos.x - x as f32;
    let dy = pos.y - y as f32;

    let w = terrain.width;
    let idx = y * w + x;

    // Bilinear interpolation
    let h00 = terrain.heights[idx];
    let h10 = terrain.heights[idx + 1];
    let h01 = terrain.heights[idx + w];
    let h11 = terrain.heights[idx + w + 1];

    let top = h00 * (1.0 - dx) + h10 * dx;
    let bot = h01 * (1.0 - dx) + h11 * dx;

    top * (1.0 - dy) + bot * dy
}

fn calculate_gradient(terrain: &TerrainMap, pos: Vec2) -> Vec2 {
    let x = pos.x.floor() as usize;
    let y = pos.y.floor() as usize;
    let dx = pos.x - x as f32;
    let dy = pos.y - y as f32;

    let w = terrain.width;
    let idx = y * w + x;

    let h00 = terrain.heights[idx];
    let h10 = terrain.heights[idx + 1];
    let h01 = terrain.heights[idx + w];
    let h11 = terrain.heights[idx + w + 1];

    let gx = (h10 - h00) * (1.0 - dy) + (h11 - h01) * dy;
    let gy = (h01 - h00) * (1.0 - dx) + (h11 - h10) * dx;

    Vec2::new(gx, gy)
}

fn deposit(terrain: &mut TerrainMap, pos: Vec2, amount: f32) {
    let x = pos.x.floor() as usize;
    let y = pos.y.floor() as usize;
    let dx = pos.x - x as f32;
    let dy = pos.y - y as f32;

    let w = terrain.width;
    let idx = y * w + x;

    // Distribute deposition
    terrain.heights[idx] += amount * (1.0 - dx) * (1.0 - dy);
    terrain.heights[idx + 1] += amount * dx * (1.0 - dy);
    terrain.heights[idx + w] += amount * (1.0 - dx) * dy;
    terrain.heights[idx + w + 1] += amount * dx * dy;

    // Track sediment for visualization
    terrain.sediment[idx] += amount * (1.0 - dx) * (1.0 - dy);
    terrain.sediment[idx + 1] += amount * dx * (1.0 - dy);
    terrain.sediment[idx + w] += amount * (1.0 - dx) * dy;
    terrain.sediment[idx + w + 1] += amount * dx * dy;
}

fn erode(terrain: &mut TerrainMap, pos: Vec2, amount: f32) {
    let x = pos.x.floor() as usize;
    let y = pos.y.floor() as usize;
    let dx = pos.x - x as f32;
    let dy = pos.y - y as f32;

    let w = terrain.width;
    let idx = y * w + x;

    // Distribute erosion
    terrain.heights[idx] -= amount * (1.0 - dx) * (1.0 - dy);
    terrain.heights[idx + 1] -= amount * dx * (1.0 - dy);
    terrain.heights[idx + w] -= amount * (1.0 - dx) * dy;
    terrain.heights[idx + w + 1] -= amount * dx * dy;

    // Erode from sediment layer first? No, simple heightmap mod.
}
