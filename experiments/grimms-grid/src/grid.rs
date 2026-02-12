use crate::phoneme::Phoneme;
use crate::rules::evolve;
use rand::Rng;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Option<Phoneme>>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![None; width]; height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Phoneme> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None; // Wall
        }
        self.cells[y as usize][x as usize]
    }

    pub fn update(&mut self) {
        let mut next_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.get_neighbors(x as i32, y as i32);
                let center = self.cells[y][x];
                next_cells[y][x] = evolve(center, &neighbors);
            }
        }
        self.cells = next_cells;
    }

    fn get_neighbors(&self, x: i32, y: i32) -> Vec<Option<Phoneme>> {
        // Order: NW, N, NE, W, E, SW, S, SE
        let coords = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        coords.iter().map(|&(nx, ny)| self.get(nx, ny)).collect()
    }

    pub fn seed_random(&mut self) {
        let mut rng = rand::thread_rng();
        let all_phonemes = Phoneme::all();

        for y in 0..self.height {
            for x in 0..self.width {
                if rng.gen_bool(0.3) {
                    let p = all_phonemes[rng.gen_range(0..all_phonemes.len())];
                    self.cells[y][x] = Some(p);
                } else {
                    self.cells[y][x] = None;
                }
            }
        }
    }

    pub fn seed_text(&mut self, text: &str) {
        // Fill the grid with the text, repeating
        let mut chars = text.chars().cycle();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = chars.next() {
                    if let Ok(p) = Phoneme::try_from(c) {
                        self.cells[y][x] = Some(p);
                    } else {
                        self.cells[y][x] = None; // Space or unknown
                    }
                }
            }
        }
    }
}
