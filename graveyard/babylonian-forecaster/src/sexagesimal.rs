use anyhow::{anyhow, Result};
use std::cmp::max;
use std::fmt;
use std::ops::{Add, Div, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sexagesimal {
    // Stored Big-Endian (Most significant digit first)
    pub digits: Vec<u8>,
}

impl Sexagesimal {
    pub fn new(digits: Vec<u8>) -> Self {
        Self { digits }
    }

    pub fn zero() -> Self {
        Self { digits: vec![0] }
    }

    // Helper to normalize (remove leading zeros)
    pub fn normalize(&mut self) {
        while self.digits.len() > 1 && self.digits[0] == 0 {
            self.digits.remove(0);
        }
        if self.digits.is_empty() {
            self.digits.push(0);
        }
    }

    pub fn to_f64(&self) -> f64 {
        let mut val = 0.0;
        for &digit in &self.digits {
            val = val * 60.0 + (digit as f64);
        }
        val
    }

    pub fn from_u64(mut val: u64) -> Self {
        if val == 0 {
            return Self::zero();
        }
        let mut digits = Vec::new();
        while val > 0 {
            digits.push((val % 60) as u8);
            val /= 60;
        }
        digits.reverse();
        Self::new(digits)
    }
}

impl FromStr for Sexagesimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_whitespace();
        let mut digits = Vec::new();

        for part in parts {
            let mut val = 0;
            for c in part.chars() {
                match c {
                    '𒐕' => val += 1,
                    '𒌋' => val += 10,
                    // Fallback/Alternatives if standard fonts miss them?
                    // Let's stick to the prompt's chars.
                    _ => return Err(anyhow!("Invalid character: {}", c)),
                }
            }
            if val >= 60 {
                return Err(anyhow!("Digit {} exceeds 59", val));
            }
            digits.push(val);
        }

        if digits.is_empty() {
            return Ok(Self::zero());
        }

        let mut s = Self::new(digits);
        s.normalize();
        Ok(s)
    }
}

impl fmt::Display for Sexagesimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for &digit in &self.digits {
            if !first {
                write!(f, " ")?;
            }
            first = false;

            if digit == 0 {
                // Babylonian zero was sometimes a space or specific symbol.
                // Later texts used "DOUBLE OBLIQUE WEDGE LOW" (PLACEHOLDER).
                // But for now, let's just print nothing or a placeholder?
                // If it's a digit 0 in the middle, we need to show it.
                // Using "Double Oblique Wedge" (looks like two small wedges).
                // U+12031  পুনরা (placeholder?) No.
                // U+12449  শূন্য (This is Bengali).
                // Babylonian zero:  বিভ্রান্ত (Wait no).
                // Let's use a generic placeholder like "||" or empty if we stick to additive.
                // But "10, 0, 1" needs to distinguish from "10, 1".
                // Let's use "𒊹" (SHAR? No).
                // Let's use "''" or similar.
                // Actually, the prompt says "NO modern decimal".
                // I'll use a specific character for 0 if non-zero exists.
                // If the number is just 0, print "ZERO" in cuneiform?
                // Let's use U+1224C (SIGN NU) "NOT". Or just print nothing if 0 is leading?
                // But I'm normalizing.
                if digit == 0 {
                    write!(f, "''")?;
                    continue;
                }
            }

            let tens = digit / 10;
            let ones = digit % 10;

            for _ in 0..tens {
                write!(f, "𒌋")?;
            }
            for _ in 0..ones {
                write!(f, "𒐕")?;
            }
        }
        Ok(())
    }
}

impl Add for Sexagesimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result_digits = Vec::new();
        let mut carry = 0;

        let len = max(self.digits.len(), other.digits.len());

        let mut a_iter = self.digits.iter().rev();
        let mut b_iter = other.digits.iter().rev();

        for _ in 0..len {
            let a_val = *a_iter.next().unwrap_or(&0);
            let b_val = *b_iter.next().unwrap_or(&0);

            let sum = a_val as u16 + b_val as u16 + carry;
            result_digits.push((sum % 60) as u8);
            carry = sum / 60;
        }

        while carry > 0 {
            result_digits.push((carry % 60) as u8);
            carry /= 60;
        }

        result_digits.reverse();
        let mut res = Self::new(result_digits);
        res.normalize();
        res
    }
}

