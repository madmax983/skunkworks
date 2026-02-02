#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize, initial_val: f32) -> Self {
        Self {
            width,
            height,
            cells: vec![initial_val; width * height],
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.cells[y * self.width + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, val: f32) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = val;
        }
    }
}

pub struct ShallowWater {
    pub width: usize,
    pub height: usize,
    pub h: Grid, // Water depth
    pub u: Grid, // Velocity X
    pub v: Grid, // Velocity Y
    pub b: Grid, // Terrain height

    // Buffers for next step
    h_next: Grid,
    u_next: Grid,
    v_next: Grid,

    pub g: f32,
    pub damping: f32,
    pub dt: f32,
}

impl ShallowWater {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            h: Grid::new(width, height, 0.0),
            u: Grid::new(width, height, 0.0),
            v: Grid::new(width, height, 0.0),
            b: Grid::new(width, height, 0.0),
            h_next: Grid::new(width, height, 0.0),
            u_next: Grid::new(width, height, 0.0),
            v_next: Grid::new(width, height, 0.0),
            g: 9.81,
            damping: 0.995,
            dt: 0.02,
        }
    }

    pub fn update(&mut self) {
        let w = self.width;
        let h = self.height;
        let dt = self.dt;
        let dx = 1.0;

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let h_c = self.h.get(x, y);
                let u_c = self.u.get(x, y);
                let v_c = self.v.get(x, y);
                // let b_c = self.b.get(x, y);

                // Central differences
                let h_px = self.h.get(x + 1, y);
                let h_mx = self.h.get(x - 1, y);
                let h_py = self.h.get(x, y + 1);
                let h_my = self.h.get(x, y - 1);

                let b_px = self.b.get(x + 1, y);
                let b_mx = self.b.get(x - 1, y);
                let b_py = self.b.get(x, y + 1);
                let b_my = self.b.get(x, y - 1);

                let u_px = self.u.get(x + 1, y);
                let u_mx = self.u.get(x - 1, y);
                let u_py = self.u.get(x, y + 1);
                let u_my = self.u.get(x, y - 1);

                let v_px = self.v.get(x + 1, y);
                let v_mx = self.v.get(x - 1, y);
                let v_py = self.v.get(x, y + 1);
                let v_my = self.v.get(x, y - 1);

                // Gradients
                let dh_dx = (h_px - h_mx) / (2.0 * dx);
                let dh_dy = (h_py - h_my) / (2.0 * dx);

                let db_dx = (b_px - b_mx) / (2.0 * dx);
                let db_dy = (b_py - b_my) / (2.0 * dx);

                let du_dx = (u_px - u_mx) / (2.0 * dx);
                let du_dy = (u_py - u_my) / (2.0 * dx);

                let dv_dx = (v_px - v_mx) / (2.0 * dx);
                let dv_dy = (v_py - v_my) / (2.0 * dx);

                // Mass conservation
                let dhu_dx = h_c * du_dx + u_c * dh_dx;
                let dhv_dy = h_c * dv_dy + v_c * dh_dy;

                let dh = -(dhu_dx + dhv_dy) * dt;

                // Momentum
                let du = -(u_c * du_dx + v_c * du_dy + self.g * (dh_dx + db_dx)) * dt;
                let dv = -(u_c * dv_dx + v_c * dv_dy + self.g * (dh_dy + db_dy)) * dt;

                let mut new_h = h_c + dh;
                if new_h < 0.0 {
                    new_h = 0.0;
                }

                self.h_next.set(x, y, new_h);
                self.u_next.set(x, y, (u_c + du) * self.damping);
                self.v_next.set(x, y, (v_c + dv) * self.damping);
            }
        }

        // Boundaries are reflective (value = 0 or value = neighbor? 0 for now as it's initialized to 0)

        std::mem::swap(&mut self.h, &mut self.h_next);
        std::mem::swap(&mut self.u, &mut self.u_next);
        std::mem::swap(&mut self.v, &mut self.v_next);
    }
}
