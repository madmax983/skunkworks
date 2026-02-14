pub struct Eye {
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub fovea_width: f32,
    pub fovea_height: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub manual_control: bool,
}

impl Eye {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            x: width / 2.0,
            y: height / 2.0,
            target_x: width / 2.0,
            target_y: height / 2.0,
            fovea_width: 20.0,
            fovea_height: 10.0,
            width,
            height,
            speed: 0.1,
            manual_control: false,
        }
    }

    pub fn update(&mut self, energy_grid: &[f32], grid_width: usize) {
        if !self.manual_control {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut total_energy = 0.0;

            for (i, &energy) in energy_grid.iter().enumerate() {
                if energy > 0.001 {
                    let y = (i / grid_width) as f32;
                    let x = (i % grid_width) as f32;
                    sum_x += x * energy;
                    sum_y += y * energy;
                    total_energy += energy;
                }
            }

            if total_energy > 0.0 {
                self.target_x = sum_x / total_energy;
                self.target_y = sum_y / total_energy;
            }
        }

        // Smooth pursuit
        self.x += (self.target_x - self.x) * self.speed;
        self.y += (self.target_y - self.y) * self.speed;

        // Clamp
        self.x = self.x.max(0.0).min(self.width - 1.0);
        self.y = self.y.max(0.0).min(self.height - 1.0);
    }

    /// Returns true if the point (x, y) is within the fovea
    pub fn in_fovea(&self, x: f32, y: f32) -> bool {
        let half_w = self.fovea_width / 2.0;
        let half_h = self.fovea_height / 2.0;

        x >= self.x - half_w && x <= self.x + half_w && y >= self.y - half_h && y <= self.y + half_h
    }

    pub fn set_target(&mut self, x: f32, y: f32) {
        self.target_x = x;
        self.target_y = y;
        self.manual_control = true;
    }
}
