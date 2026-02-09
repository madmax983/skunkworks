use macroquad::rand::{gen_range, srand};

pub struct VisualField {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<f32>,
    pub warp_x: Vec<f32>,
    pub warp_y: Vec<f32>,
    pub time: f32,
}

impl VisualField {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        srand(0); // Seed RNG
        Self {
            width,
            height,
            buffer: vec![0.0; size],
            warp_x: vec![0.0; size],
            warp_y: vec![0.0; size],
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        let w = self.width as i32;
        let h = self.height as i32;

        // 1. Generate Base Pattern (Drifting Grating + Moving Blob)
        let phase = self.time * 2.0;
        let blob_x = (self.time.sin() * 0.5 + 0.5) * self.width as f32;
        let blob_y = (self.time.cos() * 0.5 + 0.5) * self.height as f32;

        let temp_buffer = (0..self.height)
            .flat_map(|y| {
                (0..self.width).map(move |x| {
                    // Background Grating
                    let grating = (x as f32 * 0.1 + phase).sin() * 0.5 + 0.5;

                    // Moving Blob
                    let dx = x as f32 - blob_x;
                    let dy = y as f32 - blob_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let blob = (-dist * 0.1).exp();

                    (grating * 0.3 + blob * 0.7).clamp(0.0, 1.0)
                })
            })
            .collect::<Vec<f32>>();

        // 2. Apply Warp & Decay
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                // Decay warp
                self.warp_x[idx] *= 0.95;
                self.warp_y[idx] *= 0.95;

                // Apply warp
                let wx = self.warp_x[idx];
                let wy = self.warp_y[idx];

                let src_x = (x as f32 + wx).clamp(0.0, (w - 1) as f32);
                let src_y = (y as f32 + wy).clamp(0.0, (h - 1) as f32);

                // Bilinear sample or nearest neighbor?
                // Nearest neighbor for glitch aesthetic
                let sx = src_x.round() as usize;
                let sy = src_y.round() as usize;

                if sx < self.width && sy < self.height {
                    self.buffer[idx] = temp_buffer[sy * self.width + sx];
                } else {
                    self.buffer[idx] = 0.0;
                }
            }
        }
    }

    pub fn inject_feedback(&mut self, spikes: &[(usize, usize)]) {
        let strength = 2.0;
        let range = 3; // Influence radius

        for &(sx, sy) in spikes {
            // Apply a random push at the spike location
            let push_x = gen_range(-1.0, 1.0) * strength;
            let push_y = gen_range(-1.0, 1.0) * strength;

            // Spread the warp
            for dy in -range..=range {
                for dx in -range..=range {
                    let nx = sx as i32 + dx;
                    let ny = sy as i32 + dy;

                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        let idx = (ny * self.width as i32 + nx) as usize;
                        // Distance falloff
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        let falloff = (1.0 - dist / range as f32).max(0.0);

                        self.warp_x[idx] += push_x * falloff;
                        self.warp_y[idx] += push_y * falloff;
                    }
                }
            }
        }
    }
}
