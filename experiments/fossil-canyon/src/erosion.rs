use crate::map::{Terrain, Voxel, VoxelType};
use macroquad::rand::gen_range;

pub fn erode(terrain: &mut Terrain, iterations: usize) {
    for _ in 0..iterations {
        erode_droplet(terrain);
    }
}

fn erode_droplet(terrain: &mut Terrain) {
    let mut x = gen_range(0, terrain.width);
    let mut y = gen_range(0, terrain.height);

    // Droplet properties
    let mut velocity: f32 = 1.0;
    let mut suspended_sediment: Option<Voxel> = None;
    let mut lifetime = 30;

    while lifetime > 0 {
        lifetime -= 1;

        let h = terrain.get_height(x, y) as f32;

        // Find lowest neighbor
        let mut min_h = h;
        let mut best_nx = x;
        let mut best_ny = y;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < terrain.width as i32 && ny >= 0 && ny < terrain.height as i32 {
                    let nh = terrain.get_height(nx as usize, ny as usize) as f32;
                    if nh < min_h {
                        min_h = nh;
                        best_nx = nx as usize;
                        best_ny = ny as usize;
                    }
                }
            }
        }

        let slope = h - min_h;

        if slope > 0.0 {
            // Flow downhill
            // Erode if moving fast or carrying little
            if velocity > 0.5 && suspended_sediment.is_none() {
                // Check if we can pick up voxel
                let hardness = if let Some(top_voxel) = terrain.peek_voxel(x, y) {
                     match top_voxel.voxel_type {
                        VoxelType::Bone => 0.9, // Very hard
                        VoxelType::Fossil => 0.7, // Hard
                        VoxelType::Rock => 0.5,
                        VoxelType::Sediment => 0.1, // Soft
                    }
                } else {
                    1.0 // Nothing to erode
                };

                if gen_range(0.0f32, 1.0f32) > hardness {
                     if let Some(voxel) = terrain.pop_voxel(x, y) {
                        suspended_sediment = Some(voxel);
                        velocity += slope; // Gain speed
                    }
                }
            }

            x = best_nx;
            y = best_ny;
            velocity *= 0.9; // Friction
        } else {
            // Flat or pit -> Deposit
            if let Some(voxel) = suspended_sediment {
                // Deposit at current location
                terrain.push_voxel(x, y, voxel);
                suspended_sediment = None;
            } else {
                 // Deposit logic if carrying nothing? No, just stop.
            }
            // Stop if stuck in pit
            break;
        }
    }

    // Deposit any remaining sediment at end of life
    if let Some(voxel) = suspended_sediment {
        terrain.push_voxel(x, y, voxel);
    }
}
