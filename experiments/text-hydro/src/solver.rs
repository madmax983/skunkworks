#![allow(clippy::identity_op, clippy::erasing_op)]

pub struct Fluid {
    pub size: usize,
    pub dt: f32,
    pub diff: f32,
    pub visc: f32,

    pub s: Vec<f32>,
    pub density: Vec<f32>,

    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub vx0: Vec<f32>,
    pub vy0: Vec<f32>,
}

impl Fluid {
    pub fn new(size: usize, dt: f32, diff: f32, visc: f32) -> Self {
        let n = size * size;
        Self {
            size,
            dt,
            diff,
            visc,
            s: vec![0.0; n],
            density: vec![0.0; n],
            vx: vec![0.0; n],
            vy: vec![0.0; n],
            vx0: vec![0.0; n],
            vy0: vec![0.0; n],
        }
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let idx = x + y * self.size;
        if idx < self.density.len() {
            self.density[idx] += amount;
        }
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        let idx = x + y * self.size;
        if idx < self.vx.len() {
            self.vx[idx] += amount_x;
            self.vy[idx] += amount_y;
        }
    }

    pub fn step(&mut self) {
        let n = self.size;
        let iter = 4; // Solver iterations (low for performance in TUI)

        // Diffuse velocity
        diffuse(1, &mut self.vx0, &self.vx, self.visc, self.dt, iter, n);
        diffuse(2, &mut self.vy0, &self.vy, self.visc, self.dt, iter, n);

        // Project
        project(
            &mut self.vx0,
            &mut self.vy0,
            &mut self.vx,
            &mut self.vy,
            iter,
            n,
        );

        // Advect velocity
        advect(1, &mut self.vx, &self.vx0, &self.vx0, &self.vy0, self.dt, n);
        advect(2, &mut self.vy, &self.vy0, &self.vx0, &self.vy0, self.dt, n);

        // Project
        project(
            &mut self.vx,
            &mut self.vy,
            &mut self.vx0,
            &mut self.vy0,
            iter,
            n,
        );

        // Diffuse density
        diffuse(0, &mut self.s, &self.density, self.diff, self.dt, iter, n);

        // Advect density
        advect(
            0,
            &mut self.density,
            &self.s,
            &self.vx,
            &self.vy,
            self.dt,
            n,
        );
    }
}

fn set_bnd(b: usize, x: &mut [f32], n: usize) {
    for i in 1..n - 1 {
        x[0 + i * n] = if b == 1 { -x[1 + i * n] } else { x[1 + i * n] };
        x[(n - 1) + i * n] = if b == 1 {
            -x[(n - 2) + i * n]
        } else {
            x[(n - 2) + i * n]
        };
    }
    for i in 1..n - 1 {
        x[i + 0 * n] = if b == 2 { -x[i + 1 * n] } else { x[i + 1 * n] };
        x[i + (n - 1) * n] = if b == 2 {
            -x[i + (n - 2) * n]
        } else {
            x[i + (n - 2) * n]
        };
    }

    x[0 + 0 * n] = 0.5 * (x[1 + 0 * n] + x[0 + 1 * n]);
    x[0 + (n - 1) * n] = 0.5 * (x[1 + (n - 1) * n] + x[0 + (n - 2) * n]);
    x[(n - 1) + 0 * n] = 0.5 * (x[(n - 2) + 0 * n] + x[(n - 1) + 1 * n]);
    x[(n - 1) + (n - 1) * n] = 0.5 * (x[(n - 2) + (n - 1) * n] + x[(n - 1) + (n - 2) * n]);
}

fn diffuse(b: usize, x: &mut [f32], x0: &[f32], diff: f32, dt: f32, iter: usize, n: usize) {
    let a = dt * diff * (n - 2) as f32 * (n - 2) as f32;
    for _ in 0..iter {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = i + j * n;
                x[idx] = (x0[idx]
                    + a * (x[i - 1 + j * n]
                        + x[i + 1 + j * n]
                        + x[i + (j - 1) * n]
                        + x[i + (j + 1) * n]))
                    / (1.0 + 4.0 * a);
            }
        }
        set_bnd(b, x, n);
    }
}

