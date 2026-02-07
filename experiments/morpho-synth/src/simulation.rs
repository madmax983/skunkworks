use rayon::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Sensor {
    pub x: usize,
    pub y: usize,
    pub freq: f32,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub u_next: Vec<f32>,
    pub v_next: Vec<f32>,
    pub sensors: Vec<Sensor>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let u = vec![1.0; size];
        let v = vec![0.0; size];
        let u_next = vec![1.0; size];
        let v_next = vec![0.0; size];

        let mut grid = Self {
            width,
            height,
            u,
            v,
            u_next,
            v_next,
            sensors: Vec::new(),
        };
        grid.init_standard();
        grid
    }

    pub fn init_standard(&mut self) {
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        let radius = 10;
        let mut rng = rand::thread_rng();

        // Add some noise to break symmetry
        for i in 0..self.u.len() {
            self.u[i] = 1.0 - rng.gen::<f32>() * 0.05;
        }

        // Add V seed
        let y_start = center_y.saturating_sub(radius);
        let y_end = (center_y + radius).min(self.height);
        let x_start = center_x.saturating_sub(radius);
        let x_end = (center_x + radius).min(self.width);

        for y in y_start..y_end {
            for x in x_start..x_end {
                let idx = y * self.width + x;
                self.v[idx] = 0.5 + rng.gen::<f32>() * 0.1;
            }
        }
    }

    pub fn add_sensor(&mut self, x: usize, y: usize, freq: f32) {
        if x < self.width && y < self.height {
            self.sensors.push(Sensor { x, y, freq });
        }
    }

    pub fn get_active_frequencies(&self) -> Vec<f32> {
        let mut active = Vec::new();
        for sensor in &self.sensors {
             let idx = sensor.y * self.width + sensor.x;
             if idx < self.v.len() {
                 // Threshold for activation
                 if self.v[idx] > 0.2 {
                     active.push(sensor.freq);
                 }
             }
        }
        active
    }

    pub fn update(&mut self, feed: f32, kill: f32, dt: f32) {
        let width = self.width;
        let height = self.height;

        let u = &self.u;
        let v = &self.v;

        // Diffusion rates
        let du = 1.0;
        let dv = 0.5;

        self.u_next.par_iter_mut().zip(self.v_next.par_iter_mut()).enumerate().for_each(|(i, (u_next, v_next))| {
            let x = i % width;
            let y = i / width;

            // Indices with wrapping
            let left = if x == 0 { width - 1 } else { x - 1 };
            let right = if x == width - 1 { 0 } else { x + 1 };
            let top = if y == 0 { height - 1 } else { y - 1 };
            let bottom = if y == height - 1 { 0 } else { y + 1 };

            let idx = i;
            let idx_l = y * width + left;
            let idx_r = y * width + right;
            let idx_t = top * width + x;
            let idx_b = bottom * width + x;

            // 9-point stencil diagonals
            let idx_tl = top * width + left;
            let idx_tr = top * width + right;
            let idx_bl = bottom * width + left;
            let idx_br = bottom * width + right;

            let u_curr = u[idx];
            let v_curr = v[idx];

            // Laplacian (0.2 for adjacent, 0.05 for diagonal, -1.0 for center)
            let lap_u = (u[idx_l] + u[idx_r] + u[idx_t] + u[idx_b]) * 0.2 +
                        (u[idx_tl] + u[idx_tr] + u[idx_bl] + u[idx_br]) * 0.05 -
                        u_curr;

            let lap_v = (v[idx_l] + v[idx_r] + v[idx_t] + v[idx_b]) * 0.2 +
                        (v[idx_tl] + v[idx_tr] + v[idx_bl] + v[idx_br]) * 0.05 -
                        v_curr;

            let reaction = u_curr * v_curr * v_curr;

            *u_next = u_curr + (du * lap_u - reaction + feed * (1.0 - u_curr)) * dt;
            *v_next = v_curr + (dv * lap_v + reaction - (feed + kill) * v_curr) * dt;

            // Clamp to stay stable
            *u_next = u_next.clamp(0.0, 1.0);
            *v_next = v_next.clamp(0.0, 1.0);
        });

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.u_next);
        std::mem::swap(&mut self.v, &mut self.v_next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.u.len(), 100);
        assert_eq!(grid.v.len(), 100);
    }

    #[test]
    fn test_diffusion() {
        let mut grid = Grid::new(10, 10);
        let u_start = grid.u[0];
        grid.update(0.0, 0.0, 0.1);
        assert!(grid.u[0] < u_start);
    }
}