impl Div<u64> for Sexagesimal {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        if rhs == 0 {
            panic!("Division by zero");
        }
        let mut result_digits = Vec::new();
        let mut remainder: u64 = 0;

        for &digit in &self.digits {
            let current = remainder * 60 + (digit as u64);
            let res_digit = current / rhs;
            remainder = current % rhs;

            if !result_digits.is_empty() || res_digit > 0 {
                // If res_digit > 59, it means we have overflowed a single digit place?
                // Wait. If rhs is small, say 2. current can be 59*60+59 ~ 3600. / 2 = 1800.
                // This doesn't fit in a single base-60 digit (max 59).
                // Ah, long division works if the quotient digit < base.
                // If rhs is small, the quotient can be large.
                // But in base-60, we are dividing the *value* represented by the current prefix.
                // Actually, if we divide by `rhs`, the result is a number.
                // This `Div` implementation should return `Sexagesimal`.
                // The algorithm "current = remainder * 60 + digit" assumes we are producing one base-60 digit at a time.
                // This only works if `current / rhs` < 60.
                // This implies `rhs` must be large enough OR the result digits might need to be multi-digit?
                // No, standard long division produces digits in the same base.
                // If `current / rhs` >= 60, it means the result digit is too big?
                // No, it means we should have divided earlier?
                // Wait.
                // Example: 120 / 2. Base 10.
                // Digits: [1, 2, 0].
                // 1/2 = 0. rem 1.
                // 1*10+2 = 12. 12/2 = 6. rem 0.
                // 0*10+0 = 0. 0/2 = 0. rem 0.
                // Result: 060 = 60. Correct.

                // In Base 60:
                // 120 = [2, 0] (2*60).
                // 2 / 2 = 1. rem 0.
                // 0*60+0 = 0. 0/2 = 0.
                // Result [1, 0] = 60. Correct.

                // What if result digit >= 60?
                // `current` max is `(rhs-1)*60 + 59`.
                // `current / rhs` <= `((rhs-1)*60 + 59)/rhs` = `60 - 60/rhs + 59/rhs` < 60 + 1.
                // So it can be 60?
                // If rhs=1. `current` max 59. 59/1 = 59. OK.
                // If rhs=2. `current` max 1*60+59 = 119. 119/2 = 59.5 -> 59. OK.
                // If remainder is `rhs-1`, then `(rhs-1)*60 + 59`.
                // `((rhs-1)*60 + 59) / rhs` = `(60*rhs - 60 + 59) / rhs` = `60 - 1/rhs`.
                // So strictly less than 60.
                // So result digit is always < 60.
                // YES!

                result_digits.push(res_digit as u8);
            }
        }

        if result_digits.is_empty() {
            return Self::zero();
        }

        let mut res = Self::new(result_digits);
        res.normalize();
        res
    }
}

impl Sub for Sexagesimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        // Assume self >= other
        let mut result_digits = Vec::new();
        let mut borrow = 0;

        let len = max(self.digits.len(), other.digits.len());
        let mut a_iter = self.digits.iter().rev();
        let mut b_iter = other.digits.iter().rev();

        for _ in 0..len {
            let a_val = *a_iter.next().unwrap_or(&0) as i16;
            let b_val = *b_iter.next().unwrap_or(&0) as i16;

            let mut diff = a_val - b_val - borrow;
            if diff < 0 {
                diff += 60;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result_digits.push(diff as u8);
        }

        if borrow > 0 {
            // Underflow - return 0 or panic?
            // Return 0 for now.
            return Self::zero();
        }

        result_digits.reverse();
        let mut res = Self::new(result_digits);
        res.normalize();
        res
    }
}
