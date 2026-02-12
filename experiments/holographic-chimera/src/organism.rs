use font8x8::{UnicodeFonts, BASIC_FONTS};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Consume,
    Divide,
    Move,
    Photosynthesize,
    None,
}

impl OpCode {
    pub fn to_char(&self) -> char {
        match self {
            OpCode::Consume => 'C',
            OpCode::Divide => 'D',
            OpCode::Move => 'M',
            OpCode::Photosynthesize => 'P',
            OpCode::None => ' ',
        }
    }
}

#[derive(Clone)]
pub struct Organism {
    pub x: usize,
    pub y: usize,
    pub energy: f64,
    pub channel_idx: usize, // Which frequency channel (angle) it is tuned to
    pub age: usize,
    pub halted: bool,
}

impl Organism {
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x,
            y,
            energy: 50.0,
            channel_idx: rng.gen_range(0..5), // Random channel
            age: 0,
            halted: false,
        }
    }

    /// Scans the local environment (reconstructed image) for OpCodes.
    /// The `image` is expected to be the magnitude map of the channel the organism is tuned to.
    /// Returns the detected OpCode if any.
    pub fn scan(&self, image: &[f64], width: usize, height: usize) -> OpCode {
        // Extract 8x8 window centered at x, y
        // Handle wrap-around
        let mut window = [0.0; 64];

        for dy in 0..8 {
            for dx in 0..8 {
                let sy = (self.y + dy).rem_euclid(height);
                let sx = (self.x + dx).rem_euclid(width);
                window[dy * 8 + dx] = image[sy * width + sx];
            }
        }

        // Compare with ideal bitmaps
        let candidates = [
            OpCode::Consume,
            OpCode::Divide,
            OpCode::Move,
            OpCode::Photosynthesize,
        ];

        let mut best_op = OpCode::None;
        let mut best_score = 0.0;
        let threshold = 0.6; // Tune this

        for op in candidates {
            let score = self.correlate(&window, op);
            if score > best_score {
                best_score = score;
                best_op = op;
            }
        }

        if best_score > threshold {
            best_op
        } else {
            OpCode::None
        }
    }

    fn correlate(&self, window: &[f64; 64], op: OpCode) -> f64 {
        let char_code = op.to_char();
        if let Some(glyph) = BASIC_FONTS.get(char_code) {
            let mut score = 0.0;
            let mut max_score = 0.0;

            for (y, &row) in glyph.iter().enumerate() {
                for x in 0..8 {
                    let pixel_on = ((row >> x) & 1) == 1;
                    let val = window[y * 8 + x];

                    if pixel_on {
                        score += val;
                        max_score += 1.0; // Ideal max value is 1.0 (assuming normalized image)
                    } else {
                        // Penalty for noise in empty space
                        score -= val * 0.5;
                    }
                }
            }

            if max_score > 0.0 {
                score / max_score
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn step(&mut self, width: usize, height: usize, op: OpCode) -> Option<Organism> {
        if self.halted {
            return None;
        }

        self.age += 1;
        self.energy -= 0.1; // Metabolic cost

        let mut offspring = None;

        match op {
            OpCode::Consume => {
                self.energy += 5.0; // Assume food is abundant where detected
            }
            OpCode::Photosynthesize => {
                self.energy += 1.0;
            }
            OpCode::Divide => {
                if self.energy > 60.0 {
                    self.energy /= 2.0;
                    let mut child = self.clone();
                    child.x = (self.x + 1) % width;
                    child.y = (self.y + 1) % height;
                    child.age = 0;
                    // Mutation: Change channel
                    if rand::random::<f32>() < 0.1 {
                        child.channel_idx = rand::thread_rng().gen_range(0..5);
                    }
                    offspring = Some(child);
                }
            }
            OpCode::Move => {
                let mut rng = rand::thread_rng();
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                // Wrap around
                self.x = (self.x as isize + dx).rem_euclid(width as isize) as usize;
                self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
                self.energy -= 0.2;
            }
            OpCode::None => {
                // Random walk if no instruction
                let mut rng = rand::thread_rng();
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                self.x = (self.x as isize + dx).rem_euclid(width as isize) as usize;
                self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
            }
        }

        if self.energy <= 0.0 {
            self.halted = true;
        }

        offspring
    }
}
