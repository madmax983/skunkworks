use rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    Empty,
    Bid(usize),
    Ask(usize),
    Trade { age: u8 },
}

pub struct Grid4D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub hypersize: usize,
    pub cells: Vec<Particle>,
    pub updated: Vec<bool>,

    // Stats
    pub trade_count: usize,
    pub active_bids: usize,
    pub active_asks: usize,
}

impl Grid4D {
    pub fn new(size: usize) -> Self {
        let total_cells = size * size * size * size;
        Self {
            width: size,
            height: size,
            depth: size,
            hypersize: size,
            cells: vec![Particle::Empty; total_cells],
            updated: vec![false; total_cells],
            trade_count: 0,
            active_bids: 0,
            active_asks: 0,
        }
    }

    pub fn idx(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        w * (self.depth * self.height * self.width)
            + z * (self.height * self.width)
            + y * self.width
            + x
    }

    // Reverse lookup for debug/rendering if needed, but we usually iterate linearly
    #[allow(dead_code)]
    pub fn coords(&self, idx: usize) -> (usize, usize, usize, usize) {
        let w = idx / (self.depth * self.height * self.width);
        let rem_w = idx % (self.depth * self.height * self.width);
        let z = rem_w / (self.height * self.width);
        let rem_z = rem_w % (self.height * self.width);
        let y = rem_z / self.width;
        let x = rem_z % self.width;
        (x, y, z, w)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, w: usize, p: Particle) {
        let i = self.idx(x, y, z, w);
        if i < self.cells.len() {
            self.cells[i] = p;
        }
    }

