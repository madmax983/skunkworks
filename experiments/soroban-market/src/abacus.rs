use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeadType {
    Heaven, // Value 5
    Earth,  // Value 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Activate,   // Move towards the beam
    Deactivate, // Move away from the beam
}

#[derive(Debug, Clone)]
pub struct BeadMove {
    pub column_idx: usize,
    pub bead_type: BeadType,
    pub amount: u8, // How many Earth beads to move, or 1 for Heaven
    pub action: Action,
}

#[derive(Clone, Debug)]
pub struct Column {
    pub heaven: bool, // true if active (down, touching beam) -> value 5
    pub earth: u8,    // number of active beads (up, touching beam) -> value 0-4
}

impl Column {
    pub fn new() -> Self {
        Self {
            heaven: false,
            earth: 0,
        }
    }

    pub fn value(&self) -> u8 {
        (if self.heaven { 5 } else { 0 }) + self.earth
    }
}

#[derive(Clone, Debug)]
pub struct Soroban {
    pub columns: Vec<Column>,
}

impl Soroban {
    pub fn new(num_columns: usize) -> Self {
        Self {
            columns: vec![Column::new(); num_columns],
        }
    }

    pub fn value(&self) -> u64 {
        let mut val = 0;
        let mut multiplier = 1;
        for col in self.columns.iter().rev() {
            val += (col.value() as u64) * multiplier;
            multiplier *= 10;
        }
        val
    }

    pub fn set_value(&mut self, mut val: u64) {
        for col in self.columns.iter_mut().rev() {
            let digit = (val % 10) as u8;
            col.heaven = digit >= 5;
            col.earth = digit % 5;
            val /= 10;
        }
    }

    // Adds a number and returns the sequence of moves for animation
    pub fn add(&mut self, val: u64) -> Vec<BeadMove> {
        let mut moves = Vec::new();
        let mut temp_val = val;
        let mut col_idx = self.columns.len().wrapping_sub(1);

        // We process from right to left (LSD to MSD)
        while temp_val > 0 && col_idx < self.columns.len() {
            let digit = (temp_val % 10) as u8;
            temp_val /= 10;

            if digit > 0 {
                self.add_digit_recursive(col_idx, digit, &mut moves);
            }

            col_idx = col_idx.wrapping_sub(1);
        }
        moves
    }

