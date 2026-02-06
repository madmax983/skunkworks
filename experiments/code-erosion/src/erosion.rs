use crate::terrain::{Terrain, GRID_SIZE};
use macroquad::rand::gen_range;

const INERTIA: f32 = 0.1;
const GRAVITY: f32 = 4.0;
const EVAPORATION: f32 = 0.02;
const CAPACITY: f32 = 4.0;
const MIN_SLOPE: f32 = 0.05;
const DEPOSITION_RATE: f32 = 0.1;
const EROSION_RATE: f32 = 0.1;

pub fn erode(terrain: &mut Terrain, iterations: usize) {
    for _ in 0..iterations {
        // Spawn drop at random position
        // Ideally we spawn drops based on rainfall distribution (e.g. commits)
        // For now, random rain + biased rain (will add arg later)
        let start_x = gen_range(0, GRID_SIZE - 1);
        let start_y = gen_range(0, GRID_SIZE - 1);

        let mut x = start_x as f32;
        let mut y = start_y as f32;
        let mut dir_x: f32 = 0.0;
        let mut dir_y: f32 = 0.0;
        let mut speed: f32 = 1.0;
        let mut water: f32 = 1.0;
        let mut sediment: f32 = 0.0;

        for _life in 0..30 {
            let node_x = x as usize;
            let node_y = y as usize;

            // Calculate gradient
            let (gx, gy) = calculate_gradient(terrain, node_x, node_y);

            // Update direction
            dir_x = dir_x * INERTIA - gx * (1.0 - INERTIA);
            dir_y = dir_y * INERTIA - gy * (1.0 - INERTIA);

            // Normalize
            let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
            if len != 0.0 {
                dir_x /= len;
                dir_y /= len;
            }

            x += dir_x;
            y += dir_y;

            if x < 0.0 || x >= (GRID_SIZE - 1) as f32 || y < 0.0 || y >= (GRID_SIZE - 1) as f32 {
                break;
            }

            // Height diff
            let old_h = terrain.get_height(node_x, node_y);
            let new_h = terrain.get_height(x as usize, y as usize);
            let diff = old_h - new_h;

            // Capacity
            let capacity = diff.max(MIN_SLOPE) * speed * water * CAPACITY;

            if sediment > capacity || diff < 0.0 {
                // Deposit
                let amount = (sediment - capacity) * DEPOSITION_RATE;
                let amount = if diff < 0.0 {
                    amount.max(sediment.min(-diff))
                } else {
                    amount
                };

                sediment -= amount;
                deposit(terrain, node_x, node_y, amount);
            } else {
                // Erode
                let amount = (capacity - sediment) * EROSION_RATE;
                let amount = amount.min(diff);

                sediment += amount;
                erode_terrain(terrain, node_x, node_y, amount);
            }

            speed = (speed * speed + diff.max(0.0) * GRAVITY).sqrt();
            water *= 1.0 - EVAPORATION;

            if water < 0.01 {
                break;
            }
        }
    }
}

fn calculate_gradient(terrain: &Terrain, u: usize, v: usize) -> (f32, f32) {
    let idx = v * GRID_SIZE + u;
    let h00 = terrain.heightmap[idx];
    let h10 = if u + 1 < GRID_SIZE {
        terrain.heightmap[idx + 1]
    } else {
        h00
    };
    let h01 = if v + 1 < GRID_SIZE {
        terrain.heightmap[idx + GRID_SIZE]
    } else {
        h00
    };
    // let h_10 = if u > 0 { terrain.heightmap[idx - 1] } else { h00 };
    // let h_01 = if v > 0 { terrain.heightmap[idx - GRID_SIZE] } else { h00 };

    // Simple forward difference
    (h10 - h00, h01 - h00)
}

fn deposit(terrain: &mut Terrain, x: usize, y: usize, amount: f32) {
    let idx = y * GRID_SIZE + x;
    terrain.heightmap[idx] += amount;
    terrain.sediment_map[idx] += amount;
}

fn erode_terrain(terrain: &mut Terrain, x: usize, y: usize, amount: f32) {
    let idx = y * GRID_SIZE + x;
    terrain.heightmap[idx] -= amount;
}
