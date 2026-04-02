use crate::leaf::LeafMap;
use rand::Rng;

const EROSION_RATE: f32 = 0.005;
const MAX_PATH: usize = 100;

pub struct Seed {
    pub species: String,
    pub dna_source: String,
}

pub fn erode_step(
    map: &mut LeafMap,
    drops: usize,
    seeds: &[Seed],
    new_plants: &mut Vec<(usize, usize, usize)>,
) {
    let mut rng = rand::thread_rng();

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
}

pub fn erode_at(map: &mut LeafMap, start_x: usize, start_y: usize) -> Option<(usize, usize)> {
    // Ensure inside mask
    if !map.is_inside(start_x, start_y) {
        return None;
    }

    let mut x = start_x;
    let mut y = start_y;
    let width = map.width;
    let height = map.height;
    let width_i = width as isize;

    // Precompute offsets for neighbor checking
    // Order: TL, T, TR, L, R, BL, B, BR
    let offsets = [
        -width_i - 1,
        -width_i,
        -width_i + 1,
        -1,
        1,
        width_i - 1,
        width_i,
        width_i + 1,
    ];

    // Corresponding dx, dy for updating position
    let delta_x = [-1, 0, 1, -1, 1, -1, 0, 1];
    let delta_y = [-1, -1, -1, 0, 0, 1, 1, 1];

    // Flow Downhill
    for _step in 0..MAX_PATH {
        let idx = y * width + x;

        // Accumulate water visualization
        map.water[idx] += 0.1;
        if map.water[idx] > 1.0 {
            map.water[idx] = 1.0;
        }

        // Find lowest neighbor
        let mut min_h = map.heightmap[idx];
        let mut best_dx = 0;
        let mut best_dy = 0;
        let mut found_lower = false;

        // Fast path for non-edge pixels (avoids repeated boundary checks and coord calculations)
        if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
            for i in 0..8 {
                // Safety: x,y bounds check ensures idx+offset is within [0, width*height)
                let n_idx = (idx as isize + offsets[i]) as usize;

                if map.mask[n_idx] {
                    let h = map.heightmap[n_idx];
                    if h < min_h {
                        min_h = h;
                        best_dx = delta_x[i];
                        best_dy = delta_y[i];
                        found_lower = true;
                    }
                }
            }
        } else {
            // Slow path (edge cases) with full bounds checking
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        let n_idx = ny * width + nx;

                        if map.mask[n_idx] {
                            let h = map.heightmap[n_idx];
                            if h < min_h {
                                min_h = h;
                                best_dx = dx;
                                best_dy = dy;
                                found_lower = true;
                            }
                        }
                    }
                }
            }
        }

        if found_lower {
            let slope = map.heightmap[idx] - min_h;
            map.heightmap[idx] -= EROSION_RATE * slope.max(0.1);
            x = (x as isize + best_dx) as usize;
            y = (y as isize + best_dy) as usize;
        } else {
            // Local minimum (pool)
            map.heightmap[idx] += EROSION_RATE * 0.5;
            return Some((x, y));
        }
    }
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaf::LeafMap;
    use std::time::Instant;

    #[test]
    fn test_erode_step_perf() {
        let width = 200;
        let height = 200;
        let mut map = LeafMap::new(width, height);
        map.generate_shape();
        let seeds = vec![];
        let mut plants_buffer = Vec::new();

        let start = Instant::now();
        // Run 100 times * 1000 drops = 100,000 drops
        for _ in 0..100 {
            plants_buffer.clear();
            erode_step(&mut map, 1000, &seeds, &mut plants_buffer);
        }
        println!("test_erode_step_perf elapsed: {:?}", start.elapsed());
    }
}
