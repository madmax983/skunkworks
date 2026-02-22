pub const GRID_SIZE: usize = 6; // 6^4 = 1296 cells.

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
pub struct AcousticGrid4D {
    pub size: usize,
    pub u: Vec<f32>,
    pub u_prev: Vec<f32>,
    pub u_next: Vec<f32>,
    pub agent_damping: Vec<f32>, // 0.0 = full stop, 1.0 = pass through
    pub total_energy: f32,
}

impl AcousticGrid4D {
    pub fn new() -> Self {
        let len = GRID_SIZE * GRID_SIZE * GRID_SIZE * GRID_SIZE;
        Self {
            size: GRID_SIZE,
            u: vec![0.0; len],
            u_prev: vec![0.0; len],
            u_next: vec![0.0; len],
            agent_damping: vec![1.0; len],
            total_energy: 0.0,
        }
    }

    pub fn idx(&self, p: Point4D) -> usize {
        p.w * self.size * self.size * self.size
            + p.z * self.size * self.size
            + p.y * self.size
            + p.x
    }

    pub fn idx_raw(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        w * self.size * self.size * self.size
            + z * self.size * self.size
            + y * self.size
            + x
    }

    #[inline]
    fn compute_laplacian(&self, idx: usize, s: usize, s2: usize, s3: usize) -> f32 {
        let u_curr = self.u[idx];
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

    pub fn step(&mut self, c2: f32, base_damping: f32) {
        let s = self.size;
        let s2 = s * s;
        let s3 = s * s * s;

        self.total_energy = 0.0;

        for w in 1..s - 1 {
            for z in 1..s - 1 {
                for y in 1..s - 1 {
                    for x in 1..s - 1 {
                        let idx = w * s3 + z * s2 + y * s + x;

                        let laplacian = self.compute_laplacian(idx, s, s2, s3);

                        // Wave Equation
                        let mut val = 2.0 * self.u[idx] - self.u_prev[idx] + c2 * laplacian;

                        // Apply Damping (Base + Agent)
                        val *= base_damping * self.agent_damping[idx];

                        self.u_next[idx] = val;
                        self.total_energy += val.abs();
                    }
                }
            }
        }

        std::mem::swap(&mut self.u_prev, &mut self.u);
        std::mem::swap(&mut self.u, &mut self.u_next);
    }

    pub fn pluck(&mut self, p: Point4D, strength: f32) {
        if p.x > 0 && p.x < self.size - 1 &&
           p.y > 0 && p.y < self.size - 1 &&
           p.z > 0 && p.z < self.size - 1 &&
           p.w > 0 && p.w < self.size - 1
        {
            let idx = self.idx(p);
            self.u[idx] += strength;
        }
    }

    pub fn set_agent_damping(&mut self, p: Point4D, factor: f32) {
        if p.x < self.size && p.y < self.size && p.z < self.size && p.w < self.size {
            let idx = self.idx(p);
            self.agent_damping[idx] = factor;
        }
    }

    pub fn clear_agent_damping(&mut self) {
        self.agent_damping.fill(1.0);
    }
}
