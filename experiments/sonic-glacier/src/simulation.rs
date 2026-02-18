pub struct HeapTerrain {
    pub width: usize,
    pub height: usize,
    pub bedrock: Vec<f32>,
    pub ice: Vec<f32>,
    pub water: Vec<f32>,
}

impl HeapTerrain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bedrock: vec![10.0; width * height],
            ice: vec![5.0; width * height], // Start with some ice
            water: vec![0.0; width * height],
        }
    }

    /// Allocations generate heat, melting ice into water.
    pub fn apply_heat(&mut self, x: usize, y: usize, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        // Heat melts ice
        let melt = amount.min(self.ice[idx]);
        self.ice[idx] -= melt;
        self.water[idx] += melt;

        // Excessive heat can even erode bedrock slightly (thermal shock)
        if amount > self.ice[idx] + 5.0 {
            self.bedrock[idx] -= 0.05;
        }
    }

    /// Audio frequencies generate cold, freezing water into ice.
    pub fn apply_cold(&mut self, x: usize, y: usize, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        // Cold freezes water
        let freeze = amount.min(self.water[idx]);
        self.water[idx] -= freeze;
        self.ice[idx] += freeze;

        // Extreme cold can deposit new ice (crystallization from air humidity)
        if amount > 1.0 {
            self.ice[idx] += amount * 0.1;
        }
    }

    pub fn tick(&mut self) {
        let mut new_water = self.water.clone();
        let mut new_bedrock = self.bedrock.clone();
        // Ice is mostly static but can slip? For now let's keep ice static but allow water to flow.

        let width = self.width;
        let height = self.height;

        // Simple Hydraulic Erosion Simulation
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                // Effective height for flow includes ice and bedrock
                let h = self.bedrock[idx] + self.ice[idx] + self.water[idx];

                if self.water[idx] <= 0.001 {
                    continue;
                }

                // Check neighbors for flow
                let mut lower_neighbors = Vec::new();
                let mut total_diff = 0.0;

                // 4-neighbor check
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < width && ny < height {
                        let n_idx = ny * width + nx;
                        let n_h = self.bedrock[n_idx] + self.ice[n_idx] + self.water[n_idx];
                        if n_h < h {
                            let diff = h - n_h;
                            lower_neighbors.push((n_idx, diff));
                            total_diff += diff;
                        }
                    }
                }

                // Distribute water
                if !lower_neighbors.is_empty() {
                    let flow_amount = self.water[idx] * 0.5; // Flow 50% of water per tick

                    for (n_idx, diff) in lower_neighbors {
                        let share = flow_amount * (diff / total_diff);
                        new_water[idx] -= share;
                        new_water[n_idx] += share;

                        // Erosion: Fast moving water takes bedrock
                        // Velocity proxy = diff
                        let erosion = share * diff * 0.05;
                        if new_bedrock[idx] > erosion {
                            new_bedrock[idx] -= erosion;
                            // Deposition logic could go here, but let's stick to erosion for now
                            // to avoid complexity
                        }
                    }
                }
            }
        }

        // Global effects
        // Evaporation? Or maybe just freezing.
        // Let's say ambient temperature is slightly below freezing, so water slowly freezes back to ice
        // unless there is heat input.
        /*
        for idx in 0..self.width * self.height {
            let freeze_rate = 0.01;
            if new_water[idx] > 0.0 {
                let amount = new_water[idx].min(freeze_rate);
                new_water[idx] -= amount;
                self.ice[idx] += amount;
            }
        }
        */

        self.water = new_water;
        self.bedrock = new_bedrock;
    }

    pub fn get_bedrock(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.bedrock[y * self.width + x]
    }

    pub fn get_ice(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.ice[y * self.width + x]
    }

    pub fn get_water(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.water[y * self.width + x]
    }
}