    pub fn update(&mut self, volatility: f32) {
        let mut rng = thread_rng();
        self.updated.fill(false);
        self.trade_count = 0;
        self.active_bids = 0;
        self.active_asks = 0;

        // Collect indices to iterate in random order?
        // For performance, linear scan is better, but might introduce bias.
        // market-sim randomizes column scan.
        // Here we have X, Z, W as "columns".
        // Let's just iterate linearly for now, and rely on sideways jitter to break symmetries.
        // Or we can iterate Bids Up (Y 0..H) and Asks Down (Y H-1..0).

        // Pass 1: Bids (Move Up: Y -> Y-1)
        // Iterate Y from 0 to Height-1.
        for w in 0..self.hypersize {
            for z in 0..self.depth {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let i = self.idx(x, y, z, w);
                        if self.updated[i] {
                            continue;
                        }

                        if let Particle::Bid(id) = self.cells[i] {
                            self.active_bids += 1;
                            self.process_bid(x, y, z, w, i, id, volatility, &mut rng);
                        }
                    }
                }
            }
        }

        // Pass 2: Asks (Move Down: Y -> Y+1)
        // Iterate Y from Height-1 down to 0.
        for w in 0..self.hypersize {
            for z in 0..self.depth {
                for y in (0..self.height).rev() {
                    for x in 0..self.width {
                        let i = self.idx(x, y, z, w);
                        if self.updated[i] {
                            continue;
                        }

                        if let Particle::Ask(id) = self.cells[i] {
                            self.active_asks += 1;
                            self.process_ask(x, y, z, w, i, id, volatility, &mut rng);
                        }
                    }
                }
            }
        }

        // Pass 3: Decay Trades
        for i in 0..self.cells.len() {
            if let Particle::Trade { age } = self.cells[i] {
                if age > 0 {
                    self.cells[i] = Particle::Trade { age: age - 1 };
                } else {
                    self.cells[i] = Particle::Empty;
                }
            }
        }
    }

    fn process_bid(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        idx: usize,
        id: usize,
        volatility: f32,
        rng: &mut impl Rng,
    ) {
        // Goal: Move to Y-1
        if y > 0 {
            // Check direct path
            let target_y = y - 1;
            let target_idx = self.idx(x, target_y, z, w);

            match self.cells[target_idx] {
                Particle::Empty => {
                    // Move
                    self.move_particle(idx, target_idx, Particle::Bid(id));
                }
                Particle::Ask(_) => {
                    // Trade!
                    self.execute_trade(idx, target_idx);
                }
                _ => {
                    // Blocked. Try sideways or random jump based on volatility.
                    self.try_move_sideways_or_jump(
                        x,
                        y,
                        z,
                        w,
                        idx,
                        Particle::Bid(id),
                        volatility,
                        rng,
                    );
                }
            }
        } else {
            // Reached top (High Price). Expire? Or stay?
            // market-sim expires them.
            self.cells[idx] = Particle::Empty;
            self.active_bids -= 1;
        }
    }

    fn process_ask(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        idx: usize,
        id: usize,
        volatility: f32,
        rng: &mut impl Rng,
    ) {
        // Goal: Move to Y+1
        if y < self.height - 1 {
            let target_y = y + 1;
            let target_idx = self.idx(x, target_y, z, w);

            match self.cells[target_idx] {
                Particle::Empty => {
                    self.move_particle(idx, target_idx, Particle::Ask(id));
                }
                Particle::Bid(_) => {
                    // Trade!
                    self.execute_trade(idx, target_idx);
                }
                _ => {
                    // Blocked.
                    self.try_move_sideways_or_jump(
                        x,
                        y,
                        z,
                        w,
                        idx,
                        Particle::Ask(id),
                        volatility,
                        rng,
                    );
                }
            }
        } else {
            // Reached bottom. Expire.
            self.cells[idx] = Particle::Empty;
            self.active_asks -= 1;
        }
    }

    fn move_particle(&mut self, from: usize, to: usize, p: Particle) {
        self.cells[to] = p;
        self.cells[from] = Particle::Empty;
        self.updated[to] = true;
    }

    fn execute_trade(&mut self, from: usize, to: usize) {
        // Both particles are consumed.
        // 'to' becomes a Trade.
        // 'from' becomes Empty.
        self.cells[to] = Particle::Trade { age: 10 }; // Lasts 10 frames
        self.cells[from] = Particle::Empty;
        self.updated[to] = true;
        self.trade_count += 1;
    }

    fn try_move_sideways_or_jump(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        idx: usize,
        p: Particle,
        volatility: f32,
        rng: &mut impl Rng,
    ) {
        // Sideways: Change X, Z, or W.
        // 6 neighbors in 3 dimensions (X+-1, Z+-1, W+-1).

        // High volatility allows jumping further?
        // Or simply higher probability of sideways movement.

        if rng.gen::<f32>() < 0.1 + volatility {
            // Attempt to move to a random neighbor in X, Z, W
            let mut neighbors = Vec::with_capacity(6);

            if x > 0 {
                neighbors.push((x - 1, y, z, w));
            }
            if x < self.width - 1 {
                neighbors.push((x + 1, y, z, w));
            }

            if z > 0 {
                neighbors.push((x, y, z - 1, w));
            }
            if z < self.depth - 1 {
                neighbors.push((x, y, z + 1, w));
            }

            if w > 0 {
                neighbors.push((x, y, z, w - 1));
            }
            if w < self.hypersize - 1 {
                neighbors.push((x, y, z, w + 1));
            }

            if let Some(&(nx, ny, nz, nw)) = neighbors.get(rng.gen_range(0..neighbors.len())) {
                let n_idx = self.idx(nx, ny, nz, nw);
                if !self.updated[n_idx] && matches!(self.cells[n_idx], Particle::Empty) {
                    self.move_particle(idx, n_idx, p);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_movement() {
        let mut grid = Grid4D::new(4);
        // Bid at (0, 1, 0, 0)
        grid.set(0, 1, 0, 0, Particle::Bid(1));

        // Update with 0 volatility
        grid.update(0.0);

        // Should move UP (Y-1) -> (0, 0, 0, 0)
        // Wait, Y=0 is top. Bid at 1 moves to 0.
        // Check 0,0,0,0
        match grid.cells[grid.idx(0, 0, 0, 0)] {
            Particle::Bid(id) => assert_eq!(id, 1),
            _ => panic!("Bid did not move up"),
        }

        // Old spot empty
        assert_eq!(grid.cells[grid.idx(0, 1, 0, 0)], Particle::Empty);
    }

    #[test]
    fn test_ask_movement() {
        let mut grid = Grid4D::new(4);
        // Ask at (0, 0, 0, 0)
        grid.set(0, 0, 0, 0, Particle::Ask(2));

        // Update
        grid.update(0.0);

        // Should move DOWN (Y+1) -> (0, 1, 0, 0)
        match grid.cells[grid.idx(0, 1, 0, 0)] {
            Particle::Ask(id) => assert_eq!(id, 2),
            _ => panic!("Ask did not move down"),
        }
    }

    #[test]
    fn test_collision() {
        let mut grid = Grid4D::new(4);
        // Bid at (0, 1, 0, 0) -> Moves to 0
        grid.set(0, 1, 0, 0, Particle::Bid(1));
        // Ask at (0, 0, 0, 0) -> Moves to 1?
        // Wait, if Bid moves first (Pass 1), it moves to 0.
        // At 0 there is Ask. Collision!
        // 0 becomes Trade. Bid consumed.

        grid.set(0, 0, 0, 0, Particle::Ask(2));

        grid.update(0.0);

        // (0, 0, 0, 0) should be Trade
        match grid.cells[grid.idx(0, 0, 0, 0)] {
            Particle::Trade { .. } => {}
            _ => panic!("Expected Trade at 0,0,0,0"),
        }

        // (0, 1, 0, 0) should be Empty
        assert_eq!(grid.cells[grid.idx(0, 1, 0, 0)], Particle::Empty);

        assert_eq!(grid.trade_count, 1);
    }
}
