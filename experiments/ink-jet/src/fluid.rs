pub struct FluidSolver {
    pub size: usize,
    pub dt: f32,
    pub diff: f32,
    pub visc: f32,

    pub density: Vec<f32>,
    pub s: Vec<f32>,

    pub vx: Vec<f32>,
    pub vy: Vec<f32>,

    pub vx0: Vec<f32>,
    pub vy0: Vec<f32>,

    pub obstacles: Vec<bool>,
}

impl FluidSolver {
    pub fn new(size: usize, dt: f32, diff: f32, visc: f32) -> Self {
        let len = size * size;
        Self {
            size,
            dt,
            diff,
            visc,
            density: vec![0.0; len],
            s: vec![0.0; len],
            vx: vec![0.0; len],
            vy: vec![0.0; len],
            vx0: vec![0.0; len],
            vy0: vec![0.0; len],
            obstacles: vec![false; len],
        }
    }

    fn ix(x: usize, y: usize, n: usize) -> usize {
        let x = x.clamp(0, n - 1);
        let y = y.clamp(0, n - 1);
        x + y * n
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize, is_obstacle: bool) {
        let idx = Self::ix(x, y, self.size);
        self.obstacles[idx] = is_obstacle;
        if is_obstacle {
            self.density[idx] = 0.0;
            self.vx[idx] = 0.0;
            self.vy[idx] = 0.0;
        }
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let idx = Self::ix(x, y, self.size);
        if !self.obstacles[idx] {
            self.density[idx] += amount;
        }
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        let idx = Self::ix(x, y, self.size);
        if !self.obstacles[idx] {
            self.vx[idx] += amount_x;
            self.vy[idx] += amount_y;
        }
    }

    pub fn step(&mut self) {
        let n = self.size;
        let visc = self.visc;
        let diff = self.diff;
        let dt = self.dt;

        // Diffuse velocity
        Self::diffuse(n, 1, &mut self.vx0, &self.vx, visc, dt, &self.obstacles);
        Self::diffuse(n, 2, &mut self.vy0, &self.vy, visc, dt, &self.obstacles);

        // Project
        Self::project(
            n,
            &mut self.vx0,
            &mut self.vy0,
            &mut self.vx,
            &mut self.vy,
            &self.obstacles,
        );

        // Advect velocity
        Self::advect(
            n,
            1,
            &mut self.vx,
            &self.vx0,
            &self.vx0,
            &self.vy0,
            dt,
            &self.obstacles,
        );
        Self::advect(
            n,
            2,
            &mut self.vy,
            &self.vy0,
            &self.vx0,
            &self.vy0,
            dt,
            &self.obstacles,
        );

        // Project
        Self::project(
            n,
            &mut self.vx,
            &mut self.vy,
            &mut self.vx0,
            &mut self.vy0,
            &self.obstacles,
        );

        // Diffuse density
        Self::diffuse(n, 0, &mut self.s, &self.density, diff, dt, &self.obstacles);

        // Advect density
        Self::advect(
            n,
            0,
            &mut self.density,
            &self.s,
            &self.vx,
            &self.vy,
            dt,
            &self.obstacles,
        );
    }

    fn diffuse(
        n: usize,
        b: i32,
        x: &mut [f32],
        x0: &[f32],
        diff: f32,
        dt: f32,
        obstacles: &[bool],
    ) {
        let a = dt * diff * (n - 2) as f32 * (n - 2) as f32;
        Self::lin_solve(n, b, x, x0, a, 1.0 + 4.0 * a, obstacles);
    }

    fn lin_solve(n: usize, b: i32, x: &mut [f32], x0: &[f32], a: f32, c: f32, obstacles: &[bool]) {
        let c_recip = 1.0 / c;

        for _ in 0..4 {
            for j in 1..n - 1 {
                for i in 1..n - 1 {
                    let idx = Self::ix(i, j, n);
                    if obstacles[idx] {
                        x[idx] = 0.0;
                        continue;
                    }

                    x[idx] = (x0[idx]
                        + a * (x[Self::ix(i + 1, j, n)]
                            + x[Self::ix(i - 1, j, n)]
                            + x[Self::ix(i, j + 1, n)]
                            + x[Self::ix(i, j - 1, n)]))
                        * c_recip;
                }
            }
            Self::set_bnd(n, b, x, obstacles);
        }
    }

