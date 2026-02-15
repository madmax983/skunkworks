#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Column {
    /// True if the heaven bead (5) is down (active)
    pub upper_active: bool,
    /// Number of earth beads (1) up (active). Range 0-4.
    pub lower_active: u8,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            upper_active: false,
            lower_active: 0,
        }
    }
}

impl Column {
    pub fn value(&self) -> u8 {
        (if self.upper_active { 5 } else { 0 }) + self.lower_active
    }

    /// Sets the column value directly (for initialization/testing)
    pub fn set_value(&mut self, val: u8) {
        assert!(val <= 9, "Column value must be 0-9");
        self.upper_active = val >= 5;
        self.lower_active = val % 5;
    }
}

#[derive(Debug, Clone)]
pub struct Soroban {
    // Column 0 is the ones place, 1 is tens, etc.
    pub columns: [Column; 13],
}

impl Default for Soroban {
    fn default() -> Self {
        Self {
            columns: [Column::default(); 13],
        }
    }
}

impl Soroban {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(&self) -> u64 {
        let mut total = 0;
        let mut multiplier = 1;
        for col in self.columns.iter() {
            total += (col.value() as u64) * multiplier;
            multiplier *= 10;
        }
        total
    }

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

    fn add_to_column(&mut self, col_idx: usize, amount: u8) {
        if amount == 0 {
            return;
        }
        if col_idx >= 13 {
            return;
        } // Overflow ignored (or could panic)

        let current_lower = self.columns[col_idx].lower_active;
        let _sum_lower = current_lower + amount;

        // Simple logic first: reconstruct value, add, distribute back.
        // But to be "Ancient", we should try to manipulate beads.
        // Let's stick to correct state transition regardless of method for now.

        let current_val = self.columns[col_idx].value();
        let new_val_raw = current_val + amount;

        if new_val_raw >= 10 {
            // Carry
            let kept = new_val_raw - 10;
            self.columns[col_idx].set_value(kept);
            self.add_to_column(col_idx + 1, 1);
        } else {
            self.columns[col_idx].set_value(new_val_raw);
        }
    }

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

    fn borrow_from_next(&mut self, col_idx: usize) {
        if col_idx >= 13 {
            return;
        } // Underflow at top ignored

        if self.columns[col_idx].value() > 0 {
            self.sub_from_column(col_idx, 1);
        } else {
            // Need to borrow recursively
            self.borrow_from_next(col_idx + 1);
            // After borrowing, this column becomes 10, then we subtract 1 -> 9
            self.columns[col_idx].set_value(9);
        }
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
}
