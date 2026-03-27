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
                write!(f, "''")?;
                continue;
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
            let a_val = *a_iter.next().unwrap_or(&0) as u16;
            let b_val = *b_iter.next().unwrap_or(&0) as u16;

            let sum = a_val + b_val + carry;
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