    fn project(
        n: usize,
        veloc_x: &mut [f32],
        veloc_y: &mut [f32],
        p: &mut [f32],
        div: &mut [f32],
        obstacles: &[bool],
    ) {
        let h = 1.0 / n as f32;

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
                if obstacles[idx] {
                    div[idx] = 0.0;
                    p[idx] = 0.0;
                    continue;
                }
                div[idx] = -0.5
                    * h
                    * (veloc_x[Self::ix(i + 1, j, n)] - veloc_x[Self::ix(i - 1, j, n)]
                        + veloc_y[Self::ix(i, j + 1, n)]
                        - veloc_y[Self::ix(i, j - 1, n)]);
                p[idx] = 0.0;
            }
        }

        Self::set_bnd(n, 0, div, obstacles);
        Self::set_bnd(n, 0, p, obstacles);

        Self::lin_solve(n, 0, p, div, 1.0, 4.0, obstacles);

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
                if obstacles[idx] {
                    veloc_x[idx] = 0.0;
                    veloc_y[idx] = 0.0;
                    continue;
                }
                veloc_x[idx] -= 0.5 * (p[Self::ix(i + 1, j, n)] - p[Self::ix(i - 1, j, n)]) / h;
                veloc_y[idx] -= 0.5 * (p[Self::ix(i, j + 1, n)] - p[Self::ix(i, j - 1, n)]) / h;
            }
        }

        Self::set_bnd(n, 1, veloc_x, obstacles);
        Self::set_bnd(n, 2, veloc_y, obstacles);
    }

    fn advect(
        n: usize,
        b: i32,
        d: &mut [f32],
        d0: &[f32],
        veloc_x: &[f32],
        veloc_y: &[f32],
        dt: f32,
        obstacles: &[bool],
    ) {
        let dt0 = dt * (n - 2) as f32;

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
                if obstacles[idx] {
                    d[idx] = 0.0; // Clear density/velocity inside obstacle
                    continue;
                }

                let mut x = i as f32 - dt0 * veloc_x[idx];
                let mut y = j as f32 - dt0 * veloc_y[idx];

                if x < 0.5 {
                    x = 0.5;
                }
                if x > n as f32 - 1.5 {
                    x = n as f32 - 1.5;
                }
                if y < 0.5 {
                    y = 0.5;
                }
                if y > n as f32 - 1.5 {
                    y = n as f32 - 1.5;
                }

                let i0 = x as usize;
                let i1 = i0 + 1;
                let j0 = y as usize;
                let j1 = j0 + 1;

                let s1 = x - i0 as f32;
                let s0 = 1.0 - s1;
                let t1 = y - j0 as f32;
                let t0 = 1.0 - t1;

                // If any source cell is an obstacle, treat as boundary?
                // For simplicity, just sample. If sample hits obstacle (which has 0 value), it works fine.
                // But obstacle density is 0.

                d[idx] = s0 * (t0 * d0[Self::ix(i0, j0, n)] + t1 * d0[Self::ix(i0, j1, n)])
                    + s1 * (t0 * d0[Self::ix(i1, j0, n)] + t1 * d0[Self::ix(i1, j1, n)]);
            }
        }

        Self::set_bnd(n, b, d, obstacles);
    }

    fn set_bnd(n: usize, b: i32, x: &mut [f32], obstacles: &[bool]) {
        for i in 1..n - 1 {
            x[Self::ix(i, 0, n)] = if b == 2 {
                -x[Self::ix(i, 1, n)]
            } else {
                x[Self::ix(i, 1, n)]
            };
            x[Self::ix(i, n - 1, n)] = if b == 2 {
                -x[Self::ix(i, n - 2, n)]
            } else {
                x[Self::ix(i, n - 2, n)]
            };
        }

        for j in 1..n - 1 {
            x[Self::ix(0, j, n)] = if b == 1 {
                -x[Self::ix(1, j, n)]
            } else {
                x[Self::ix(1, j, n)]
            };
            x[Self::ix(n - 1, j, n)] = if b == 1 {
                -x[Self::ix(n - 2, j, n)]
            } else {
                x[Self::ix(n - 2, j, n)]
            };
        }

        x[Self::ix(0, 0, n)] = 0.5 * (x[Self::ix(1, 0, n)] + x[Self::ix(0, 1, n)]);
        x[Self::ix(0, n - 1, n)] = 0.5 * (x[Self::ix(1, n - 1, n)] + x[Self::ix(0, n - 2, n)]);
        x[Self::ix(n - 1, 0, n)] = 0.5 * (x[Self::ix(n - 2, 0, n)] + x[Self::ix(n - 1, 1, n)]);
        x[Self::ix(n - 1, n - 1, n)] =
            0.5 * (x[Self::ix(n - 2, n - 1, n)] + x[Self::ix(n - 1, n - 2, n)]);

        // Enforce obstacles
        for j in 0..n {
            for i in 0..n {
                let idx = Self::ix(i, j, n);
                if obstacles[idx] {
                    x[idx] = 0.0;
                }
            }
        }
    }
}
