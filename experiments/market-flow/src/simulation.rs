use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    Empty,
    Bid,               // Green, moves Up
    Ask,               // Red, moves Down
    Trade { age: u8 }, // White flash, decays
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Particle>,
    pub trade_count: usize,
    pub total_bids: usize,
    pub total_asks: usize,
    pub center_of_mass: f32, // Represents "Price"
    // Scratch buffers for update loop
    updated: Vec<bool>,
    scan_x: Vec<usize>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Particle::Empty; width * height],
            trade_count: 0,
            total_bids: 0,
            total_asks: 0,
            center_of_mass: height as f32 / 2.0,
            updated: vec![false; width * height],
            scan_x: (0..width).collect(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::Empty;
        }
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = p;
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let mut trades = 0;
        let mut bids = 0;
        let mut asks = 0;
        let mut weighted_y_sum = 0.0;
        let mut mass_sum = 0.0;

        // Reset scratch buffers
        self.updated.fill(false);
        // Randomize scan direction
        if rng.gen_bool(0.5) {
            self.scan_x.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        } else {
            self.scan_x
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.width - 1 - i);
        }

        // Pass 1: Handle Bids (Up).
        for y in 0..self.height {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }

                if let Particle::Bid = self.cells[idx] {
                    // Try Move Up (y-1)
                    if y > 0 {
                        let target_idx = (y - 1) * self.width + x;
                        match self.cells[target_idx] {
                            Particle::Empty => {
                                self.cells[target_idx] = Particle::Bid;
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                            }
                            Particle::Ask => {
                                // Annihilate!
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                            }
                            _ => {
                                // Blocked. Try sideways?
                                // Randomly left or right.
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx]
                                            && matches!(self.cells[n_idx], Particle::Empty)
                                        {
                                            self.cells[n_idx] = Particle::Bid;
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Pass 2: Handle Asks (Down).
        for y in (0..self.height).rev() {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }

                if let Particle::Ask = self.cells[idx] {
                    // Try Move Down (y+1)
                    if y < self.height - 1 {
                        let target_idx = (y + 1) * self.width + x;
                        match self.cells[target_idx] {
                            Particle::Empty => {
                                self.cells[target_idx] = Particle::Ask;
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                            }
                            Particle::Bid => {
                                // Annihilate!
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                            }
                            _ => {
                                // Blocked. Try sideways.
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx]
                                            && matches!(self.cells[n_idx], Particle::Empty)
                                        {
                                            self.cells[n_idx] = Particle::Ask;
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Pass 3: Decay Trades and Stats
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                match self.cells[idx] {
                    Particle::Trade { age } => {
                        if age > 0 {
                            self.cells[idx] = Particle::Trade { age: age - 1 };
                        } else {
                            self.cells[idx] = Particle::Empty;
                        }
                    }
                    Particle::Bid => {
                        bids += 1;
                        weighted_y_sum += y as f32;
                        mass_sum += 1.0;
                    }
                    Particle::Ask => {
                        asks += 1;
                        weighted_y_sum += y as f32;
                        mass_sum += 1.0;
                    }
                    _ => {}
                }
            }
        }

        self.trade_count = trades;
        self.total_bids = bids;
        self.total_asks = asks;
        if mass_sum > 0.0 {
            self.center_of_mass = weighted_y_sum / mass_sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_moves_up() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 2, Particle::Bid); // Bottom center
        grid.update();
        assert_eq!(grid.get(1, 1), Particle::Bid);
        assert_eq!(grid.get(1, 2), Particle::Empty);
    }

    #[test]
    fn test_ask_moves_down() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 0, Particle::Ask); // Top center
        grid.update();
        assert_eq!(grid.get(1, 1), Particle::Ask);
        assert_eq!(grid.get(1, 0), Particle::Empty);
    }

    #[test]
    fn test_annihilation() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 2, Particle::Bid);
        grid.set(1, 0, Particle::Ask);

        // Tick 1: Both move to center (1, 1) and collide?
        // Depends on update order.
        // Bid (y=2) moves to y=1. Cell (1,1) is now Bid.
        // Ask (y=0) moves to y=1. Cell (1,1) is Bid. Ask sees Bid and annihilates!

        grid.update();
        match grid.get(1, 1) {
            Particle::Trade { .. } => {}
            _ => panic!("Expected Trade at (1,1), found {:?}", grid.get(1, 1)),
        }
        assert_eq!(grid.trade_count, 1);
    }
}
