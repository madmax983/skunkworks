use glam::Vec2;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
    pub next_cells: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0.0; width * height],
            next_cells: vec![0.0; width * height],
        }
    }

    pub fn deposit(&mut self, pos: Vec2, amount: f32) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] = (self.cells[idx] + amount).min(1.0);
        }
    }

    pub fn diffuse_and_decay(&mut self, decay_rate: f32) {
        let w = self.width;
        let h = self.height;

        // Simple 3x3 Mean Filter
        for y in 0..h {
            for x in 0..w {
                let mut sum = 0.0;
                let mut count = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as isize + dx ;
                        let ny = y as isize + dy ;

                        // Periodic boundaries
                        let nx = nx.rem_euclid(w as isize) as usize;
                        let ny = ny.rem_euclid(h as isize) as usize;

                        sum += self.cells[ny * w + nx];
                        count += 1.0;
                    }
                }

                let avg = sum / count;
                self.next_cells[y * w + x] = avg * decay_rate;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            0.0
        }
    }
}

pub struct HiddenLayer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
}

impl HiddenLayer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    pub fn is_active(&self, x: usize, y: usize) -> bool {
         if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            false
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }
}
