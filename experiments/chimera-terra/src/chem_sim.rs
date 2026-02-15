use rand::prelude::*;
use rayon::prelude::*;

pub struct ChemicalState {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub next_u: Vec<f32>,
    pub next_v: Vec<f32>,
}

impl ChemicalState {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![1.0; size],
            v: vec![0.0; size],
            next_u: vec![1.0; size],
            next_v: vec![0.0; size],
        }
    }

    pub fn seed_noise(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.v.len() {
            if rng.r#gen::<f32>() > 0.99 {
                self.v[i] = 1.0;
            }
        }

        // Add a solid block in the middle
        let cx = self.width / 2;
        let cy = self.height / 2;
        let r = 10;
        for y in (cy.saturating_sub(r))..(cy.saturating_add(r)) {
            for x in (cx.saturating_sub(r))..(cx.saturating_add(r)) {
                let idx = y * self.width + x;
                if idx < self.v.len() {
                    self.v[idx] = 1.0;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, feed: f32, kill: f32) {
        let width = self.width;
        let height = self.height;
        // Borrow fields separately to avoid borrow checker conflicts
        let u = &self.u;
        let v = &self.v;
        let next_u = &mut self.next_u;
        let next_v = &mut self.next_v;

        // Parallel iteration
        next_u
            .par_iter_mut()
            .zip(next_v.par_iter_mut())
            .enumerate()
            .for_each(|(i, (nu, nv))| {
                let x = i % width;
                let y = i / width;

                let mut lap_u = 0.0;
                let mut lap_v = 0.0;

                // 3x3 Convolution
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                        let neighbor_idx = ny * width + nx;

                        let weight = if dx == 0 && dy == 0 {
                            -1.0
                        } else if dx == 0 || dy == 0 {
                            0.2
                        } else {
                            0.05
                        };

                        lap_u += u[neighbor_idx] * weight;
                        lap_v += v[neighbor_idx] * weight;
                    }
                }

                let cur_u = u[i];
                let cur_v = v[i];
                let uvv = cur_u * cur_v * cur_v;

                // Diffusion rates
                let diff_u = 1.0;
                let diff_v = 0.5;

                let du = diff_u * lap_u - uvv + feed * (1.0 - cur_u);
                let dv = diff_v * lap_v + uvv - (feed + kill) * cur_v;

                *nu = (cur_u + du * dt).clamp(0.0, 1.0);
                *nv = (cur_v + dv * dt).clamp(0.0, 1.0);
            });

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }

    pub fn get_u(&self, x: usize, y: usize) -> f32 {
        self.u[y * self.width + x]
    }

    pub fn get_v(&self, x: usize, y: usize) -> f32 {
        self.v[y * self.width + x]
    }

    pub fn add_chemical(&mut self, x: usize, y: usize, amount_v: f32) {
        let idx = y * self.width + x;
        if idx < self.v.len() {
            self.v[idx] = (self.v[idx] + amount_v).clamp(0.0, 1.0);
        }
    }

    pub fn remove_chemical(&mut self, x: usize, y: usize, amount_v: f32) {
        let idx = y * self.width + x;
        if idx < self.v.len() {
            self.v[idx] = (self.v[idx] - amount_v).clamp(0.0, 1.0);
        }
    }

    pub fn add_chemical_blob(&mut self, cx: usize, cy: usize, radius: f32, amount_v: f32) {
        let r_int = radius.ceil() as isize;
        for dy in -r_int..=r_int {
            for dx in -r_int..=r_int {
                if (dx * dx + dy * dy) as f32 <= radius * radius {
                    let nx = (cx as isize + dx).rem_euclid(self.width as isize) as usize;
                    let ny = (cy as isize + dy).rem_euclid(self.height as isize) as usize;
                    let idx = ny * self.width + nx;
                    if idx < self.v.len() {
                        self.v[idx] = (self.v[idx] + amount_v).clamp(0.0, 1.0);
                    }
                }
            }
        }
    }
}
