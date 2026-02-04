use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    Empty,
    Bid(usize),        // Green, moves Up, Owner ID
    Ask(usize),        // Red, moves Down, Owner ID
    Trade { age: u8 }, // White flash, decays
}

pub struct TradeEvent {
    pub buyer: usize,
    pub seller: usize,
    pub price: f32, // y-coordinate as price
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Particle>,
    pub trade_count: usize,
    pub total_bids: usize,
    pub total_asks: usize,
    pub center_of_mass: f32,
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

    pub fn update(&mut self) -> Vec<TradeEvent> {
        let mut rng = rand::thread_rng();
        let mut trade_events = Vec::new();
        let mut trades = 0;
        let mut bids = 0;
        let mut asks = 0;
        let mut weighted_y_sum = 0.0;
        let mut mass_sum = 0.0;

        self.updated.fill(false);
        if rng.gen_bool(0.5) {
            self.scan_x.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        } else {
            self.scan_x
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.width - 1 - i);
        }

        // Pass 1: Bids (Up)
        for y in 0..self.height {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] { continue; }

                if let Particle::Bid(owner) = self.cells[idx] {
                    if y > 0 {
                        let target_idx = (y - 1) * self.width + x;
                        match self.cells[target_idx] {
                            Particle::Empty => {
                                self.cells[target_idx] = Particle::Bid(owner);
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                            }
                            Particle::Ask(seller) => {
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                                trade_events.push(TradeEvent {
                                    buyer: owner,
                                    seller,
                                    price: (self.height - 1 - (y - 1)) as f32, // Invert Y for price
                                });
                            }
                            _ => {
                                // Sideways logic
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx] && matches!(self.cells[n_idx], Particle::Empty) {
                                            self.cells[n_idx] = Particle::Bid(owner);
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Reached top, remove? Or stay? Let's remove for now (expired order)
                         self.cells[idx] = Particle::Empty;
                    }
                }
            }
        }

        // Pass 2: Asks (Down)
        for y in (0..self.height).rev() {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] { continue; }

                if let Particle::Ask(owner) = self.cells[idx] {
                    if y < self.height - 1 {
                        let target_idx = (y + 1) * self.width + x;
                        match self.cells[target_idx] {
                            Particle::Empty => {
                                self.cells[target_idx] = Particle::Ask(owner);
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                            }
                            Particle::Bid(buyer) => {
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                                trade_events.push(TradeEvent {
                                    buyer,
                                    seller: owner,
                                    price: (self.height - 1 - (y + 1)) as f32,
                                });
                            }
                            _ => {
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx] && matches!(self.cells[n_idx], Particle::Empty) {
                                            self.cells[n_idx] = Particle::Ask(owner);
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Reached bottom, expire
                        self.cells[idx] = Particle::Empty;
                    }
                }
            }
        }

        // Pass 3: Stats
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
                    Particle::Bid(_) => {
                        bids += 1;
                        weighted_y_sum += (self.height - y) as f32; // Height - y gives "height from bottom"
                        mass_sum += 1.0;
                    }
                    Particle::Ask(_) => {
                        asks += 1;
                        weighted_y_sum += (self.height - y) as f32;
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

        trade_events
    }
}
