use crate::sexagesimal::Sexagesimal;

#[derive(Clone, Debug)]
pub struct TabletCell {
    pub value: u8,
    pub hardness: f32,
    pub age: u64,
    pub author: Option<u64>,
}

impl TabletCell {
    pub fn new() -> Self {
        Self {
            value: 0,
            hardness: 0.0,
            age: 0,
            author: None,
        }
    }

    pub fn is_immutable(&self) -> bool {
        self.hardness >= 1.0
    }
}

pub struct Tablet {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<TabletCell>,
}

impl Tablet {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![TabletCell::new(); width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&TabletCell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y * self.width + x])
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut TabletCell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.cells[y * self.width + x])
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8, author: u64) -> bool {
        if let Some(cell) = self.get_mut(x, y) {
            if cell.is_immutable() {
                return false;
            }
            cell.value = value % 60; // Base 60 constraint
            cell.hardness = 0.1; // Fresh clay is soft
            cell.age = 0;
            cell.author = Some(author);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self) {
        for cell in &mut self.cells {
            if cell.value != 0 {
                cell.age += 1;
                if cell.hardness < 1.0 {
                    cell.hardness += 0.005; // Hardens over ~200 ticks
                    if cell.hardness > 1.0 {
                        cell.hardness = 1.0;
                    }
                }
            }
        }
    }

    pub fn flood(&mut self) -> usize {
        let mut washed_away = 0;
        for cell in &mut self.cells {
            if cell.hardness < 0.8 {
                if cell.value != 0 {
                    washed_away += 1;
                }
                *cell = TabletCell::new(); // Wash away
            }
        }
        washed_away
    }
}
