#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

use rand::Rng;

impl Cell {
    #[allow(clippy::match_like_matches_macro)]
    pub fn beats(&self, other: &Cell) -> bool {
        match (self, other) {
            (Cell::Rock, Cell::Scissors) | (Cell::Rock, Cell::Lizard) => true,
            (Cell::Paper, Cell::Rock) | (Cell::Paper, Cell::Spock) => true,
            (Cell::Scissors, Cell::Paper) | (Cell::Scissors, Cell::Lizard) => true,
            (Cell::Lizard, Cell::Spock) | (Cell::Lizard, Cell::Paper) => true,
            (Cell::Spock, Cell::Scissors) | (Cell::Spock, Cell::Rock) => true,
            _ => false,
        }
    }

    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..6) {
            0 => Cell::Empty,
            1 => Cell::Rock,
            2 => Cell::Paper,
            3 => Cell::Scissors,
            4 => Cell::Lizard,
            _ => Cell::Spock,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..width * height).map(|_| Cell::random()).collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn update(&mut self) {
        let mut next_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let current = self.cells[idx];
                let mut counts = [0; 6]; // Empty, Rock, Paper, Scissors, Lizard, Spock

                // Count neighbors
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                        let n_idx = self.get_index(nx, ny);
                        let neighbor = self.cells[n_idx];

                        match neighbor {
                            Cell::Empty => counts[0] += 1,
                            Cell::Rock => counts[1] += 1,
                            Cell::Paper => counts[2] += 1,
                            Cell::Scissors => counts[3] += 1,
                            Cell::Lizard => counts[4] += 1,
                            Cell::Spock => counts[5] += 1,
                        }
                    }
                }

                if current == Cell::Empty {
                    // Reproduction: If exactly 3 neighbors of a species, become it.
                    // If multiple, pick the one with max count (or random? Logic needs to be deterministic preferably)
                    // Let's iterate and find species with exactly 3 (standard GoL is 3).
                    // Or maybe > 2?
                    // Let's try: Find species with max count. If max count == 3, become it.
                    let mut max_count = 0;
                    let mut max_species = Cell::Empty;

                    for (i, &count) in counts.iter().enumerate().skip(1) {
                        // Skip Empty
                        if count > max_count {
                            max_count = count;
                            max_species = match i {
                                1 => Cell::Rock,
                                2 => Cell::Paper,
                                3 => Cell::Scissors,
                                4 => Cell::Lizard,
                                5 => Cell::Spock,
                                _ => Cell::Empty,
                            };
                        } else if count == max_count {
                            // Tie breaker? No birth if tie?
                            max_species = Cell::Empty;
                        }
                    }

                    if max_count == 3 && max_species != Cell::Empty {
                        next_cells[idx] = max_species;
                    }
                } else {
                    // Predation: If >= 3 neighbors beat me, I change.
                    let mut max_threat_count = 0;
                    let mut max_threat = Cell::Empty;

                    for (i, &count) in counts.iter().enumerate().skip(1) {
                        let species = match i {
                            1 => Cell::Rock,
                            2 => Cell::Paper,
                            3 => Cell::Scissors,
                            4 => Cell::Lizard,
                            5 => Cell::Spock,
                            _ => Cell::Empty,
                        };

                        if species.beats(&current) {
                            if count > max_threat_count {
                                max_threat_count = count;
                                max_threat = species;
                            } else if count == max_threat_count {
                                // Tie? Pick one? Or stay same?
                                // If multiple predators attacking with same force, maybe just pick the last one or stay
                                // Let's simplify: if any predator has >= 3, succumb to the strongest threat.
                            }
                        }
                    }

                    if max_threat_count >= 3 {
                        next_cells[idx] = max_threat;
                    }
                }
            }
        }
        self.cells = next_cells;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Cell::*;

    #[test]
    fn test_rock_rules() {
        assert!(Rock.beats(&Scissors));
        assert!(Rock.beats(&Lizard));
        assert!(!Rock.beats(&Paper));
        assert!(!Rock.beats(&Spock));
        assert!(!Rock.beats(&Rock));
    }

    #[test]
    fn test_paper_rules() {
        assert!(Paper.beats(&Rock));
        assert!(Paper.beats(&Spock));
        assert!(!Paper.beats(&Scissors));
        assert!(!Paper.beats(&Lizard));
        assert!(!Paper.beats(&Paper));
    }

    #[test]
    fn test_scissors_rules() {
        assert!(Scissors.beats(&Paper));
        assert!(Scissors.beats(&Lizard));
        assert!(!Scissors.beats(&Rock));
        assert!(!Scissors.beats(&Spock));
    }

    #[test]
    fn test_lizard_rules() {
        assert!(Lizard.beats(&Spock));
        assert!(Lizard.beats(&Paper));
        assert!(!Lizard.beats(&Rock));
        assert!(!Lizard.beats(&Scissors));
    }

    #[test]
    fn test_spock_rules() {
        assert!(Spock.beats(&Scissors));
        assert!(Spock.beats(&Rock));
        assert!(!Spock.beats(&Paper));
        assert!(!Spock.beats(&Lizard));
    }

    #[test]
    fn test_grid_update_predation() {
        // Create a 3x3 grid
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![Cell::Empty; 9],
        };

        // Center is Rock
        grid.cells[4] = Cell::Rock;

        // Surround with 3 Papers (Top-Left, Top, Top-Right)
        grid.cells[0] = Cell::Paper;
        grid.cells[1] = Cell::Paper;
        grid.cells[2] = Cell::Paper;

        grid.update();

        // Center should become Paper (Predation threshold 3 met)
        assert_eq!(grid.cells[4], Cell::Paper);
    }

    #[test]
    fn test_grid_update_survival() {
        // Create a 3x3 grid
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![Cell::Empty; 9],
        };

        // Center is Rock
        grid.cells[4] = Cell::Rock;

        // Surround with 2 Papers (Top-Left, Top)
        grid.cells[0] = Cell::Paper;
        grid.cells[1] = Cell::Paper;

        grid.update();

        // Center should stay Rock (Predation threshold 3 NOT met)
        assert_eq!(grid.cells[4], Cell::Rock);
    }

    #[test]
    fn test_grid_update_reproduction() {
        // Create a 3x3 grid
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![Cell::Empty; 9],
        };

        // Center is Empty
        grid.cells[4] = Cell::Empty;

        // Surround with 3 Spocks
        grid.cells[0] = Cell::Spock;
        grid.cells[1] = Cell::Spock;
        grid.cells[2] = Cell::Spock;

        grid.update();

        // Center should become Spock
        assert_eq!(grid.cells[4], Cell::Spock);
    }
}
