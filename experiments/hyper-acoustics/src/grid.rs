pub const GRID_SIZE: usize = 6; // 6^4 = 1296 cells. 44.1kHz * 1296 = ~600M ops. Safer.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point4D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
}

impl Point4D {
    pub fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
        Self { x, y, z, w }
    }
}

#[derive(Clone)]
pub struct PhysicsGrid4D {
    pub size: usize,
    pub u: Vec<f32>,
    pub u_prev: Vec<f32>,
    pub u_next: Vec<f32>,
    pub total_energy: f32,
}

impl PhysicsGrid4D {
    pub fn new() -> Self {
        let len = GRID_SIZE * GRID_SIZE * GRID_SIZE * GRID_SIZE;
        Self {
            size: GRID_SIZE,
            u: vec![0.0; len],
            u_prev: vec![0.0; len],
            u_next: vec![0.0; len],
            total_energy: 0.0,
        }
    }

    pub fn idx(&self, p: Point4D) -> usize {
        p.w * self.size * self.size * self.size
            + p.z * self.size * self.size
            + p.y * self.size
            + p.x
    }

    #[inline]
    fn compute_laplacian(&self, idx: usize, s: usize, s2: usize, s3: usize) -> f32 {
        let u_curr = self.u[idx];
        // Neighbors
        // We assume idx is safe (interior point) so offsets won't underflow/overflow
        self.u[idx - 1]
            + self.u[idx + 1]
            + self.u[idx - s]
            + self.u[idx + s]
            + self.u[idx - s2]
            + self.u[idx + s2]
            + self.u[idx - s3]
            + self.u[idx + s3]
            - 8.0 * u_curr
    }

    #[inline]
    fn update_cell(&mut self, idx: usize, laplacian: f32, c2: f32, damping: f32) {
        let u_curr = self.u[idx];
        let u_prev = self.u_prev[idx];

        // Wave Equation Update
        // u_next = 2*u - u_prev + c^2 * laplacian
        let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
        val *= damping;

        self.u_next[idx] = val;
        self.total_energy += val.abs();
    }

    pub fn step(&mut self, c2: f32, damping: f32) {
        let s = self.size;
        let s2 = s * s;
        let s3 = s * s * s;

        // Clear energy accumulator
        self.total_energy = 0.0;

        // Iterate interior
        // We skip boundaries (fixed at 0, acting as hard walls)
        for w in 1..s - 1 {
            for z in 1..s - 1 {
                for y in 1..s - 1 {
                    for x in 1..s - 1 {
                        let idx = w * s3 + z * s2 + y * s + x;

                        let laplacian = self.compute_laplacian(idx, s, s2, s3);
                        self.update_cell(idx, laplacian, c2, damping);
                    }
                }
            }
        }

        // Cycle buffers
        // u_prev <- u
        // u <- u_next
        // u_next is overwritten next step
        std::mem::swap(&mut self.u_prev, &mut self.u);
        std::mem::swap(&mut self.u, &mut self.u_next);
    }

    pub fn pluck(&mut self, p: Point4D, strength: f32) {
        if p.x > 0
            && p.x < self.size - 1
            && p.y > 0
            && p.y < self.size - 1
            && p.z > 0
            && p.z < self.size - 1
            && p.w > 0
            && p.w < self.size - 1
        {
            let idx = self.idx(p);
            self.u[idx] += strength;
            // Add some to prev to give it momentum? Or just displacement.
            // Displacement is potential energy.
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, p: Point4D) -> f32 {
        if p.x < self.size && p.y < self.size && p.z < self.size && p.w < self.size {
            self.u[self.idx(p)]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_behavior() {
        let mut grid = PhysicsGrid4D::new();
        // Pluck the center
        let center_coord = GRID_SIZE / 2;
        let center = Point4D::new(center_coord, center_coord, center_coord, center_coord);
        grid.pluck(center, 1.0);

        // Check initial state
        assert_eq!(grid.get(center), 1.0);

        // Perform one step
        // u_curr = 1.0, u_prev = 0.0, neighbors = 0.0
        // laplacian = 0 - 8*1.0 = -8.0
        // next = (2*1.0 - 0.0 + 0.5 * -8.0) * 0.9 = (2.0 - 4.0) * 0.9 = -2.0 * 0.9 = -1.8
        grid.step(0.5, 0.9);

        let val = grid.get(center);
        assert!((val + 1.8).abs() < 1e-6, "Expected -1.8, got {}", val);
    }
}
