use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstellationShape {
    Triangulum, // 3 unique strands in cycle
    Quadra,     // 4 unique strands in cycle
    Pentagram,  // 5 unique strands in cycle
    Loop,       // 1 unique strand (self-recursion)
    Binary,     // 2 unique strands (oscillation)
}

#[derive(Debug, Clone)]
pub struct Constellation {
    pub shape: ConstellationShape,
    pub duration: usize,
    pub strands: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct StarMap {
    pub history: VecDeque<usize>,
    pub active_constellations: Vec<Constellation>,
    pub history_limit: usize,
}

impl Default for StarMap {
    fn default() -> Self {
        Self::new()
    }
}

impl StarMap {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            active_constellations: Vec::new(),
            history_limit: 12,
        }
    }

    /// Records a jump to a new strand and checks for constellations.
    pub fn record_jump(&mut self, to_strand: usize) {
        // Only push if different from last to avoid spamming history with self-jumps?
        // Actually, self-jumps are Loops. We want to detect them.

        self.history.push_back(to_strand);
        if self.history.len() > self.history_limit {
            self.history.pop_front();
        }

        self.detect_cycles();
    }

    fn detect_cycles(&mut self) {
        if self.history.len() < 2 {
            return;
        }

        let current = *self.history.back().unwrap();

        // Look backwards for the same strand
        let mut cycle_start_idx = None;
        // Iterate backwards skipping the very last element (which is current)
        for (i, &strand) in self.history.iter().enumerate().rev().skip(1) {
            if strand == current {
                cycle_start_idx = Some(i);
                break;
            }
        }

        if let Some(start) = cycle_start_idx {
            let end = self.history.len() - 1;

            // Extract unique strands in the cycle
            let mut unique_strands = std::collections::HashSet::new();
            let mut path = Vec::new();

            // Capture from start to end-1
            for i in start..end {
                let s = self.history[i];
                unique_strands.insert(s);
                path.push(s);
            }

            let shape = match unique_strands.len() {
                1 => Some(ConstellationShape::Loop),
                2 => Some(ConstellationShape::Binary),
                3 => Some(ConstellationShape::Triangulum),
                4 => Some(ConstellationShape::Quadra),
                5 => Some(ConstellationShape::Pentagram),
                _ => None,
            };

            if let Some(s) = shape {
                // Add new constellation
                self.active_constellations.push(Constellation {
                    shape: s,
                    duration: 50,
                    strands: path,
                });

                // Cap active constellations to prevent infinite growth
                if self.active_constellations.len() > 8 {
                    self.active_constellations.remove(0);
                }
            }
        }
    }

    /// Ticks the star map, decaying constellations and returning energy gained.
    pub fn tick(&mut self, tick_counter: u64) -> i64 {
        let mut energy_gain = 0;

        for c in &self.active_constellations {
            match c.shape {
                ConstellationShape::Triangulum => {
                    // Efficiency: +1 Energy
                    energy_gain += 1;
                }
                ConstellationShape::Quadra => {
                    // Stability: +2 Energy
                    energy_gain += 2;
                }
                ConstellationShape::Pentagram => {
                    // Power: +3 Energy
                    energy_gain += 3;
                }
                ConstellationShape::Binary => {
                    // Oscillation: Small chance to gain energy
                    if tick_counter % 2 == 0 {
                        energy_gain += 1;
                    }
                }
                ConstellationShape::Loop => {
                    // Recursion: No direct energy
                }
            }
        }

        // Decay
        self.active_constellations.retain_mut(|c| {
            if c.duration > 0 {
                c.duration -= 1;
                true
            } else {
                false
            }
        });

        energy_gain
    }

    /// Consumes all active constellations for a massive bonus.
    pub fn consume_all(&mut self) -> i64 {
        let count = self.active_constellations.len() as i64;
        if count == 0 {
            return 0;
        }

        let mut bonus = 0;
        for c in &self.active_constellations {
            bonus += match c.shape {
                ConstellationShape::Pentagram => 50,
                ConstellationShape::Quadra => 30,
                ConstellationShape::Triangulum => 15,
                ConstellationShape::Binary => 5,
                ConstellationShape::Loop => 2,
            };
        }
        self.active_constellations.clear();
        bonus + (count * 10)
    }
}
