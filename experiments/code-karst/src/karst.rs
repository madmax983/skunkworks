use macroquad::prelude::*;

pub const GRID_SIZE: usize = 100;

pub struct VoxelGrid {
    pub density: Vec<f32>, // Flattened array
}

#[allow(dead_code)]
impl VoxelGrid {
    pub fn new() -> Self {
        Self {
            density: vec![1.0; GRID_SIZE * GRID_SIZE * GRID_SIZE],
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= GRID_SIZE || y >= GRID_SIZE || z >= GRID_SIZE {
            return 0.0;
        }
        self.density[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: f32) {
        if x >= GRID_SIZE || y >= GRID_SIZE || z >= GRID_SIZE {
            return;
        }
        self.density[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE] = val;
    }

    pub fn erode_at(&mut self, pos: Vec3, amount: f32, radius: f32) {
        let cx = pos.x as i32;
        let cy = pos.y as i32;
        let cz = pos.z as i32;

        let r_int = radius.ceil() as i32;

        for z in (cz - r_int)..=(cz + r_int) {
            for y in (cy - r_int)..=(cy + r_int) {
                for x in (cx - r_int)..=(cx + r_int) {
                    if x >= 0 && x < GRID_SIZE as i32 &&
                       y >= 0 && y < GRID_SIZE as i32 &&
                       z >= 0 && z < GRID_SIZE as i32 {

                        let dx = x as f32 - pos.x;
                        let dy = y as f32 - pos.y;
                        let dz = z as f32 - pos.z;
                        let dist_sq = dx*dx + dy*dy + dz*dz;

                        if dist_sq <= radius * radius {
                            let idx = x as usize + y as usize * GRID_SIZE + z as usize * GRID_SIZE * GRID_SIZE;
                            self.density[idx] = (self.density[idx] - amount).max(0.0);
                        }
                    }
                }
            }
        }
    }

    pub fn erode_path(&mut self, start: Vec3, end: Vec3, amount: f32) {
        let dist = start.distance(end);
        if dist < 0.1 { return; }

        let steps = (dist * 2.0) as usize; // 2 steps per unit
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let pos = start.lerp(end, t);
            self.erode_at(pos, amount, 1.5);
        }
    }
}
