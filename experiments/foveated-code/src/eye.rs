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
            speed: 0.15,
        }
    }

    pub fn update(&mut self, spikes: &[bool], width: usize) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0.0;

        // Simple center of mass calculation
        for (i, &spiked) in spikes.iter().enumerate() {
            if spiked {
                let y = (i / width) as f32;
                let x = (i % width) as f32;
                sum_x += x;
                sum_y += y;
                count += 1.0;
            }
        }

        // If enough activity, update target
        if count > 3.0 {
            self.target_x = sum_x / count;
            self.target_y = sum_y / count;
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

        x >= self.x - half_w && x <= self.x + half_w &&
        y >= self.y - half_h && y <= self.y + half_h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_movement() {
        let w = 100.0;
        let h = 100.0;
        let mut eye = Eye::new(w, h);

        // Initial center
        assert_eq!(eye.x, 50.0);

        // Simulate spikes at (90, 50)
        let width_usize = 100;
        let mut spikes = vec![false; 100 * 100];
        // Set a block of spikes around (90, 50)
        let target_idx = 50 * 100 + 90;
        spikes[target_idx] = true;
        spikes[target_idx+1] = true;
        spikes[target_idx-1] = true;
        spikes[target_idx+100] = true; // row below

        // Update multiple times to drift towards target
        for _ in 0..20 {
            eye.update(&spikes, width_usize);
        }

        assert!(eye.x > 60.0, "Eye should move towards spike cluster at x=90");
    }
}
