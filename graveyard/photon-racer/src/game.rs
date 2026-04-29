use crate::physics::{self, MirrorType};
use locus::Vec2; // Fixed import

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Block,           // '#'
    MirrorSlash,     // '/'
    MirrorBackslash, // '\'
    Target,          // '@'
    Source,          // '*'
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Empty; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::Empty;
        }
    }
}

pub struct Photon {
    pub position: Vec2,
    pub velocity: Vec2,
    pub active: bool,
    pub trail: Vec<Vec2>,
    pub won: bool,
}

impl Photon {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(vx, vy),
            active: true,
            trail: vec![Vec2::new(x, y)],
            won: false,
        }
    }

    pub fn update(&mut self, grid: &Grid) {
        if !self.active {
            return;
        }

        // 1. Move
        let new_pos = self.position + self.velocity;

        // 2. Check Bounds
        let ix = new_pos.x.round() as i32;
        let iy = new_pos.y.round() as i32;

        if ix < 0 || iy < 0 || ix >= grid.width as i32 || iy >= grid.height as i32 {
            self.active = false;
            // Record last position even if out of bounds for visual trail end
            self.trail.push(new_pos);
            return;
        }

        // 3. Check Cell interaction
        // Safe unwrap because we checked bounds
        let cell = grid.get(ix as usize, iy as usize).unwrap();

        match cell {
            Cell::Block => {
                self.active = false;
            }
            Cell::Target => {
                self.won = true;
                self.active = false;
            }
            Cell::MirrorSlash => {
                self.velocity = physics::reflect(self.velocity, MirrorType::Slash);
            }
            Cell::MirrorBackslash => {
                self.velocity = physics::reflect(self.velocity, MirrorType::Backslash);
            }
            Cell::Source | Cell::Empty => {
                // Pass through
            }
        }

        self.position = new_pos;
        self.trail.push(new_pos);
    }
}

pub struct GameState {
    pub grid: Grid,
    pub photon: Option<Photon>,
    pub cursor: (usize, usize),
    pub mode: GameMode,
    pub message: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameMode {
    Edit,
    Run,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height);
        // Default setup: Source at (1, height/2), Target at (width-2, height/2)
        grid.set(1, height / 2, Cell::Source);
        grid.set(width - 2, height / 2, Cell::Target);

        Self {
            grid,
            photon: None,
            cursor: (width / 2, height / 2),
            mode: GameMode::Edit,
            message: "Welcome! Arrows to move, Space to Fire, / \\ # s t to place.".to_string(),
        }
    }

    pub fn fire(&mut self) {
        // Find source
        let mut source_pos = None;
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if let Some(Cell::Source) = self.grid.get(x, y) {
                    source_pos = Some((x, y));
                    break;
                }
            }
        }

        if let Some((sx, sy)) = source_pos {
            self.photon = Some(Photon::new(sx as f64, sy as f64, 1.0, 0.0)); // Fire Right
            self.mode = GameMode::Run;
            self.message = "Photon Fired!".to_string();
        } else {
            self.message = "No Source! Place 's'.".to_string();
        }
    }

    #[allow(clippy::collapsible_if)]
    pub fn update(&mut self) {
        if self.mode != GameMode::Run {
            return;
        }
        if let Some(ref mut photon) = self.photon {
            if photon.active {
                photon.update(&self.grid);
                if photon.won {
                    self.message = "TARGET HIT! YOU WIN! (Press 'r' to reset)".to_string();
                } else if !photon.active {
                    self.message = "Photon lost. (Press 'r' to retry)".to_string();
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.photon = None;
        self.mode = GameMode::Edit;
        self.message = "Reset. Edit Mode.".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photon_move() {
        let grid = Grid::new(10, 10);
        let mut photon = Photon::new(0.0, 0.0, 1.0, 0.0);

        photon.update(&grid);
        assert_eq!(photon.position, Vec2::new(1.0, 0.0));
        assert!(photon.active);
    }

    #[test]
    fn test_photon_hit_block() {
        let mut grid = Grid::new(10, 10);
        grid.set(2, 0, Cell::Block);
        let mut photon = Photon::new(0.0, 0.0, 1.0, 0.0);

        photon.update(&grid); // moves to (1,0)
        assert!(photon.active);

        photon.update(&grid); // moves to (2,0) which is Block
        assert!(!photon.active);
        assert_eq!(photon.position, Vec2::new(2.0, 0.0));
    }

    #[test]
    fn test_photon_reflect() {
        let mut grid = Grid::new(10, 10);
        grid.set(2, 0, Cell::MirrorSlash); // '/' at (2,0)
        let mut photon = Photon::new(0.0, 0.0, 1.0, 0.0);

        photon.update(&grid); // (1,0)
        photon.update(&grid); // (2,0) -> Hits '/' -> Vel becomes (0, -1) [Up]

        assert_eq!(photon.position, Vec2::new(2.0, 0.0));
        assert_eq!(photon.velocity, Vec2::new(0.0, -1.0));

        photon.update(&grid); // (2, -1) -> Out of bounds (since y < 0)
        assert!(!photon.active);
    }
}
