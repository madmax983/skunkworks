use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len == 0.0 {
            *self
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    pub fn reflect(&self, normal: Vec2) -> Self {
        let n = normal.normalize();
        let d = *self;
        let dot = d.dot(n);
        Self {
            x: d.x - 2.0 * dot * n.x,
            y: d.y - 2.0 * dot * n.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub start: Vec2,
    pub pos: Vec2,
    pub dir: Vec2,
    pub path: Vec<Vec2>,
    pub color: Color,
    pub active: bool,
}

impl Ray {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, color: Color) -> Self {
        let pos = Vec2::new(x, y);
        let dir = Vec2::new(dx, dy).normalize();
        Self {
            start: pos,
            pos,
            dir,
            path: vec![pos],
            color,
            active: true,
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<char>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![' '; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            return ' ';
        }
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = c;
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for c in &mut self.cells {
            *c = ' ';
        }
    }
}

pub struct Simulation {
    pub grid: Grid,
    pub rays: Vec<Ray>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            rays: Vec::new(),
        }
    }

    pub fn add_ray(&mut self, ray: Ray) {
        self.rays.push(ray);
    }

    pub fn clear_rays(&mut self) {
        self.rays.clear();
    }

    pub fn step(&mut self) {
        // Clear paths and reset rays to start for real-time interactivity
        // Or do we simulate continuously?
        // For optics, usually we want to see the full path every frame (instant speed of light).
        // So step() effectively recalculates the whole path.

        for ray in &mut self.rays {
            ray.pos = ray.start;
            ray.path.clear();
            ray.path.push(ray.start);
            ray.active = true;

            // Limit iterations to prevent infinite loops in mirrors
            let mut steps = 0;
            let max_steps = 200;
            let mut last_hit_cell = None;

            while ray.active && steps < max_steps {
                steps += 1;

                // March
                let step_size = 0.5; // Half a cell
                let next_pos = Vec2::new(
                    ray.pos.x + ray.dir.x * step_size,
                    ray.pos.y + ray.dir.y * step_size,
                );

                // Check collision at next_pos
                // Actually, let's use floor for cell index
                // But rays are points.
                let ix = next_pos.x.floor() as isize;
                let iy = next_pos.y.floor() as isize;

                // Bounds check
                if ix < 0
                    || iy < 0
                    || ix >= self.grid.width as isize
                    || iy >= self.grid.height as isize
                {
                    ray.active = false;
                    ray.pos = next_pos;
                    ray.path.push(next_pos);
                    break;
                }

                let cell_idx = (iy as usize) * self.grid.width + (ix as usize);
                let cell = self.grid.get(ix as usize, iy as usize);

                if cell != ' ' && Some(cell_idx) != last_hit_cell {
                    // Interaction
                    match cell {
                        '/' => {
                            // Mirror / (bottom-left to top-right)
                            // Normal for UP-reflection from LEFT is (1, 1)
                            ray.dir = ray.dir.reflect(Vec2::new(1.0, 1.0));
                        }
                        '\\' => {
                            // Mirror \ (top-left to bottom-right)
                            // Normal for DOWN-reflection from LEFT is (-1, 1)
                            ray.dir = ray.dir.reflect(Vec2::new(-1.0, 1.0));
                        }
                        '|' => {
                            // Vertical Mirror
                            ray.dir = ray.dir.reflect(Vec2::new(1.0, 0.0));
                        }
                        '-' => {
                            // Horizontal Mirror
                            ray.dir = ray.dir.reflect(Vec2::new(0.0, 1.0));
                        }
                        '#' => {
                            // Absorb
                            ray.active = false;
                        }
                        'x' => {
                            // Splitter? For now just absorb/stop to show hit
                            ray.active = false;
                        }
                        _ => {}
                    }

                    last_hit_cell = Some(cell_idx);
                }

                ray.pos = next_pos;
                ray.path.push(next_pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_reflect() {
        // Reflect moving right (1, 0) against vertical wall (-1, 0) -> (-1, 0)
        let v = Vec2::new(1.0, 0.0);
        let n = Vec2::new(-1.0, 0.0);
        let r = v.reflect(n);
        assert!((r.x - -1.0).abs() < 1e-6);
        assert!((r.y - 0.0).abs() < 1e-6);

        // Reflect moving down (0, 1) against horizontal wall (0, -1) -> (0, -1)
        // Note: Y grows down in screen coordinates
        let v = Vec2::new(0.0, 1.0);
        let n = Vec2::new(0.0, -1.0);
        let r = v.reflect(n);
        assert!((r.x - 0.0).abs() < 1e-6);
        assert!((r.y - -1.0).abs() < 1e-6);
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(10, 10);
        grid.set(5, 5, '#');
        assert_eq!(grid.get(5, 5), '#');
        assert_eq!(grid.get(0, 0), ' ');
        assert_eq!(grid.get(20, 20), ' '); // Out of bounds
    }

    #[test]
    fn test_mirror_reflection_directions() {
        // Test /
        // Ray moving Right (1, 0) hits / -> Should go Up (0, -1)
        let dir = Vec2::new(1.0, 0.0);
        let normal_slash = Vec2::new(1.0, 1.0);
        let r = dir.reflect(normal_slash);
        assert!((r.x - 0.0).abs() < 1e-6);
        assert!((r.y - -1.0).abs() < 1e-6);

        // Test \
        // Ray moving Right (1, 0) hits \ -> Should go Down (0, 1)
        let dir = Vec2::new(1.0, 0.0);
        let normal_backslash = Vec2::new(-1.0, 1.0);
        let r = dir.reflect(normal_backslash);
        assert!((r.x - 0.0).abs() < 1e-6);
        assert!((r.y - 1.0).abs() < 1e-6);
    }
}
