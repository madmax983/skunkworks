
pub const GRID_SIZE: usize = 6; // 6^4 = 1296 cells. 44.1kHz * 1296 = ~600M ops. Safer.

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

    pub fn idx(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        w * self.size * self.size * self.size + z * self.size * self.size + y * self.size + x
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

                        let u_curr = self.u[idx];
                        let u_prev = self.u_prev[idx];

                        // Neighbors
                        // Optimization: Precompute offsets or just use idx arithmetic
                        let left = idx - 1;
                        let right = idx + 1;
                        let up = idx - s;
                        let down = idx + s;
                        let front = idx - s2;
                        let back = idx + s2;
                        let ana = idx - s3; // "Ana" and "Kata" are 4D directions
                        let kata = idx + s3;

                        // 4D Laplacian
                        let laplacian = self.u[left] + self.u[right]
                            + self.u[up] + self.u[down]
                            + self.u[front] + self.u[back]
                            + self.u[ana] + self.u[kata]
                            - 8.0 * u_curr;

                        // Wave Equation Update
                        // u_next = 2*u - u_prev + c^2 * laplacian
                        // Damping applied to velocity or value
                        // Simple damping: multiply result by factor < 1.0
                        let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
                        val *= damping;

                        self.u_next[idx] = val;
                        self.total_energy += val.abs();
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

    pub fn pluck(&mut self, x: usize, y: usize, z: usize, w: usize, strength: f32) {
        if x > 0 && x < self.size - 1 &&
           y > 0 && y < self.size - 1 &&
           z > 0 && z < self.size - 1 &&
           w > 0 && w < self.size - 1 {
            let idx = self.idx(x, y, z, w);
            self.u[idx] += strength;
            // Add some to prev to give it momentum? Or just displacement.
            // Displacement is potential energy.
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize, z: usize, w: usize) -> f32 {
         if x < self.size && y < self.size && z < self.size && w < self.size {
            self.u[self.idx(x, y, z, w)]
        } else {
            0.0
        }
    }
}
