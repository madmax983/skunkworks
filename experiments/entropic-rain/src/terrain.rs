pub struct Terrain {
    pub width: usize,
    pub heights: Vec<f64>,
}

impl Terrain {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            heights: vec![0.0; width],
        }
    }

    pub fn uplift(&mut self, x: usize, amount: f64) {
        if x < self.width {
            self.heights[x] += amount;
        }
    }

    pub fn erode(&mut self, x: usize, amount: f64) {
        if x < self.width {
            self.heights[x] = (self.heights[x] - amount).max(0.0);
        }
    }

    pub fn get_height(&self, x: usize) -> f64 {
        if x < self.width {
            self.heights[x]
        } else {
            0.0
        }
    }
}
