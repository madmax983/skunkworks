use std::cmp::max;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

/// Represents a single rod on the Suanpan.
/// Traditional 2-5 Suanpan:
/// - 2 Heaven beads (each worth 5)
/// - 5 Earth beads (each worth 1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rod {
    /// Number of active Heaven beads (0-2). Each counts as 5.
    pub heaven: u8,
    /// Number of active Earth beads (0-5). Each counts as 1.
    pub earth: u8,
}

impl Rod {
    pub fn new() -> Self {
        Self { heaven: 0, earth: 0 }
    }

    /// Calculate the value of this rod (0-15).
    pub fn value(&self) -> u8 {
        self.heaven * 5 + self.earth
    }

    /// Create a rod state from a value (0-15).
    pub fn from_value(val: u8) -> Self {
        let val = val.min(15);
        let heaven = val / 5;
        let earth = val % 5;
        let (heaven, earth) = if heaven > 2 {
            (2, 5)
        } else {
            (heaven, earth)
        };

        Self { heaven, earth }
    }

    pub fn reset(&mut self) {
        self.heaven = 0;
        self.earth = 0;
    }
}

impl Default for Rod {
    fn default() -> Self {
        Self::new()
    }
}

/// The Suanpan (Abacus).
#[derive(Debug, Clone, PartialEq)]
pub struct Suanpan {
    pub rods: Vec<Rod>,
}

impl Suanpan {
    pub fn new(width: usize) -> Self {
        Self {
            rods: vec![Rod::new(); width],
        }
    }

    pub fn to_u64(&self) -> u64 {
        let mut value: u64 = 0;
        let mut multiplier: u64 = 1;

        for rod in &self.rods {
            value += (rod.value() as u64) * multiplier;
            multiplier *= 10;
        }
        value
    }

    pub fn normalize(&mut self) {
        let mut carry = 0;
        for i in 0..self.rods.len() {
            let mut val = self.rods[i].value() + carry;
            if val >= 10 {
                carry = val / 10;
                val %= 10;
            } else {
                carry = 0;
            }
            self.rods[i] = Rod::from_value(val);
        }
        while carry > 0 {
            let val = carry % 10;
            carry /= 10;
            self.rods.push(Rod::from_value(val));
        }
    }
}

impl From<u64> for Suanpan {
    fn from(val: u64) -> Self {
        let mut rods = Vec::new();
        let mut current = val;

        if current == 0 {
             return Self::new(13);
        }

        while current > 0 {
            let digit = (current % 10) as u8;
            rods.push(Rod::from_value(digit));
            current /= 10;
        }

        while rods.len() < 13 {
            rods.push(Rod::new());
        }

        Self { rods }
    }
}

impl fmt::Display for Suanpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Suanpan[")?;
        for (i, rod) in self.rods.iter().enumerate().rev() {
            if i < self.rods.len() - 1 {
                write!(f, "|")?;
            }
            write!(f, "{}", rod.value())?;
        }
        write!(f, "]")
    }
}

impl Add for Suanpan {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let max_len = max(self.rods.len(), other.rods.len());
        let mut result_rods = Vec::with_capacity(max_len + 1);

        let mut carry = 0;

        for i in 0..max_len {
            let r1 = self.rods.get(i).cloned().unwrap_or_default();
            let r2 = other.rods.get(i).cloned().unwrap_or_default();

            let mut earth = r1.earth + r2.earth + carry;
            let mut heaven = r1.heaven + r2.heaven;

            while earth >= 5 {
                earth -= 5;
                heaven += 1;
            }

            if heaven >= 2 {
                carry = heaven / 2;
                heaven %= 2;
            } else {
                carry = 0;
            }

            result_rods.push(Rod { heaven, earth });
        }

        while carry > 0 {
             result_rods.push(Rod { heaven: 0, earth: carry });
             carry = 0;
        }

        Suanpan { rods: result_rods }
    }
}

impl Sub for Suanpan {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let max_len = max(self.rods.len(), other.rods.len());
        let mut result_rods = Vec::with_capacity(max_len);

        let mut borrow = 0;

        for i in 0..max_len {
            let r1 = self.rods.get(i).cloned().unwrap_or_default();
            let r2 = other.rods.get(i).cloned().unwrap_or_default();

            let val_self = (r1.earth + r1.heaven * 5) as i8;
            let val_other = (r2.earth + r2.heaven * 5) as i8 + borrow;

            if val_self >= val_other {
                let diff = val_self - val_other;
                result_rods.push(Rod::from_value(diff as u8));
                borrow = 0;
            } else {
                // Borrow adds 10 (2 Heavens) to self
                let diff = (val_self + 10) - val_other;
                result_rods.push(Rod::from_value(diff as u8));
                borrow = 1;
            }
        }

        if borrow > 0 {
            // Underflow returns 0
            return Suanpan::new(max_len);
        }

        while result_rods.len() > 1 && result_rods.last().map(|r| r.value() == 0).unwrap_or(false) {
            result_rods.pop();
        }

        Suanpan { rods: result_rods }
    }
}

impl AddAssign for Suanpan {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl SubAssign for Suanpan {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rod_values() {
        let r = Rod::from_value(7);
        assert_eq!(r.heaven, 1);
        assert_eq!(r.earth, 2);
        assert_eq!(r.value(), 7);
    }

    #[test]
    fn test_suanpan_conversion() {
        let val = 123456789;
        let s = Suanpan::from(val);
        assert_eq!(s.to_u64(), val);
    }

    #[test]
    fn test_add() {
        let a = Suanpan::from(15);
        let b = Suanpan::from(7);
        let c = a + b;
        assert_eq!(c.to_u64(), 22);
    }

    #[test]
    fn test_add_carry() {
        let a = Suanpan::from(999);
        let b = Suanpan::from(1);
        let c = a + b;
        assert_eq!(c.to_u64(), 1000);
    }

    #[test]
    fn test_sub() {
        let a = Suanpan::from(22);
        let b = Suanpan::from(7);
        let c = a - b;
        assert_eq!(c.to_u64(), 15);
    }

    #[test]
    fn test_sub_borrow() {
        let a = Suanpan::from(1000);
        let b = Suanpan::from(1);
        let c = a - b;
        assert_eq!(c.to_u64(), 999);
    }
}
