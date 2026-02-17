//! # Soroban: The Ancient Calculator 🧮
//!
//! A **Soroban** (算盤) is a Japanese abacus, optimized for decimal calculation.
//! Unlike the Chinese Suanpan (which has 2 heaven and 5 earth beads), the modern Soroban
//! has:
//!
//! *   **1 Heaven Bead** (Upper Deck): Represents the value **5**.
//! *   **4 Earth Beads** (Lower Deck): Each represents the value **1**.
//!
//! This minimal design allows for extremely fast mental calculation (Anzan).
//!
//! ## How to Read a Soroban
//!
//! Each column represents a decimal digit (Units, Tens, Hundreds, etc.).
//!
//! - **The Beam (Horizontal Bar):** The divider between the upper and lower deck. Beads only have value when they are pushed *towards* the beam.
//! - **Heaven Bead (Top):** Worth **5**. It is active when pushed *down* to the beam.
//! - **Earth Beads (Bottom):** Worth **1** each. They are active when pushed *up* to the beam.
//!
//! ### Bead Values
//!
//! | Digit | Heaven (5) | Earth (1s) | Visual |
//! |-------|------------|------------|--------|
//! | **0** | Up (Off)   | 0 Up       | `   |   ` |
//! | **1** | Up (Off)   | 1 Up       | `   |*  ` |
//! | **5** | Down (On)  | 0 Up       | ` * |   ` |
//! | **7** | Down (On)  | 2 Up       | ` * |** ` |
//!
//! ## Example
//!
//! ```
//! use soroban::Soroban;
//!
//! let mut s = Soroban::new();
//!
//! // Add 123
//! s += 123;
//! assert_eq!(s.value(), 123);
//!
//! // Visual Output via Display trait:
//! // println!("{}", s);
//! ```

use std::fmt;

/// A single rod on the Soroban, representing one decimal digit (0-9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Column {
    /// The Heaven Bead (Value 5).
    ///
    /// * `true`: Bead is **Down** (Active). Value adds 5.
    /// * `false`: Bead is **Up** (Inactive). Value adds 0.
    pub upper_active: bool,

    /// The Earth Beads (Value 1 each).
    ///
    /// * Range: 0 to 4.
    /// * Represents the number of beads pushed **Up** (Active) against the beam.
    pub lower_active: u8,
}

impl Column {
    /// Returns the decimal value of this column (0-9).
    pub fn value(&self) -> u8 {
        (self.upper_active as u8 * 5) + self.lower_active
    }

    /// Sets the column value directly (for initialization/testing).
    ///
    /// # Panics
    /// Clamps `val` to 9 if it exceeds the limit.
    pub fn set_value(&mut self, val: u8) {
        // Clamping value to prevent panic
        let val = val.min(9);
        self.upper_active = val >= 5;
        self.lower_active = val - (self.upper_active as u8 * 5);
    }
}

/// A 13-column Japanese Abacus.
///
/// Capable of representing numbers up to $10^{13} - 1$ (10 Trillion).
///
/// # Layout
/// * **Column 0**: The "Ones" place ($10^0$).
/// * **Column 1**: The "Tens" place ($10^1$).
/// * ...
/// * **Column 12**: The "Trillions" place ($10^{12}$).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Soroban {
    // Column 0 is the ones place, 1 is tens, etc.
    pub columns: [Column; 13],
}

impl From<u64> for Soroban {
    /// Creates a Soroban from a u64.
    ///
    /// # Examples
    ///
    /// ```
    /// use soroban::Soroban;
    /// let s = Soroban::from(123);
    /// assert_eq!(s.value(), 123);
    /// ```
    fn from(val: u64) -> Self {
        let mut s = Soroban::new();
        s.add(val);
        s
    }
}

impl Soroban {
    /// Creates a new, zeroed Soroban.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total integer value represented by the Soroban.
    pub fn value(&self) -> u64 {
        let mut total = 0;
        let mut multiplier = 1;
        for col in self.columns.iter() {
            total += (col.value() as u64) * multiplier;
            multiplier *= 10;
        }
        total
    }