    fn add_digit_recursive(&mut self, col_idx: usize, val: u8, moves: &mut Vec<BeadMove>) {
        if col_idx >= self.columns.len() {
            // Overflow off the board
            return;
        }

        let current_val = self.columns[col_idx].value();
        let target = current_val + val;

        if target >= 10 {
            // Carry required
            // Algorithm: - (10 - val) then +1 to next column
            // Equivalent to: Adding `val` results in `target - 10` in this column and +1 in next.
            // Example: 8 + 3 = 11. End state: 1. Carry 1.
            // Operation: Current 8. We want to reach 1.
            // 8 -> 1 is -7.
            // So we subtract (10 - val).
            let subtract_amt = 10 - val;
            self.sub_digit_internal(col_idx, subtract_amt, moves);

            // Handle Carry
            if col_idx > 0 {
                self.add_digit_recursive(col_idx - 1, 1, moves);
            }
        } else {
            // No Carry
            // We just add `val` to `current_val`.
            // Determine moves based on Soroban rules (High Speed mechanics)

            let h = self.columns[col_idx].heaven;
            let e = self.columns[col_idx].earth;

            // Simple addition available?
            // If val < 5, and e + val <= 4: Just add Earth.
            if val < 5 && (e + val) <= 4 {
                self.columns[col_idx].earth += val;
                moves.push(BeadMove {
                    column_idx: col_idx,
                    bead_type: BeadType::Earth,
                    amount: val,
                    action: Action::Activate,
                });
            }
            // If val >= 5:
            else if val >= 5 {
                // val is 5, 6, 7, 8, 9.
                // If we add >= 5, we must activate Heaven.
                // But Heaven might already be active.
                // Wait, if target < 10, and we add >= 5...
                // Current must be small enough such that current + val < 10.
                // Max current if val=5 is 4. (4+5=9). Heaven is 0.
                // So Heaven MUST be inactive initially if we are here (adding >=5 without carry).

                // Activate Heaven (+5)
                self.columns[col_idx].heaven = true;
                moves.push(BeadMove {
                    column_idx: col_idx,
                    bead_type: BeadType::Heaven,
                    amount: 1,
                    action: Action::Activate,
                });

                // Add remainder to Earth
                let rem = val - 5;
                if rem > 0 {
                    // We know e + rem <= 4 because target < 10.
                    self.columns[col_idx].earth += rem;
                    moves.push(BeadMove {
                        column_idx: col_idx,
                        bead_type: BeadType::Earth,
                        amount: rem,
                        action: Action::Activate,
                    });
                }
            }
            // If val < 5 but e + val > 4 (Need 5 complement)
            else {
                // Example: Current 4. Add 1. Target 5.
                // +5 (Heaven Activate), -4 (Earth Deactivate).
                // Example: Current 3. Add 3. Target 6.
                // +5 (Heaven Activate), -2 (Earth Deactivate).
                // Logic: Activate Heaven (+5). Deactivate Earth (5 - val).

                self.columns[col_idx].heaven = true;
                moves.push(BeadMove {
                    column_idx: col_idx,
                    bead_type: BeadType::Heaven,
                    amount: 1,
                    action: Action::Activate,
                });

                let earth_sub = 5 - val;
                self.columns[col_idx].earth -= earth_sub;
                moves.push(BeadMove {
                    column_idx: col_idx,
                    bead_type: BeadType::Earth,
                    amount: earth_sub,
                    action: Action::Deactivate,
                });
            }
        }
    }

    pub fn sub(&mut self, val: u64) -> Vec<BeadMove> {
        let mut moves = Vec::new();
        let mut temp_val = val;
        let mut col_idx = self.columns.len().wrapping_sub(1);

        while temp_val > 0 && col_idx < self.columns.len() {
            let digit = (temp_val % 10) as u8;
            temp_val /= 10;

            if digit > 0 {
                self.sub_digit_recursive(col_idx, digit, &mut moves);
            }

            col_idx = col_idx.wrapping_sub(1);
        }
        moves
    }

    fn sub_digit_recursive(&mut self, col_idx: usize, val: u8, moves: &mut Vec<BeadMove>) {
        if col_idx >= self.columns.len() { return; }

        let current_val = self.columns[col_idx].value();

        if val <= current_val {
            // No borrow needed
            self.sub_digit_internal(col_idx, val, moves);
        } else {
            // Borrow needed
            // Logic: -val = -10 + (10 - val)
            // Borrow 1 from next column (left), then add (10-val) to this column.
            if col_idx > 0 {
                self.sub_digit_recursive(col_idx - 1, 1, moves);
                let add_amt = 10 - val;
                self.add_digit_recursive(col_idx, add_amt, moves); // Recursive add won't carry because current < val so current + (10-val) < 10.
                                                                   // Wait. Current < val. Max current = val-1.
                                                                   // Max val = 9.
                                                                   // If current=0, val=1. Add 9. Res 9. No carry.
                                                                   // If current=8, val=9. Add 1. Res 9. No carry.
                                                                   // Correct.
            } else {
                // Underflow! We can't borrow.
                // In a physical abacus, you just end up with a mess or negative representation isn't supported like this.
                // We'll ignore or panic. Let's ignore.
            }
        }
    }

