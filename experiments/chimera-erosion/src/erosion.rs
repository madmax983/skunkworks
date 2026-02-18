use crate::leaf::LeafMap;
use rand::Rng;

const EROSION_RATE: f32 = 0.005;
const MAX_PATH: usize = 100;

pub struct Seed {
    pub species: String,
    pub dna_source: String,
}

pub fn erode_step(map: &mut LeafMap, drops: usize, seeds: &[Seed]) -> Vec<(usize, usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut new_plants = Vec::new();

    for _ in 0..drops {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);

        // Chance to pick up a seed (1%)
        let carried_seed_idx = if !seeds.is_empty() && rng.gen_bool(0.01) {
            Some(rng.gen_range(0..seeds.len()))
        } else {
            None
        };

        if let Some((final_x, final_y)) = erode_at(map, x, y) {
            if let Some(seed_idx) = carried_seed_idx {
                // Chance to plant at deposition site (10%)
                if rng.gen_bool(0.1) {
                    new_plants.push((final_x, final_y, seed_idx));
                }
            }
        }
    }
    new_plants
}

pub fn erode_at(map: &mut LeafMap, start_x: usize, start_y: usize) -> Option<(usize, usize)> {
    // Ensure inside mask
    if !map.is_inside(start_x, start_y) {
        return None;
    }

    let mut x = start_x;
    let mut y = start_y;

    // Flow Downhill
    for _step in 0..MAX_PATH {
        let idx = y * map.width + x;

        // Accumulate water visualization
        map.water[idx] += 0.1;
        if map.water[idx] > 1.0 {
            map.water[idx] = 1.0;
        }

        // Find lowest neighbor
        let mut best_x = x;
        let mut best_y = y;
        let mut min_h = map.heightmap[idx];

        // Check 8 neighbors
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < map.width as isize && ny >= 0 && ny < map.height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if map.mask[ny * map.width + nx] {
                        let h = map.heightmap[ny * map.width + nx];
                        if h < min_h {
                            min_h = h;
                            best_x = nx;
                            best_y = ny;
                        }
                    }
                }
            }
        }

        if best_x != x || best_y != y {
            let slope = map.heightmap[idx] - min_h;
            map.heightmap[idx] -= EROSION_RATE * slope.max(0.1);
            x = best_x;
            y = best_y;
        } else {
            // Local minimum (pool)
            map.heightmap[idx] += EROSION_RATE * 0.5;
            return Some((x, y));
        }
    }
    Some((x, y))
}
