use std::fmt;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use num_bigint::BigUint;
use num_traits::Zero;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseSymbol {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl BaseSymbol {
    pub fn value(&self) -> u64 {
        *self as u64
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'I' => Some(Self::I),
            'V' => Some(Self::V),
            'X' => Some(Self::X),
            'L' => Some(Self::L),
            'C' => Some(Self::C),
            'D' => Some(Self::D),
            'M' => Some(Self::M),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol {
    pub base: BaseSymbol,
    pub vinculum: u32, // Number of bars (x1000 per bar)
}

impl Symbol {
    pub fn new(base: BaseSymbol, vinculum: u32) -> Self {
        Self { base, vinculum }
    }

    pub fn value(&self) -> BigUint {
        let val = BigUint::from(self.base.value());
        let multiplier = BigUint::from(1000u32).pow(self.vinculum);
        val * multiplier
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare vinculum first (higher power dominates)
        match self.vinculum.cmp(&other.vinculum) {
            std::cmp::Ordering::Equal => {
                // Same magnitude, compare base
                self.base.cmp(&other.base)
            }
            ord => ord,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roman {
    pub digits: Vec<Symbol>,
}

impl Roman {
    pub fn new() -> Self {
        Roman { digits: Vec::new() }
    }

    pub fn zero() -> Self {
        Roman { digits: Vec::new() }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn value(&self) -> BigUint {
        let mut sum = BigUint::zero();
        if self.digits.is_empty() {
            return sum;
        }

        let mut prev_val = BigUint::zero();

        for digit in self.digits.iter().rev() {
            let val = digit.value();
            let next_prev = val.clone();
            if val < prev_val {
                sum -= val;
            } else {
                sum += val;
            }
            prev_val = next_prev;
        }
        sum
    }

    pub fn from_u64(mut n: u64) -> Self {
        if n == 0 {
            return Roman::zero();
        }
        let mut digits = Vec::new();
        // Additive-only mappings to ensure sorting (normalize) preserves value
        let mappings = [
            (1000, vec![Symbol::new(BaseSymbol::M, 0)]),
            (500, vec![Symbol::new(BaseSymbol::D, 0)]),
            (100, vec![Symbol::new(BaseSymbol::C, 0)]),
            (50, vec![Symbol::new(BaseSymbol::L, 0)]),
            (10, vec![Symbol::new(BaseSymbol::X, 0)]),
            (5, vec![Symbol::new(BaseSymbol::V, 0)]),
            (1, vec![Symbol::new(BaseSymbol::I, 0)]),
        ];

        for (val, syms) in mappings.iter() {
            while n >= *val {
                digits.extend_from_slice(syms);
                n -= val;
            }
        }
        Roman { digits }
    }

    // Helper to normalize representation (simplistic)
    // Real normalization is hard, but we can just use "additive" form for internal logic if we want,
    // or keep it canonical.
    // For RSA, canonical is best to keep size down.
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Represent vinculum with parentheses or overline logic if possible.
        // Plain text: |V| for 5000? or (V) ?
        // Standard ASCII for Vinculum is often underlines or parentheses.
        // Let's use parentheses: (V) = 5000, ((V)) = 5,000,000
        let s = format!("{:?}", self.base);
        let mut res = s;
        for _ in 0..self.vinculum {
            res = format!("({})", res);
        }
        write!(f, "{}", res)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.digits.is_empty() {
            return write!(f, "NULLA");
        }
        for digit in &self.digits {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

// Basic FromStr implementation (only for standard numerals for now)
impl FromStr for Roman {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "NULLA" {
            return Ok(Roman::zero());
        }
        let mut digits = Vec::new();
        for c in s.chars() {
            if let Some(base) = BaseSymbol::from_char(c) {
                digits.push(Symbol::new(base, 0));
            } else {
                // Handling parentheses for vinculum is hard without a proper parser.
                // For now, let's just support basic chars.
                return Err(anyhow!("Invalid Roman numeral char: {}", c));
            }
        }
        Ok(Roman { digits })
    }
}
