use anyhow::{anyhow, Result};
use num_bigint::BigUint;
use num_traits::Zero;
use std::fmt;
use std::str::FromStr;

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

    pub fn is_five_unit(&self) -> bool {
        matches!(self, Self::V | Self::L | Self::D)
    }

    pub fn next_magnitude(&self) -> Option<Self> {
        match self {
            Self::I => Some(Self::V),
            Self::V => Some(Self::X),
            Self::X => Some(Self::L),
            Self::L => Some(Self::C),
            Self::C => Some(Self::D),
            Self::D => Some(Self::M),
            Self::M => None,
        }
    }
}

impl TryFrom<char> for BaseSymbol {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'I' => Ok(Self::I),
            'V' => Ok(Self::V),
            'X' => Ok(Self::X),
            'L' => Ok(Self::L),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'M' => Ok(Self::M),
            _ => Err(anyhow!("Invalid Roman numeral char: {}", c)),
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

impl Default for Roman {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn from_biguint(mut n: BigUint) -> Self {
        if n.is_zero() {
            return Roman::zero();
        }
        let mut digits = Vec::new();

        // Strategy: Decompose into 1000^N chunks.
        // N=0: I..M
        // N=1: (I)..(M)
        // etc.

        let thousand = BigUint::from(1000u32);
        let mut vinculum = 0;

        while !n.is_zero() {
            let chunk = (&n % &thousand).try_into().unwrap_or(0u64);
            n /= &thousand;

            if chunk > 0 {
                let chunk_digits = Self::from_u64_chunk(chunk, vinculum);
                // Prepend because we want higher powers at the start (but we process chunks from low to high)
                // Actually, standard Roman is big-endian.
                // So if we process small chunks first, we should insert at 0.
                let mut new_digits = chunk_digits.digits;
                new_digits.append(&mut digits);
                digits = new_digits;
            }
            vinculum += 1;
        }

        Roman { digits }
    }

    fn from_u64_chunk(mut n: u64, vinculum: u32) -> Self {
        let mut digits = Vec::new();
        // Basic conversion for 0..999
        const MAPPINGS: [(u64, BaseSymbol); 7] = [
            (1000, BaseSymbol::M), // Shouldn't be reached in chunk < 1000 except maybe at boundary?
            (500, BaseSymbol::D),
            (100, BaseSymbol::C),
            (50, BaseSymbol::L),
            (10, BaseSymbol::X),
            (5, BaseSymbol::V),
            (1, BaseSymbol::I),
        ];

        // Simplistic additive for now to avoid subtraction logic complexity in generic form
        // But let's try to do standard subtractive for readability?
        // Actually, simple additive is fine for hybrid experimentation.
        // Or better: The standard logic for 1-1000.

        // Let's implement subtractive for the chunk level
        let levels = [
            (1000, BaseSymbol::M, BaseSymbol::M, BaseSymbol::M), // M is limit
            (900, BaseSymbol::C, BaseSymbol::M, BaseSymbol::M),
            (500, BaseSymbol::D, BaseSymbol::D, BaseSymbol::D),
            (400, BaseSymbol::C, BaseSymbol::D, BaseSymbol::D),
            (100, BaseSymbol::C, BaseSymbol::C, BaseSymbol::C),
            (90, BaseSymbol::X, BaseSymbol::C, BaseSymbol::C),
            (50, BaseSymbol::L, BaseSymbol::L, BaseSymbol::L),
            (40, BaseSymbol::X, BaseSymbol::L, BaseSymbol::L),
            (10, BaseSymbol::X, BaseSymbol::X, BaseSymbol::X),
            (9, BaseSymbol::I, BaseSymbol::X, BaseSymbol::X),
            (5, BaseSymbol::V, BaseSymbol::V, BaseSymbol::V),
            (4, BaseSymbol::I, BaseSymbol::V, BaseSymbol::V),
            (1, BaseSymbol::I, BaseSymbol::I, BaseSymbol::I),
        ];

        // This is tricky with BaseSymbol enum which doesn't support 'IV' as a unit.
        // My Symbol struct is additive (Symbol is one char).
        // Subtractive notation (IV) is represented as Symbol(I), Symbol(V).
        // And my value() logic handles subtraction if previous < current.

        for (val, base, _mid, _high) in levels {
            while n >= val {
                // Check subtractives
                if val == 900 {
                    digits.push(Symbol::new(BaseSymbol::C, vinculum));
                    digits.push(Symbol::new(BaseSymbol::M, vinculum));
                } else if val == 400 {
                    digits.push(Symbol::new(BaseSymbol::C, vinculum));
                    digits.push(Symbol::new(BaseSymbol::D, vinculum));
                } else if val == 90 {
                    digits.push(Symbol::new(BaseSymbol::X, vinculum));
                    digits.push(Symbol::new(BaseSymbol::C, vinculum));
                } else if val == 40 {
                    digits.push(Symbol::new(BaseSymbol::X, vinculum));
                    digits.push(Symbol::new(BaseSymbol::L, vinculum));
                } else if val == 9 {
                    digits.push(Symbol::new(BaseSymbol::I, vinculum));
                    digits.push(Symbol::new(BaseSymbol::X, vinculum));
                } else if val == 4 {
                    digits.push(Symbol::new(BaseSymbol::I, vinculum));
                    digits.push(Symbol::new(BaseSymbol::V, vinculum));
                } else {
                    digits.push(Symbol::new(base, vinculum));
                }
                n -= val;
            }
        }

        Roman { digits }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Represent vinculum with parentheses or overline logic if possible.
        // Plain text: |V| for 5000? or (V) ?
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

impl FromStr for Roman {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "NULLA" {
            return Ok(Roman::zero());
        }
        let mut digits = Vec::new();
        // Naive parsing: doesn't handle parenthesis for vinculum yet.
        // Assuming standard input for now.
        for c in s.chars() {
            if c == '(' || c == ')' {
                continue;
            } // Skip vinculum markers for naive parse
            let base = BaseSymbol::try_from(c)?;
            digits.push(Symbol::new(base, 0));
        }
        Ok(Roman { digits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_conversion() {
        let r = Roman::from_u64_chunk(42, 0);
        assert_eq!(format!("{}", r), "XLII");

        let r = Roman::from_u64_chunk(1984, 0);
        assert_eq!(format!("{}", r), "MCMLXXXIV");
    }

    #[test]
    fn test_large_roman() {
        // 5000 = (V)
        let val = BigUint::from(5000u32);
        let r = Roman::from_biguint(val);
        // My implementation pushes (V) then nothing else.
        assert_eq!(format!("{}", r), "(V)");
    }
}
