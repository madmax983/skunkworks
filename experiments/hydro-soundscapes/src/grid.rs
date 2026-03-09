#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Water,
    Rock,
    #[allow(dead_code)]
    Vent(u8), // u8 is the lock ID
    #[allow(dead_code)]
    Chimney,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self { cell_type }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for _x in 0..width {
                // Bottom row is rock
                let cell_type = if y == height - 1 {
                    CellType::Rock
                } else {
                    CellType::Water
                };
                cells.push(Cell::new(cell_type));
            }
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y * self.width + x])
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.cells[y * self.width + x])
        }
    }

    #[allow(dead_code)]
    pub fn set_type(&mut self, x: usize, y: usize, cell_type: CellType) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.cell_type = cell_type;
        }
    }
}
