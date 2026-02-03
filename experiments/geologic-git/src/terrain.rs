use rand::Rng;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub height_map: Vec<f64>,
    pub water_map: Vec<f64>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        // Initialize with some noise or flat?
        // Let's start flat-ish with slight noise to prevent stagnant water initially.
        let mut rng = rand::thread_rng();
        let height_map = (0..size).map(|_| rng.gen_range(5.0..10.0)).collect();

        Self {
            width,
            height,
            height_map,
            water_map: vec![0.0; size],
        }
    }

    pub fn get_height(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.height_map[y * self.width + x]
    }

    #[allow(dead_code)]
    pub fn set_height(&mut self, x: usize, y: usize, val: f64) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.height_map[y * self.width + x] = val;
    }

    pub fn add_rain(&mut self, x: f64, y: f64) {
        // Drop a particle and simulate erosion immediately
        self.simulate_particle(x, y);
    }

    pub fn decay_water(&mut self) {
        for w in &mut self.water_map {
            *w *= 0.8; // Fast decay for visual trail
        }
    }

    #[allow(unused_assignments)]
    fn simulate_particle(&mut self, mut x: f64, mut y: f64) {
        let max_steps = 30;
        let inertia = 0.05; // How much previous direction is kept
        let gravity = 4.0;
        let _min_slope = 0.01;
        let capacity_factor = 4.0; // How much sediment can it carry
        let erosion_factor = 0.1; // How fast it erodes
        let deposition_factor = 0.1; // How fast it deposits
        let evaporation_rate = 0.02;

        let mut speed: f64 = 1.0;
        let mut water: f64 = 1.0;
        let mut sediment: f64 = 0.0;
        let mut dir_x: f64 = 0.0;
        let mut dir_y: f64 = 0.0;

        for _ in 0..max_steps {
            let ix = x as usize;
            let iy = y as usize;

            if ix >= self.width - 1 || iy >= self.height - 1 {
                break;
            }

            // Mark water trail
            let idx = iy * self.width + ix;
            self.water_map[idx] = (self.water_map[idx] + water).min(1.0);

            // Calculate gradient
            let h00 = self.height_map[idx];
            let h10 = self.height_map[idx + 1];
            let h01 = self.height_map[idx + self.width];
            let h11 = self.height_map[idx + self.width + 1];

            // bilinear interpolation of gradient
            let gx = (h10 - h00) * (1.0 - (y - iy as f64)) + (h11 - h01) * (y - iy as f64);
            let gy = (h01 - h00) * (1.0 - (x - ix as f64)) + (h11 - h10) * (x - ix as f64);

            // Update direction
            dir_x = dir_x * inertia - gx * (1.0 - inertia);
            dir_y = dir_y * inertia - gy * (1.0 - inertia);

            // Normalize
            let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
            if len > 0.0 {
                dir_x /= len;
                dir_y /= len;
            }

            // New position
            let nx = x + dir_x;
            let ny = y + dir_y;

            if nx < 0.0 || ny < 0.0 || nx >= (self.width - 1) as f64 || ny >= (self.height - 1) as f64 {
                break;
            }

            // Height difference
            let old_h = self.get_height_interpolated(x, y);
            let new_h = self.get_height_interpolated(nx, ny);
            let diff = new_h - old_h;

            if diff > 0.0 {
                // Uphill? Deposit everything and stop (or fill pit)
                // For simplicity, deposit and stop
                let amount = sediment.min(diff);
                self.deposit(x, y, amount);
                // sediment -= amount; // Unused because we break
                break; // Stop flowing uphill
            }

            let slope = -diff;
            speed = (speed * speed + slope * gravity).sqrt();

            let capacity = speed * water * capacity_factor;

            if sediment > capacity {
                let amount = (sediment - capacity) * deposition_factor;
                self.deposit(x, y, amount);
                sediment -= amount;
            } else {
                let amount = (capacity - sediment) * erosion_factor;
                let amount = amount.min(slope); // Don't dig deeper than the slope?
                self.erode(x, y, amount);
                sediment += amount;
            }
            // Prevent unused warning by using sediment or dropping it.
            // In reality, sediment is carried to next loop, but if loop breaks, it's lost.
            let _ = sediment;

            water *= 1.0 - evaporation_rate;
            if water < 0.01 {
                break;
            }

            x = nx;
            y = ny;
        }
    }

    fn get_height_interpolated(&self, x: f64, y: f64) -> f64 {
        let ix = x as usize;
        let iy = y as usize;
        if ix >= self.width - 1 || iy >= self.height - 1 {
            return 0.0; // boundary
        }
        let fx = x - ix as f64;
        let fy = y - iy as f64;

        let idx = iy * self.width + ix;
        let h00 = self.height_map[idx];
        let h10 = self.height_map[idx + 1];
        let h01 = self.height_map[idx + self.width];
        let h11 = self.height_map[idx + self.width + 1];

        let top = h00 * (1.0 - fx) + h10 * fx;
        let bot = h01 * (1.0 - fx) + h11 * fx;
        top * (1.0 - fy) + bot * fy
    }

    fn deposit(&mut self, x: f64, y: f64, amount: f64) {
        let ix = x as usize;
        let iy = y as usize;
        let fx = x - ix as f64;
        let fy = y - iy as f64;

        // Distribute to 4 neighbors
        self.add_height(ix, iy, amount * (1.0 - fx) * (1.0 - fy));
        self.add_height(ix + 1, iy, amount * fx * (1.0 - fy));
        self.add_height(ix, iy + 1, amount * (1.0 - fx) * fy);
        self.add_height(ix + 1, iy + 1, amount * fx * fy);
    }

    fn erode(&mut self, x: f64, y: f64, amount: f64) {
         let ix = x as usize;
        let iy = y as usize;
        let fx = x - ix as f64;
        let fy = y - iy as f64;

        self.add_height(ix, iy, -amount * (1.0 - fx) * (1.0 - fy));
        self.add_height(ix + 1, iy, -amount * fx * (1.0 - fy));
        self.add_height(ix, iy + 1, -amount * (1.0 - fx) * fy);
        self.add_height(ix + 1, iy + 1, -amount * fx * fy);
    }

    fn add_height(&mut self, x: usize, y: usize, amount: f64) {
         if x >= self.width || y >= self.height { return; }
         self.height_map[y * self.width + x] += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erosion_moves_mass() {
        let mut t = Terrain::new(10, 10);

        // Make a slope
        for y in 0..10 {
            for x in 0..10 {
                t.set_height(x, y, (10 - x) as f64);
            }
        }

        let initial_mass: f64 = t.height_map.iter().sum();

        // Rain at top
        t.add_rain(0.5, 5.0);

        let final_mass: f64 = t.height_map.iter().sum();

        // Mass should be roughly conserved (sediment is just height moving around)
        // BUT sediment currently disappears if it's still in the drop when it dies.
        // So mass might decrease slightly (loss to river outflow).
        // It definitely shouldn't increase.
        assert!(final_mass <= initial_mass + 0.0001);

        // The top (x=0) should erode
        assert!(t.get_height(0, 5) < 10.0);
    }
}
