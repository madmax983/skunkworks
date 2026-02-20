use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
    Soil,
    HardSoil,
    Rock,
    Water, // Goal
    Seed,  // Start
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize, cell_type: CellType) -> Self {
        Self { cell_type, x, y }
    }

    pub fn cost(&self) -> f32 {
        match self.cell_type {
            CellType::Soil => 1.0,
            CellType::HardSoil => 5.0,
            CellType::Rock => f32::INFINITY,
            CellType::Water => 1.0,
            CellType::Seed => 1.0,
        }
    }

    pub fn color(&self) -> Color {
        match self.cell_type {
            CellType::Soil => BROWN,
            CellType::HardSoil => Color::new(0.4, 0.2, 0.1, 1.0), // Darker brown
            CellType::Rock => GRAY,
            CellType::Water => BLUE,
            CellType::Seed => GREEN,
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub start: Option<(usize, usize)>,
    pub goal: Option<(usize, usize)>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y, CellType::Soil));
            }
        }
        Self {
            width,
            height,
            cells,
            start: None,
            goal: None,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn set_type(&mut self, x: usize, y: usize, cell_type: CellType) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx].cell_type = cell_type;

            if cell_type == CellType::Seed {
                self.start = Some((x, y));
            }
            if cell_type == CellType::Water {
                self.goal = Some((x, y));
            }
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 4-way connectivity

        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                n.push((nx as usize, ny as usize));
            }
        }
        n
    }

    pub fn draw(&self, cell_size: f32, offset_x: f32, offset_y: f32) {
        for cell in &self.cells {
            draw_rectangle(
                cell.x as f32 * cell_size + offset_x,
                cell.y as f32 * cell_size + offset_y,
                cell_size,
                cell_size,
                cell.color(),
            );
            // Draw grid lines for better visibility
            draw_rectangle_lines(
                cell.x as f32 * cell_size + offset_x,
                cell.y as f32 * cell_size + offset_y,
                cell_size,
                cell_size,
                1.0,
                BLACK,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.cells.len(), 100);
        assert_eq!(grid.get(0, 0).unwrap().cell_type, CellType::Soil);
    }

    #[test]
    fn test_set_type() {
        let mut grid = Grid::new(5, 5);
        grid.set_type(2, 2, CellType::Rock);
        assert_eq!(grid.get(2, 2).unwrap().cell_type, CellType::Rock);
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::new(5, 5);
        let n = grid.neighbors(0, 0);
        assert_eq!(n.len(), 2); // Corner
        assert!(n.contains(&(0, 1)));
        assert!(n.contains(&(1, 0)));

        let n = grid.neighbors(2, 2);
        assert_eq!(n.len(), 4); // Center
    }
}
