use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Column {
    // True if Heaven bead (value 5) is ACTIVE (moved down to the bar).
    pub heaven_active: bool,
    // Number of Earth beads (value 1) ACTIVE (moved up to the bar). 0..=4.
    pub earth_active: u8,
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl Column {
    pub fn new() -> Self {
        Self {
            heaven_active: false,
            earth_active: 0,
        }
    }

    pub fn value(&self) -> u8 {
        let h = if self.heaven_active { 5 } else { 0 };
        h + self.earth_active
    }

    pub fn from_digit(digit: u8) -> Self {
        debug_assert!(digit < 10);
        Self {
            heaven_active: digit >= 5,
            earth_active: digit % 5,
        }
    }

    // Mechanical addition of 1 unit. Returns carry (0 or 1).
    pub fn add_one(&mut self) -> u8 {
        if self.earth_active < 4 {
            self.earth_active += 1;
            0
        } else {
            // Earth overflow (4 -> 5 or 9 -> 10)
            self.earth_active = 0;
            if !self.heaven_active {
                // 4 -> 5
                self.heaven_active = true;
                0
            } else {
                // 9 -> 10
                self.heaven_active = false;
                1
            }
        }
    }

    // Mechanical subtraction of 1 unit. Returns borrow (0 or 1).
    pub fn sub_one(&mut self) -> u8 {
        if self.earth_active > 0 {
            self.earth_active -= 1;
            0
        } else {
            // Earth underflow (5 -> 4 or 0 -> -1)
            if self.heaven_active {
                // 5 -> 4
                self.heaven_active = false;
                self.earth_active = 4;
                0
            } else {
                // 0 -> 9 (borrow)
                self.heaven_active = true;
                self.earth_active = 4;
                1
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Soroban {
    // Columns, ordered from LEAST significant (index 0 = 1s place)
    // to MOST significant.
    pub columns: Vec<Column>,
}

impl Soroban {
    pub fn new() -> Self {
        Self { columns: vec![Column::new()] }
    }

    pub fn zero() -> Self {
        Self::new()
    }

    pub fn from_u64(mut n: u64) -> Self {
        if n == 0 {
            return Self::new();
        }
        let mut columns = Vec::new();
        while n > 0 {
            columns.push(Column::from_digit((n % 10) as u8));
            n /= 10;
        }
        Self { columns }
    }

    pub fn to_u64(&self) -> u64 {
        let mut val = 0;
        let mut mult = 1;
        for col in &self.columns {
            val += (col.value() as u64) * mult;
            mult *= 10;
        }
        val
    }

    // Normalize removes leading zero columns (except the last one if value is 0)
    pub fn normalize(&mut self) {
        while self.columns.len() > 1 && self.columns.last().is_some_and(|c| c.value() == 0) {
            self.columns.pop();
        }
    }
}

impl PartialOrd for Soroban {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Soroban {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_len = std::cmp::max(self.columns.len(), other.columns.len());
        for i in (0..max_len).rev() {
            let val_self = self.columns.get(i).map_or(0, |c| c.value());
            let val_other = other.columns.get(i).map_or(0, |c| c.value());
            match val_self.cmp(&val_other) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

impl fmt::Display for Soroban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cols: Vec<&Column> = self.columns.iter().collect();
        // Remove trailing zeros for display if desired, or just show raw columns?
        // Let's show all columns but reversed (Most Significant First)
        if cols.is_empty() {
             write!(f, "[^|....]")?;
             return Ok(());
        }

        // Reverse for display: MSB -> LSB
        cols.reverse();

        for col in cols {
            let h_char = if col.heaven_active { 'v' } else { '^' };
            let mut e_str = String::new();
            for _ in 0..col.earth_active {
                e_str.push('*');
            }
            for _ in col.earth_active..4 {
                e_str.push('.');
            }
            write!(f, "[{}|{}]", h_char, e_str)?;
        }
        Ok(())
    }
}

impl Default for Soroban {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Soroban {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.clone();
        // Ensure result has enough columns to hold other + potential carry
        // We might grow dynamically, but let's pre-pad a bit logic wise.
        // Actually, vector grows automatically if we push.

        // Mechanical addition
        for (i, col) in other.columns.iter().enumerate() {
            let val = col.value();
            // Add 'val' units to result's i-th column
            for _ in 0..val {
                let mut current_idx = i;
                // Ensure column exists
                while current_idx >= result.columns.len() {
                    result.columns.push(Column::new());
                }

                let mut carry = result.columns[current_idx].add_one();
                while carry > 0 {
                    current_idx += 1;
                    while current_idx >= result.columns.len() {
                        result.columns.push(Column::new());
                    }
                    carry = result.columns[current_idx].add_one();
                }
            }
        }
        result.normalize();
        result
    }
}

impl Sub for Soroban {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.clone();

        for (i, col) in other.columns.iter().enumerate() {
            let val = col.value();
            for _ in 0..val {
                let mut current_idx = i;
                if current_idx >= result.columns.len() {
                     panic!("Soroban underflow");
                }

                let mut borrow = result.columns[current_idx].sub_one();
                while borrow > 0 {
                    current_idx += 1;
                    if current_idx >= result.columns.len() {
                         panic!("Soroban underflow");
                    }
                    borrow = result.columns[current_idx].sub_one();
                }
            }
        }
        result.normalize();
        result
    }
}
