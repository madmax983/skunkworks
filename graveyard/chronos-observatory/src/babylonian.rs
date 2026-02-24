use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// Babylonian Number (Base 60)
/// Wraps a rational number for precise arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BabylonianNumber(pub Ratio<i64>);

impl BabylonianNumber {
    pub fn new(num: i64, den: i64) -> Self {
        Self(Ratio::new(num, den))
    }

    pub fn from_f64(f: f64) -> Self {
        // Ratio::from_float returns Option<Ratio<BigInt>>
        let r_big = Ratio::from_float(f).unwrap_or_else(Ratio::zero);
        // Convert BigInt to i64 (saturating or truncating or failing)
        // Ideally we check bounds. For simplicity, unwrap_or(0).
        let num = r_big.numer().to_i64().unwrap_or(0);
        let den = r_big.denom().to_i64().unwrap_or(1);
        Self(Ratio::new(num, den))
    }

    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap_or(0.0)
    }

    /// Convert a digit (0-59) to Cuneiform string
    fn digit_to_cuneiform(digit: u8) -> String {
        if digit == 0 {
            return " ".to_string(); // Placeholder/Space
        }

        let tens = digit / 10;
        let units = digit % 10;

        let mut s = String::new();

        // U+12423 (𒌋) for 10
        for _ in 0..tens {
            s.push('\u{12423}');
        }

        // U+12415 (𒐕) for 1
        for _ in 0..units {
            s.push('\u{12415}');
        }

        s
    }
}

impl fmt::Display for BabylonianNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0.abs();
        let sign = if self.0 < Ratio::zero() { "-" } else { "" };

        let integer_part = val.to_integer();
        let mut fraction = val - Ratio::from_integer(integer_part);

        // Format integer part in base 60
        let mut int_digits = Vec::new();
        let mut n = integer_part; // Always positive because val is abs()

        if n == 0 {
            int_digits.push(0);
        } else {
            while n > 0 {
                int_digits.push((n % 60) as u8);
                n /= 60;
            }
            int_digits.reverse();
        }

        write!(f, "{}", sign)?;

        for (i, &d) in int_digits.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", Self::digit_to_cuneiform(d))?;
        }

        // Format fractional part (up to 3 sexagesimal places)
        if !fraction.is_zero() {
            write!(f, ";")?;
            for i in 0..3 {
                fraction *= Ratio::from_integer(60);
                let digit = fraction.to_integer();

                // Write digit, followed by space if not last
                write!(f, "{}", Self::digit_to_cuneiform(digit as u8))?;

                fraction -= Ratio::from_integer(digit);

                if fraction.is_zero() {
                    break;
                }
                if i < 2 {
                    write!(f, " ")?;
                }
            }
        }

        Ok(())
    }
}

impl Add for BabylonianNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BabylonianNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for BabylonianNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for BabylonianNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0.is_zero() {
            return Self(Ratio::zero());
        }
        Self(self.0 / rhs.0)
    }
}

impl From<i64> for BabylonianNumber {
    fn from(n: i64) -> Self {
        Self(Ratio::from_integer(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let a = BabylonianNumber::from(10);
        let b = BabylonianNumber::from(2);
        assert_eq!((a + b).0.to_integer(), 12);
        assert_eq!((a / b).0.to_integer(), 5);
    }

    #[test]
    fn test_display_integer() {
        // 61 = 1, 1 = 𒐕 𒐕
        let n = BabylonianNumber::from(61);
        // Digit 1: 𒐕
        // Space
        // Digit 1: 𒐕
        // The display string should contain two 1s separated by space.
        let s = format!("{}", n);
        assert!(s.contains('\u{12415}'));
        assert!(s.contains(' '));
    }

    #[test]
    fn test_display_fraction() {
        // 1.5 = 1;30
        let n = BabylonianNumber::new(3, 2);
        // 1 = 𒐕
        // ;
        // 30 = 𒌋𒌋𒌋 (3 tens)
        let s = format!("{}", n);
        assert!(s.contains('\u{12415}')); // 1
        assert!(s.contains(';'));
        assert!(s.contains('\u{12423}')); // 10
    }
}
