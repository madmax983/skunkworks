use macroquad::prelude::*;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub heightmap: Vec<f64>,
    pub water_map: Vec<f64>,    // Tracks water accumulation (transient)
    pub sediment_map: Vec<f64>, // Tracks sediment history (where deposition happens)
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            heightmap: vec![0.0; width * height],
            water_map: vec![0.0; width * height],
            sediment_map: vec![0.0; width * height],
        }
    }

    pub fn get_height(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.heightmap[y * self.width + x]
    }

    pub fn set_height(&mut self, x: usize, y: usize, h: f64) {
        if x < self.width && y < self.height {
            self.heightmap[y * self.width + x] = h;
        }
    }

    pub fn uplift(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            self.heightmap[y * self.width + x] += amount;
        }
    }

    // Helper for bilinear interpolation
    fn height_at(&self, x: f64, y: f64) -> f64 {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        let u = x - x_i as f64;
        let v = y - y_i as f64;

        let h00 = self.get_height(x_i, y_i);
        let h10 = self.get_height(x_i + 1, y_i);
        let h01 = self.get_height(x_i, y_i + 1);
        let h11 = self.get_height(x_i + 1, y_i + 1);

        (h00 * (1.0 - u) + h10 * u) * (1.0 - v) + (h01 * (1.0 - u) + h11 * u) * v
    }

    fn gradient_at(&self, x: f64, y: f64) -> (f64, f64) {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        let u = x - x_i as f64;
        let v = y - y_i as f64;

        let h00 = self.get_height(x_i, y_i);
        let h10 = self.get_height(x_i + 1, y_i);
        let h01 = self.get_height(x_i, y_i + 1);
        let h11 = self.get_height(x_i + 1, y_i + 1);

        let dx = (h10 - h00) * (1.0 - v) + (h11 - h01) * v;
        let dy = (h01 - h00) * (1.0 - u) + (h11 - h10) * u;

        (dx, dy)
    }

    pub fn erode_droplet(&mut self, mut x: f64, mut y: f64) {
        let max_steps = 64;
        let inertia = 0.05; // Low inertia = follow gradient closely
        let gravity = 4.0;
        let evaporation = 0.02;
        let capacity_factor = 4.0; // How much sediment can it carry
        let min_slope = 0.01;
        let deposit_speed = 0.3;
        let erode_speed = 0.3;

        let mut speed: f64 = 1.0;
        let mut water: f64 = 1.0;
        let mut sediment: f64 = 0.0;
        let mut dir_x: f64 = 0.0;
        let mut dir_y: f64 = 0.0;

        for _ in 0..max_steps {
            // Check bounds
            if x < 1.0 || x >= (self.width - 2) as f64 || y < 1.0 || y >= (self.height - 2) as f64 {
                break;
            }

            // Track water for visualization
            let ipos = (y.round() as usize) * self.width + (x.round() as usize);
            if ipos < self.water_map.len() {
                self.water_map[ipos] += water * 0.1;
            }

            let (gx, gy) = self.gradient_at(x, y);

            // Update direction
            dir_x = dir_x * inertia - gx * (1.0 - inertia);
            dir_y = dir_y * inertia - gy * (1.0 - inertia);

            // Normalize
            let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
            if len == 0.0 {
                // Flat ground: Deposit all sediment and stop
                self.deposit(x, y, sediment);
                break;
            }
            dir_x /= len;
            dir_y /= len;

            let old_x = x;
            let old_y = y;
            x += dir_x;
            y += dir_y;

            // Check bounds again
            if x < 1.0 || x >= (self.width - 2) as f64 || y < 1.0 || y >= (self.height - 2) as f64 {
                // Hit edge: Deposit sediment at edge? Or let it flow off?
                // Flow off = lose sediment. That's fine for open world.
                break;
            }

            let h_old = self.height_at(old_x, old_y);
            let h_new = self.height_at(x, y);
            let diff = h_new - h_old; // Positive = Uphill, Negative = Downhill

            // Update capacity and velocity
            // If downhill, capacity is proportional to slope (-diff).
            // If uphill, capacity is minimal (min_slope).
            let slope = (-diff).max(min_slope);
            let c = slope * speed * water * capacity_factor;

            if diff > 0.0 {
                // Moving uphill (kinetic energy depleted or pit)
                // Fill pit
                let amount = sediment.min(diff); // Fill up to the height difference
                sediment -= amount;
                self.deposit(old_x, old_y, amount);

                // Stop moving if we hit a wall or pit
                speed = 0.0;

                // Deposit remaining sediment at the foot of the slope/pit
                if sediment > 0.0 {
                    self.deposit(old_x, old_y, sediment);
                }
                break;
            } else {
                // Moving downhill
                // Increase speed based on gravity and slope
                // Ensure the argument to sqrt is non-negative
                let speed_sq = speed * speed + (-diff) * gravity;
                speed = speed_sq.max(0.0).sqrt();

                if sediment > c {
                    // Carrying too much sediment -> Deposit
                    let amount = (sediment - c) * deposit_speed;
                    sediment -= amount;
                    self.deposit(old_x, old_y, amount);
                } else {
                    // Erode
                    let amount = (c - sediment) * erode_speed;
                    // Don't erode more than the height difference (avoid digging pits deeper than the fall)
                    let amount = amount.min(-diff);
                    sediment += amount;
                    self.erode_point(old_x, old_y, amount);
                }
            }

            water *= 1.0 - evaporation;
            if water < 0.01 {
                // Evaporated: deposit remaining sediment
                self.deposit(old_x, old_y, sediment);
                break;
            }
        }
    }

    fn deposit(&mut self, x: f64, y: f64, amount: f64) {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        let u = x - x_i as f64;
        let v = y - y_i as f64;

        // Bilinear deposit
        self.add_height(x_i, y_i, amount * (1.0 - u) * (1.0 - v));
        self.add_height(x_i + 1, y_i, amount * u * (1.0 - v));
        self.add_height(x_i, y_i + 1, amount * (1.0 - u) * v);
        self.add_height(x_i + 1, y_i + 1, amount * u * v);

        // Track sediment
        let idx = y_i * self.width + x_i;
        if idx < self.sediment_map.len() {
            self.sediment_map[idx] += amount;
        }
    }

    fn erode_point(&mut self, x: f64, y: f64, amount: f64) {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        let u = x - x_i as f64;
        let v = y - y_i as f64;

        self.add_height(x_i, y_i, -amount * (1.0 - u) * (1.0 - v));
        self.add_height(x_i + 1, y_i, -amount * u * (1.0 - v));
        self.add_height(x_i, y_i + 1, -amount * (1.0 - u) * v);
        self.add_height(x_i + 1, y_i + 1, -amount * u * v);
    }

    fn add_height(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            self.heightmap[y * self.width + x] += amount;
        }
    }

    pub fn decay_water(&mut self) {
        for w in &mut self.water_map {
            *w *= 0.8; // Fast decay for display
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return BLACK;
        }
        let h = self.heightmap[y * self.width + x];
        let w = self.water_map[y * self.width + x];
        // let _s = self.sediment_map[y * self.width + x];

        // Base terrain color
        let mut color = if h < 5.0 {
            // Lowlands / Sea
            Color::new(0.2, 0.4, 0.2, 1.0)
        } else if h < 20.0 {
            // Hills
            Color::new(0.4, 0.5, 0.3, 1.0)
        } else if h < 50.0 {
            // Mountains
            Color::new(0.5, 0.5, 0.5, 1.0)
        } else {
            // Snow
            Color::new(0.9, 0.9, 0.9, 1.0)
        };

        // Shade by height (Simple AO / Height fog)
        let shade = (h / 100.0).clamp(0.0, 0.5) as f32;
        color.r += shade;
        color.g += shade;
        color.b += shade;

        // Overlay water
        if w > 0.1 {
            let water_alpha = (w * 0.5).clamp(0.0, 0.8) as f32;
            let water_color = Color::new(0.0, 0.5, 1.0, 1.0);

            color.r = color.r * (1.0 - water_alpha) + water_color.r * water_alpha;
            color.g = color.g * (1.0 - water_alpha) + water_color.g * water_alpha;
            color.b = color.b * (1.0 - water_alpha) + water_color.b * water_alpha;
        }

        color
    }
}
