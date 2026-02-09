use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Knot {
    Simple,      // 1 (in positions > units)
    Long(u8),    // 2-9 (in units)
    FigureEight, // 1 (in units)
}

impl Knot {
    pub fn value(&self) -> u8 {
        match self {
            Knot::Simple => 1,
            Knot::Long(v) => *v,
            Knot::FigureEight => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cord {
    // Clusters of knots. Index 0 = Highest power of 10 stored?
    // Quipus read top-down. Top = High value.
    // Let's store as Vec of Clusters.
    // cluster[0] = highest power (e.g. 1000s)
    // ...
    // cluster[last] = units
    pub clusters: Vec<Vec<Knot>>,
}

impl Cord {
    pub fn new() -> Self {
        Self { clusters: Vec::new() }
    }

    /// Reads the integer value of the cord
    pub fn read(&self) -> i64 {
        let mut total: i64 = 0;
        for cluster in &self.clusters {
            let mut cluster_val = 0;
            for knot in cluster {
                cluster_val += knot.value() as i64;
            }
            total = total * 10 + cluster_val;
        }
        total
    }

    /// Appends a new value as a sequence of clusters (knots)
    /// In traditional Quipu, one cord holds one number (or a sum).
    /// Here, `tie` will APPEND a number to the cord?
    /// Or should it set the value?
    /// "Knot Memory": Maybe it appends. A cord is a history of values?
    /// Or a cord represents a single large number.
    ///
    /// Let's make `tie` ADD a new number to the cord as a distinct section?
    /// No, standard Quipu cords usually hold one value (or sum of subsidiaries).
    /// Let's say `tie` replaces the cord content with the new value for now,
    /// or appends to a "sequence of values" if we want a list.
    ///
    /// "Quipu Memory System": Let's treat each Cord as a stack/list of numbers?
    /// No, let's treat each Cord as a single mutable integer register, represented topologically.
    /// `tie(x)` -> Writes x to cord (overwriting? or adding?).
    /// Let's say it adds to the existing value?
    /// `knot(x)` -> Adds x to current value.
    /// `unknot()` -> Clears?
    ///
    /// Better: `tie` sets the value. `untie` clears it.
    pub fn tie(&mut self, val: i64) {
        let val = val.abs(); // Ignore sign for now
        let s = val.to_string();
        let mut new_clusters = Vec::new();

        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as u8;
            let is_units = i == s.len() - 1;
            let mut cluster = Vec::new();

            if is_units {
                if digit == 1 {
                    cluster.push(Knot::FigureEight);
                } else if digit > 1 {
                    cluster.push(Knot::Long(digit));
                }
                // 0 is empty space
            } else {
                // Tens, Hundreds, etc. use Simple knots
                for _ in 0..digit {
                    cluster.push(Knot::Simple);
                }
            }
            new_clusters.push(cluster);
        }
        self.clusters = new_clusters;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuipuState {
    pub cords: Vec<Cord>,
    pub active_cord: usize,
}

impl QuipuState {
    pub fn new() -> Self {
        // 16 Cords by default
        let cords = vec![Cord::new(); 16];
        Self {
            cords,
            active_cord: 0,
        }
    }

    pub fn tie(&mut self, val: i64) {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord].tie(val);
        }
    }

    pub fn read(&self) -> i64 {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord].read()
        } else {
            0
        }
    }

    pub fn untie(&mut self) -> i64 {
        if self.active_cord < self.cords.len() {
            let val = self.cords[self.active_cord].read();
            self.cords[self.active_cord] = Cord::new();
            val
        } else {
            0
        }
    }

    pub fn select_cord(&mut self, idx: usize) {
        if idx < self.cords.len() {
            self.active_cord = idx;
        }
    }

    pub fn tangle(&mut self, other_idx: usize) {
        if self.active_cord < self.cords.len() && other_idx < self.cords.len() {
            let val_a = self.cords[self.active_cord].read();
            let val_b = self.cords[other_idx].read();
            // Entanglement adds values? Or XOR?
            // Let's add them.
            self.cords[self.active_cord].tie(val_a.wrapping_add(val_b));
        }
    }
}