    /// Adds a number to the Soroban.
    ///
    /// Simulates the physical process of adding bead values, including
    /// "Carrying" over to the next column when a column exceeds 9.
    ///
    /// # Examples
    ///
    /// ```
    /// use soroban::Soroban;
    /// let mut s = Soroban::new();
    /// s.add(5);
    /// assert_eq!(s.value(), 5);
    /// s += 7; // Use AddAssign trait (5 + 7 = 12)
    /// assert_eq!(s.value(), 12);
    /// ```
    pub fn add(&mut self, val: u64) {
        let mut temp_val = val;
        let mut col_idx = 0;

        while temp_val > 0 && col_idx < 13 {
            let digit = (temp_val % 10) as u8;
            self.add_to_column(col_idx, digit);
            temp_val /= 10;
            col_idx += 1;
        }
    }

    fn add_to_column(&mut self, mut col_idx: usize, mut amount: u8) {
        while amount > 0 && col_idx < 13 {
            let current_val = self.columns[col_idx].value();
            let new_val_raw = current_val + amount;

            if new_val_raw >= 10 {
                // Carry
                let kept = new_val_raw - 10;
                self.columns[col_idx].set_value(kept);
                amount = 1;
                col_idx += 1;
            } else {
                self.columns[col_idx].set_value(new_val_raw);
                amount = 0;
            }
        }
    }

    /// Subtracts a number from the Soroban.
    ///
    /// Simulates the physical process of removing bead values, including
    /// "Borrowing" from higher columns when a column cannot subtract the amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use soroban::Soroban;
    /// let mut s = Soroban::new();
    /// s += 10;
    /// s -= 3; // Use SubAssign trait (10 - 3 = 7)
    /// assert_eq!(s.value(), 7);
    /// ```
    pub fn sub(&mut self, val: u64) {
        let mut temp_val = val;
        let mut col_idx = 0;

        while temp_val > 0 && col_idx < 13 {
            let digit = (temp_val % 10) as u8;
            self.sub_from_column(col_idx, digit);
            temp_val /= 10;
            col_idx += 1;
        }
    }

    fn sub_from_column(&mut self, col_idx: usize, amount: u8) {
        if amount == 0 {
            return;
        }
        if col_idx >= 13 {
            return;
        }

        let current_val = self.columns[col_idx].value();

        if amount <= current_val {
            self.columns[col_idx].set_value(current_val - amount);
        } else {
            // Borrow
            // We need to borrow 10 from higher column
            self.borrow_from_next(col_idx + 1);
            let borrowed_val = current_val + 10;
            self.columns[col_idx].set_value(borrowed_val - amount);
        }
    }

    fn borrow_from_next(&mut self, start_idx: usize) {
        let mut idx = start_idx;
        // Find the first non-zero column
        while idx < 13 && self.columns[idx].value() == 0 {
            idx += 1;
        }

        if idx >= 13 {
            // Underflow: borrow from "infinity" (wrap around)
            // Original recursive behavior: set all traversed columns to 9
            for i in start_idx..13 {
                self.columns[i].set_value(9);
            }
            return;
        }

        // Decrease that column by 1
        let val = self.columns[idx].value();
        self.columns[idx].set_value(val - 1);

        // Set all intermediate columns to 9
        for i in start_idx..idx {
            self.columns[i].set_value(9);
        }
    }
}

impl std::ops::AddAssign<u64> for Soroban {
    fn add_assign(&mut self, rhs: u64) {
        self.add(rhs);
    }
}

impl std::ops::SubAssign<u64> for Soroban {
    fn sub_assign(&mut self, rhs: u64) {
        self.sub(rhs);
    }
}

impl std::ops::Add<u64> for Soroban {
    type Output = Soroban;

    fn add(mut self, rhs: u64) -> Self::Output {
        Soroban::add(&mut self, rhs);
        self
    }
}

impl std::ops::Sub<u64> for Soroban {
    type Output = Soroban;

    fn sub(mut self, rhs: u64) -> Self::Output {
        Soroban::sub(&mut self, rhs);
        self
    }
}

