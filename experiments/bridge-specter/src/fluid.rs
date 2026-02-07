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
        }
    }

    fn ix(x: usize, y: usize, n: usize) -> usize {
        let x = x.clamp(0, n - 1);
        let y = y.clamp(0, n - 1);
        x + y * n
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let idx = Self::ix(x, y, self.size);
        self.density[idx] += amount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        let idx = Self::ix(x, y, self.size);
        self.vx[idx] += amount_x;
        self.vy[idx] += amount_y;
    }

    pub fn step(&mut self) {
        let n = self.size;
        let visc = self.visc;
        let diff = self.diff;
        let dt = self.dt;

        // Diffuse velocity
        Self::diffuse(n, 1, &mut self.vx0, &self.vx, visc, dt);
        Self::diffuse(n, 2, &mut self.vy0, &self.vy, visc, dt);

        // Project
        Self::project(n, &mut self.vx0, &mut self.vy0, &mut self.vx, &mut self.vy);

        // Advect velocity
        Self::advect(n, 1, &mut self.vx, &self.vx0, &self.vx0, &self.vy0, dt);
        Self::advect(n, 2, &mut self.vy, &self.vy0, &self.vx0, &self.vy0, dt);

        // Project
        Self::project(n, &mut self.vx, &mut self.vy, &mut self.vx0, &mut self.vy0);

        // Diffuse density
        Self::diffuse(n, 0, &mut self.s, &self.density, diff, dt);

        // Advect density
        Self::advect(n, 0, &mut self.density, &self.s, &self.vx, &self.vy, dt);
    }

    fn diffuse(n: usize, b: i32, x: &mut [f32], x0: &[f32], diff: f32, dt: f32) {
        let a = dt * diff * (n - 2) as f32 * (n - 2) as f32;
        Self::lin_solve(n, b, x, x0, a, 1.0 + 4.0 * a);
    }

    fn lin_solve(n: usize, b: i32, x: &mut [f32], x0: &[f32], a: f32, c: f32) {
        let c_recip = 1.0 / c;

        for _ in 0..4 {
            for j in 1..n - 1 {
                for i in 1..n - 1 {
                    let idx = Self::ix(i, j, n);
                    x[idx] = (x0[idx]
                        + a * (x[Self::ix(i + 1, j, n)]
                            + x[Self::ix(i - 1, j, n)]
                            + x[Self::ix(i, j + 1, n)]
                            + x[Self::ix(i, j - 1, n)]))
                        * c_recip;
                }
            }
            Self::set_bnd(n, b, x);
        }
    }

    fn project(n: usize, veloc_x: &mut [f32], veloc_y: &mut [f32], p: &mut [f32], div: &mut [f32]) {
        let h = 1.0 / n as f32;

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
                div[idx] = -0.5
                    * h
                    * (veloc_x[Self::ix(i + 1, j, n)] - veloc_x[Self::ix(i - 1, j, n)]
                        + veloc_y[Self::ix(i, j + 1, n)]
                        - veloc_y[Self::ix(i, j - 1, n)]);
                p[idx] = 0.0;
            }
        }

        Self::set_bnd(n, 0, div);
        Self::set_bnd(n, 0, p);

        Self::lin_solve(n, 0, p, div, 1.0, 4.0);

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
                veloc_x[idx] -= 0.5 * (p[Self::ix(i + 1, j, n)] - p[Self::ix(i - 1, j, n)]) / h;
                veloc_y[idx] -= 0.5 * (p[Self::ix(i, j + 1, n)] - p[Self::ix(i, j - 1, n)]) / h;
            }
        }

        Self::set_bnd(n, 1, veloc_x);
        Self::set_bnd(n, 2, veloc_y);
    }

    fn advect(
        n: usize,
        b: i32,
        d: &mut [f32],
        d0: &[f32],
        veloc_x: &[f32],
        veloc_y: &[f32],
        dt: f32,
    ) {
        let dt0 = dt * (n - 2) as f32;

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = Self::ix(i, j, n);
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

                d[idx] = s0 * (t0 * d0[Self::ix(i0, j0, n)] + t1 * d0[Self::ix(i0, j1, n)])
                    + s1 * (t0 * d0[Self::ix(i1, j0, n)] + t1 * d0[Self::ix(i1, j1, n)]);
            }
        }

        Self::set_bnd(n, b, d);
    }

    fn set_bnd(n: usize, b: i32, x: &mut [f32]) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_step() {
        let mut fluid = FluidSolver::new(10, 0.1, 0.001, 0.001);
        fluid.add_density(5, 5, 1.0);
        // We can't access ix directly as it is private, need to expose or duplicate logic for test
        // But since we copy pasted, tests will fail to access private method ix if we use it here.
        // Wait, the test uses FluidSolver::ix. But ix is private.
        // Ah, in Rust unit tests (mod tests) inside the same file can access private items of parent module.
        // BUT `ix` is an associated function, so it needs to be called as `FluidSolver::ix` or just `ix` if imported?
        // It is `fn ix`.

        // Actually, let's fix the test code to use a public method or just rely on public API if possible.
        // But `ix` is useful for checking index.
        // Since I'm pasting the code, I can make `ix` public for crate or just keep it as is.
        // The original code had `fn ix`.

        // Let's rely on `add_density` and check internal state.

        fluid.step();

        // Add velocity to see movement
        fluid.add_velocity(5, 5, 1.0, 0.0);
        fluid.step();

        // Check for NaN
        for d in &fluid.density {
            assert!(!d.is_nan(), "Density contains NaN");
        }
    }
}
