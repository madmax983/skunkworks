use rand::Rng;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Empty,
    Wall,
    Sand,
    Water,
    Text, // Behaves like Sand but preserves character
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub kind: ParticleKind,
    pub char: char,
    pub color: Color,
}

#[allow(dead_code)]
impl Particle {
    pub fn new(kind: ParticleKind, char: char, color: Color) -> Self {
        Self { kind, char, color }
    }

    pub fn empty() -> Self {
        Self {
            kind: ParticleKind::Empty,
            char: ' ',
            color: Color::Reset,
        }
    }

    pub fn wall() -> Self {
        Self {
            kind: ParticleKind::Wall,
            char: '#',
            color: Color::Gray,
        }
    }

    pub fn sand() -> Self {
        Self {
            kind: ParticleKind::Sand,
            char: 's', // Default, can be overridden
            color: Color::Yellow,
        }
    }

    pub fn water() -> Self {
        Self {
            kind: ParticleKind::Water,
            char: '~',
            color: Color::Blue,
        }
    }
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Particle>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![Particle::empty(); width * height];
        Self {
            width,
            height,
            grid,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width == width && self.height == height {
            return;
        }
        let mut new_grid = vec![Particle::empty(); width * height];

        // Copy over what fits (simple crop/pad)
        for y in 0..std::cmp::min(self.height, height) {
            for x in 0..std::cmp::min(self.width, width) {
                new_grid[y * width + x] = self.grid[y * self.width + x];
            }
        }

        self.width = width;
        self.height = height;
        self.grid = new_grid;
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::wall(); // Treat out of bounds as walls
        }
        self.grid[self.get_index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            self.grid[idx] = p;
        }
    }

    pub fn update(&mut self) {
        // We iterate and modify in place (traditional falling sand often does this,
        // iterating bottom-up to prevent teleportation).
        // However, to get "perfect" simultaneous updates, double buffering is needed.
        // But for "falling sand" aesthetic, bottom-up is standard and faster.

        let mut rng = rand::thread_rng();

        for y in (0..self.height).rev() {
            // Iterate x in random order or alternating order to prevent bias?
            // Simple left-right is fine for MVP, but let's do random direction per row maybe?
            // Or just forward.
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let p = self.grid[idx];

                if p.kind == ParticleKind::Empty || p.kind == ParticleKind::Wall {
                    continue;
                }

                if p.kind == ParticleKind::Sand || p.kind == ParticleKind::Text {
                    self.update_sand(x, y, &mut rng);
                } else if p.kind == ParticleKind::Water {
                    self.update_water(x, y, &mut rng);
                }
            }
        }
    }

    fn update_sand(&mut self, x: usize, y: usize, _rng: &mut impl Rng) {
        if y + 1 >= self.height {
            return; // On floor
        }

        // Try down
        if self.is_empty_or_liquid(x, y + 1) {
            self.swap(x, y, x, y + 1);
        }
        // Try down-left
        else if x > 0 && self.is_empty_or_liquid(x - 1, y + 1) {
            self.swap(x, y, x - 1, y + 1);
        }
        // Try down-right
        else if x + 1 < self.width && self.is_empty_or_liquid(x + 1, y + 1) {
            self.swap(x, y, x + 1, y + 1);
        }
    }

    fn update_water(&mut self, x: usize, y: usize, rng: &mut impl Rng) {
        if y + 1 >= self.height {
            // Spread sideways if on floor
            self.spread_water(x, y, rng);
            return;
        }

        // Try down
        if self.get(x, y + 1).kind == ParticleKind::Empty {
            self.swap(x, y, x, y + 1);
        } else {
            self.spread_water(x, y, rng);
        }
    }

    fn spread_water(&mut self, x: usize, y: usize, rng: &mut impl Rng) {
        let left_empty = x > 0 && self.get(x - 1, y).kind == ParticleKind::Empty;
        let right_empty = x + 1 < self.width && self.get(x + 1, y).kind == ParticleKind::Empty;

        if left_empty && right_empty {
            if rng.gen_bool(0.5) {
                self.swap(x, y, x - 1, y);
            } else {
                self.swap(x, y, x + 1, y);
            }
        } else if left_empty {
            self.swap(x, y, x - 1, y);
        } else if right_empty {
            self.swap(x, y, x + 1, y);
        }
    }

    fn is_empty_or_liquid(&self, x: usize, y: usize) -> bool {
        let p = self.get(x, y);
        p.kind == ParticleKind::Empty || p.kind == ParticleKind::Water
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = self.get_index(x1, y1);
        let idx2 = self.get_index(x2, y2);
        self.grid.swap(idx1, idx2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sand_falls() {
        let mut world = World::new(3, 3);
        world.set(1, 0, Particle::sand()); // Sand at top center

        world.update();

        assert_eq!(world.get(1, 0).kind, ParticleKind::Empty);
        assert_eq!(world.get(1, 1).kind, ParticleKind::Sand);

        world.update();

        assert_eq!(world.get(1, 1).kind, ParticleKind::Empty);
        assert_eq!(world.get(1, 2).kind, ParticleKind::Sand);

        world.update();

        // Should stay at bottom
        assert_eq!(world.get(1, 2).kind, ParticleKind::Sand);
    }

    #[test]
    fn test_sand_piles() {
        let mut world = World::new(3, 3);
        world.set(1, 2, Particle::wall()); // Wall at bottom center
        world.set(1, 1, Particle::sand()); // Sand at center (directly above wall)

        world.update();

        // Can't go down (Wall at 1,2), so should go down-left (0,2) or down-right (2,2)
        // Check that it moved from (1,1)
        assert_eq!(world.get(1, 1).kind, ParticleKind::Empty);
        // Check that it's in one of the diagonals
        let left = world.get(0, 2).kind == ParticleKind::Sand;
        let right = world.get(2, 2).kind == ParticleKind::Sand;
        assert!(left || right);
    }
}