    // Helper for non-carrying subtraction (guaranteed possible)
    fn sub_digit_internal(&mut self, col_idx: usize, val: u8, moves: &mut Vec<BeadMove>) {
        let e = self.columns[col_idx].earth;
        let h = self.columns[col_idx].heaven;

        // Simple subtraction?
        // If val < 5 and e >= val: Just sub Earth.
        if val < 5 && e >= val {
            self.columns[col_idx].earth -= val;
            moves.push(BeadMove {
                column_idx: col_idx,
                bead_type: BeadType::Earth,
                amount: val,
                action: Action::Deactivate,
            });
        }
        // If val >= 5:
        else if val >= 5 {
            // Must have Heaven active because val >= 5.
            // -5 (Heaven Deactivate)
            self.columns[col_idx].heaven = false;
            moves.push(BeadMove {
                column_idx: col_idx,
                bead_type: BeadType::Heaven,
                amount: 1,
                action: Action::Deactivate,
            });

            let rem = val - 5;
            if rem > 0 {
                self.columns[col_idx].earth -= rem;
                moves.push(BeadMove {
                    column_idx: col_idx,
                    bead_type: BeadType::Earth,
                    amount: rem,
                    action: Action::Deactivate,
                });
            }
        }
        // If val < 5 but e < val (Need 5 complement)
        else {
            // Example: Current 5 (H=1, E=0). Sub 1. Target 4.
            // -5 (Heaven Deactivate), +4 (Earth Activate).
            // Logic: Deactivate Heaven (-5). Activate Earth (5 - val).

            self.columns[col_idx].heaven = false;
            moves.push(BeadMove {
                column_idx: col_idx,
                bead_type: BeadType::Heaven,
                amount: 1,
                action: Action::Deactivate,
            });

            let earth_add = 5 - val;
            self.columns[col_idx].earth += earth_add;
            moves.push(BeadMove {
                column_idx: col_idx,
                bead_type: BeadType::Earth,
                amount: earth_add,
                action: Action::Activate,
            });
        }
    }
}

impl fmt::Display for Soroban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Render top to bottom
        // Top Frame
        // Heaven Beads (Active is down)
        // Beam
        // Earth Beads (Active is up)
        // Bottom Frame

        // Visualization for Debug:
        // Col: 0 1 2
        //      | | |
        // H:   * | *  (Active down near beam)
        //      - - -
        // E:   * * *
        //      * * |
        //      | | |

        // Actually, let's just print the values for now.
        for col in &self.columns {
            write!(f, "[{}|{}] ", if col.heaven { 5 } else { 0 }, col.earth)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut s = Soroban::new(3); // 000
        s.add(1);
        assert_eq!(s.value(), 1);
        s.add(3);
        assert_eq!(s.value(), 4);
        s.add(5); // 4 + 5 = 9
        assert_eq!(s.value(), 9);
    }

    #[test]
    fn test_carry_add() {
        let mut s = Soroban::new(3);
        s.set_value(9);
        s.add(1); // 9 + 1 = 10
        assert_eq!(s.value(), 10);

        s.set_value(99);
        s.add(1);
        assert_eq!(s.value(), 100);
    }

    #[test]
    fn test_complex_carry() {
        let mut s = Soroban::new(3);
        s.set_value(5);
        s.add(5); // 5+5=10
        assert_eq!(s.value(), 10);

        s.set_value(8);
        s.add(7); // 8+7=15
        assert_eq!(s.value(), 15);
    }

    #[test]
    fn test_sub() {
        let mut s = Soroban::new(3);
        s.set_value(10);
        s.sub(1);
        assert_eq!(s.value(), 9);

        s.set_value(100);
        s.sub(1);
        assert_eq!(s.value(), 99);

        s.set_value(50);
        s.sub(5);
        assert_eq!(s.value(), 45);
    }

    #[test]
    fn test_moves_generated() {
        let mut s = Soroban::new(2);
        let moves = s.add(5);
        // Should generate moves. 5 is Heaven Down.
        // Action Activate Heaven.
        assert!(!moves.is_empty());
        assert_eq!(moves[0].bead_type, BeadType::Heaven);
        assert_eq!(moves[0].action, Action::Activate);
    }
}