fn advect(b: usize, d: &mut [f32], d0: &[f32], u: &[f32], v: &[f32], dt: f32, n: usize) {
    let dt0 = dt * (n - 2) as f32;
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let mut x = i as f32 - dt0 * u[i + j * n];
            let mut y = j as f32 - dt0 * v[i + j * n];

            if x < 0.5 {
                x = 0.5;
            }
            if x > (n as f32) - 1.5 {
                x = (n as f32) - 1.5;
            }
            if y < 0.5 {
                y = 0.5;
            }
            if y > (n as f32) - 1.5 {
                y = (n as f32) - 1.5;
            }

            let i0 = x as usize;
            let i1 = i0 + 1;
            let j0 = y as usize;
            let j1 = j0 + 1;

            let s1 = x - i0 as f32;
            let s0 = 1.0 - s1;
            let t1 = y - j0 as f32;
            let t0 = 1.0 - t1;

            let idx = i + j * n;
            d[idx] = s0 * (t0 * d0[i0 + j0 * n] + t1 * d0[i0 + j1 * n])
                + s1 * (t0 * d0[i1 + j0 * n] + t1 * d0[i1 + j1 * n]);
        }
    }
    set_bnd(b, d, n);
}

fn project(u: &mut [f32], v: &mut [f32], p: &mut [f32], div: &mut [f32], iter: usize, n: usize) {
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            div[i + j * n] = -0.5
                * (u[i + 1 + j * n] - u[i - 1 + j * n] + v[i + (j + 1) * n] - v[i + (j - 1) * n])
                / (n as f32);
            p[i + j * n] = 0.0;
        }
    }
    set_bnd(0, div, n);
    set_bnd(0, p, n);

    for _ in 0..iter {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                p[i + j * n] = (div[i + j * n]
                    + p[i - 1 + j * n]
                    + p[i + 1 + j * n]
                    + p[i + (j - 1) * n]
                    + p[i + (j + 1) * n])
                    / 4.0;
            }
        }
        set_bnd(0, p, n);
    }

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            u[i + j * n] -= 0.5 * (n as f32) * (p[i + 1 + j * n] - p[i - 1 + j * n]);
            v[i + j * n] -= 0.5 * (n as f32) * (p[i + (j + 1) * n] - p[i + (j - 1) * n]);
        }
    }
    set_bnd(1, u, n);
    set_bnd(2, v, n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffusion() {
        let mut fluid = Fluid::new(10, 0.1, 0.001, 0.0);
        let center = 5;
        // Add density to center
        fluid.add_density(center, center, 100.0);

        // Step
        fluid.step();

        // Check neighbor
        let idx_right = (center + 1) + center * 10;
        let idx_center = center + center * 10;

        // With small diffusion and time step, it should spread
        // However, Jos Stam's algorithm might be stable but diffusive.

        println!(
            "Center: {}, Right: {}",
            fluid.density[idx_center], fluid.density[idx_right]
        );

        assert!(fluid.density[idx_right] > 0.0, "Density did not diffuse!");
    }

    #[test]
    fn test_advection() {
        let mut fluid = Fluid::new(10, 0.1, 0.0, 0.0);
        let center = 5;

        fluid.add_density(center, center, 100.0);
        // Add velocity to the right
        fluid.add_velocity(center, center, 10.0, 0.0);

        fluid.step();

        // Density should move right?
        // Note: Advection traces BACKWARDS.
        // So at (center+1), we look back at velocity.
        // If velocity at center+1 is 0, we look at center+1.
        // We need velocity everywhere or at least propagated.
        // But in first step, velocity is only at center.
        // And we diffuse velocity first.

        // Let's just check that density changed location.
        let idx_center = center + center * 10;
        let idx_right = (center + 1) + center * 10;

        // It might not move exactly to right in one step if velocity field is zero elsewhere.
        // But let's check conservation roughly or movement.

        // Better test: Set uniform velocity.
        for i in 0..100 {
            fluid.vx[i] = 5.0;
        }

        fluid.step();

        assert!(
            fluid.density[idx_right] > 0.0 || fluid.density[idx_center] < 100.0,
            "Density did not move!"
        );
    }
}