impl fmt::Display for Soroban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate columns from highest power (12) to units (0)
        let iter = self.columns.iter().rev();

        // Row 0: Heaven Inactive (Up position)
        for col in iter.clone() {
            if !col.upper_active {
                write!(f, " O ")?;
            } else {
                write!(f, " | ")?;
            }
        }
        writeln!(f)?;

        // Row 1: Heaven Active (Down position)
        for col in iter.clone() {
            if col.upper_active {
                write!(f, " O ")?;
            } else {
                write!(f, " | ")?;
            }
        }
        writeln!(f)?;

        // Row 2: Beam
        for _ in 0..13 {
            write!(f, "---")?;
        }
        writeln!(f)?;

        // Earth Beads (5 slots for 4 beads + 1 gap)
        for slot in 0..5 {
            for col in iter.clone() {
                // Determine what is in this slot: Active Bead, Gap, or Inactive Bead
                if slot < col.lower_active {
                    write!(f, " O ")?; // Active Bead (Pushed Up)
                } else if slot == col.lower_active {
                    write!(f, " | ")?; // Gap
                } else {
                    write!(f, " O ")?; // Inactive Bead (Resting Down)
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let s = Soroban::new();
        assert_eq!(s.value(), 0);
    }

    #[test]
    fn test_set_value() {
        let mut c = Column::default();
        c.set_value(7);
        assert!(c.upper_active); // 5
        assert_eq!(c.lower_active, 2); // 2
        assert_eq!(c.value(), 7);
    }

    #[test]
    fn test_add_simple() {
        let mut s = Soroban::new();
        s.add(5);
        assert_eq!(s.value(), 5);
        assert!(s.columns[0].upper_active);
        assert_eq!(s.columns[0].lower_active, 0);

        s.add(2);
        assert_eq!(s.value(), 7);
        assert!(s.columns[0].upper_active);
        assert_eq!(s.columns[0].lower_active, 2);
    }

    #[test]
    fn test_add_carry() {
        let mut s = Soroban::new();
        s.add(9);
        s.add(1);
        assert_eq!(s.value(), 10);
        assert_eq!(s.columns[0].value(), 0);
        assert_eq!(s.columns[1].value(), 1);
    }

    #[test]
    fn test_add_large() {
        let mut s = Soroban::new();
        s.add(123456789);
        assert_eq!(s.value(), 123456789);
        s.add(1);
        assert_eq!(s.value(), 123456790);
    }

    #[test]
    fn test_sub_simple() {
        let mut s = Soroban::new();
        s.add(10);
        s.sub(3);
        assert_eq!(s.value(), 7);
    }

    #[test]
    fn test_sub_borrow() {
        let mut s = Soroban::new();
        s.add(100);
        s.sub(1);
        assert_eq!(s.value(), 99);
        assert_eq!(s.columns[0].value(), 9);
        assert_eq!(s.columns[1].value(), 9);
        assert_eq!(s.columns[2].value(), 0);
    }

    #[test]
    fn test_carry_overflow() {
        let mut s = Soroban::new();
        // 9,999,999,999,999 (13 nines)
        for i in 0..13 {
            s.columns[i].set_value(9);
        }
        s.add(1);
        // Should wrap around to 0 effectively for 13 columns?
        // Or if we check columns, they should be 0.
        // Wait, add_to_column checks col_idx >= 13.
        assert_eq!(s.value(), 0);
    }

    #[test]
    fn test_borrow_underflow() {
        let mut s = Soroban::new();
        s.sub(1);
        // 0 - 1 = -1 (mod 10^13) -> 9,999,999,999,999
        let expected = 9_999_999_999_999u64;
        assert_eq!(s.value(), expected);
    }

    #[test]
    fn test_fuzz_arithmetic() {
        // Simple Linear Congruential Generator to avoid dependencies
        let mut seed = 123456789u64;
        let mut rng = move || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed
        };

        let mut s = Soroban::new();
        // Shadow value is modulo 10^13
        let modulus = 10_000_000_000_000u64;
        let mut shadow = 0u64;

        // Run 10,000 random operations
        for i in 0..10_000 {
            let op = rng() % 2;
            // Allow values larger than modulus to test wrapping behavior
            let val = rng() % (modulus * 10);

            if op == 0 {
                // Add
                s.add(val);
                shadow = (shadow + val) % modulus;
            } else {
                // Sub
                s.sub(val);
                let val_mod = val % modulus;
                if val_mod > shadow {
                    shadow = shadow + modulus - val_mod;
                } else {
                    shadow -= val_mod;
                }
            }

            assert_eq!(
                s.value(),
                shadow,
                "Mismatch at iter {}: op {} val {} (shadow: {}, soroban: {})",
                i,
                if op == 0 { "ADD" } else { "SUB" },
                val,
                shadow,
                s.value()
            );
        }
    }
}
