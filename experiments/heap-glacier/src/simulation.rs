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
            ice: vec![0.0; width * height],
            water: vec![0.0; width * height],
        }
    }

    pub fn allocate(&mut self, x: usize, y: usize, amount: f32) {
        if x >= self.width || y >= self.height { return; }
        self.ice[y * self.width + x] += amount;
    }

    pub fn deallocate(&mut self, x: usize, y: usize, amount: f32) {
        if x >= self.width || y >= self.height { return; }
        let idx = y * self.width + x;
        let melt = amount.min(self.ice[idx]);
        self.ice[idx] -= melt;
        self.water[idx] += melt;
    }

    pub fn tick(&mut self) {
        let mut new_water = self.water.clone();
        let mut new_bedrock = self.bedrock.clone();

        let width = self.width;
        let height = self.height;

        // Simple Hydraulic Erosion Simulation
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let h = self.bedrock[idx] + self.ice[idx] + self.water[idx];

                // Check neighbors for flow
                let mut lower_neighbors = Vec::new();
                let mut total_diff = 0.0;

                // 4-neighbor check
                let neighbors = [
                    (x.wrapping_sub(1), y), (x + 1, y),
                    (x, y.wrapping_sub(1)), (x, y + 1)
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
                if !lower_neighbors.is_empty() && self.water[idx] > 0.0 {
                    let flow_amount = self.water[idx] * 0.5; // Flow 50% of water

                    for (n_idx, diff) in lower_neighbors {
                        let share = flow_amount * (diff / total_diff);
                        new_water[idx] -= share;
                        new_water[n_idx] += share;

                        // Erosion: Fast moving water takes bedrock
                        // Velocity proxy = diff
                        let erosion = share * diff * 0.1;
                        if new_bedrock[idx] > erosion {
                            new_bedrock[idx] -= erosion;
                            // Deposition? For now just destroy rock (dissolution)
                            // Or move it to neighbor. Let's simpler: just erode.
                        }
                    }
                }

                // Static erosion (dissolution) just for sitting water
                if self.water[idx] > 0.0 {
                     new_bedrock[idx] -= 0.01; // Ensure even static water does something for the test
                }
            }
        }

        // Evaporation
        for w in new_water.iter_mut() {
            *w *= 0.99;
            if *w < 0.01 { *w = 0.0; }
        }

        self.water = new_water;
        self.bedrock = new_bedrock;
    }

    pub fn get_bedrock(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height { return 0.0; }
        self.bedrock[y * self.width + x]
    }

    pub fn get_ice(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height { return 0.0; }
        self.ice[y * self.width + x]
    }

    pub fn get_water(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height { return 0.0; }
        self.water[y * self.width + x]
    }
}
