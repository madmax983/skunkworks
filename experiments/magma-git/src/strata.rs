use ratatui::style::Color;

#[derive(Clone, Debug, Default)]
pub struct StrataCell {
    pub char: char,
    pub color: Color,
    pub commit_hash: String,
    pub temperature: f32, // Retain heat for a while?
}

pub struct StrataGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<StrataCell>>,
}

impl StrataGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&StrataCell> {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].as_ref()
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: StrataCell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = Some(cell);
        }
    }

    pub fn solidify(&mut self, x: usize, y: usize, hash: String, temp: f32) {
        let color = if temp > 0.5 { Color::Red } else { Color::DarkGray };
        let char = '█';

        self.set(x, y, StrataCell {
            char,
            color,
            commit_hash: hash,
            temperature: temp,
        });
    }

    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_some()
    }
}
