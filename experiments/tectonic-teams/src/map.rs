pub struct HilbertCurve {
    n: usize, // Side length (power of 2)
}

impl HilbertCurve {
    pub fn new(n: usize) -> Self {
        Self { n }
    }

    // Convert 1D distance d to (x, y) coordinates
    pub fn d2xy(&self, mut d: usize) -> (usize, usize) {
        let mut x = 0;
        let mut y = 0;
        let mut s = 1;
        while s < self.n {
            let rx = 1 & (d / 2);
            let ry = 1 & (d ^ rx);
            self.rot(s, &mut x, &mut y, rx, ry);
            x += s * rx;
            y += s * ry;
            d /= 4;
            s *= 2;
        }
        (x, y)
    }

    fn rot(&self, n: usize, x: &mut usize, y: &mut usize, rx: usize, ry: usize) {
        if ry == 0 {
            if rx == 1 {
                *x = n - 1 - *x;
                *y = n - 1 - *y;
            }
            // Swap x and y
            let t = *x;
            *x = *y;
            *y = t;
        }
    }
}

pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    pub heightmap: Vec<f32>,
    pub owner: Vec<u8>,     // 0 = No Owner
    pub strength: Vec<f32>, // 0.0 to 1.0
    pub file_indices: Vec<Option<usize>>, // Index into file list, if any
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            heightmap: vec![0.0; size],
            owner: vec![0; size],
            strength: vec![0.0; size],
            file_indices: vec![None; size],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<(&mut f32, &mut u8, &mut f32)> {
        if let Some(idx) = self.get_index(x, y) {
            Some((
                &mut self.heightmap[idx],
                &mut self.owner[idx],
                &mut self.strength[idx],
            ))
        } else {
            None
        }
    }
}
