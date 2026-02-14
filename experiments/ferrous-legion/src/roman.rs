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

    #[deprecated(since = "0.1.0", note = "Use TryFrom<char> instead")]
    pub fn from_char(c: char) -> Option<Self> {
        Self::try_from(c).ok()
    }
}

impl fmt::Display for BaseSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
            _ => Err(anyhow!("Invalid base symbol: {}", c)),
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
        // Compare vinculum first (higher power dominates), then base
        (self.vinculum, self.base).cmp(&(other.vinculum, other.base))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roman {
    pub digits: Vec<Symbol>,
}

impl Roman {
    #[allow(dead_code)]
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

        const MAPPINGS: [(u64, BaseSymbol); 7] = [
            (1000, BaseSymbol::M),
            (500, BaseSymbol::D),
            (100, BaseSymbol::C),
            (50, BaseSymbol::L),
            (10, BaseSymbol::X),
            (5, BaseSymbol::V),
            (1, BaseSymbol::I),
        ];

        let mut digits = Vec::new();
        for (val, base) in MAPPINGS {
            while n >= val {
                digits.push(Symbol::new(base, 0));
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
        let s = self.base.to_string();
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
            // Handling parentheses for vinculum is hard without a proper parser.
            // For now, let's just support basic chars.
            let base = BaseSymbol::try_from(c)
                .map_err(|_| anyhow!("Invalid Roman numeral char: {}", c))?;
            digits.push(Symbol::new(base, 0));
        }
        Ok(Roman { digits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_base_symbol_values() {
        assert_eq!(BaseSymbol::I.value(), 1);
        assert_eq!(BaseSymbol::V.value(), 5);
        assert_eq!(BaseSymbol::X.value(), 10);
        assert_eq!(BaseSymbol::L.value(), 50);
        assert_eq!(BaseSymbol::C.value(), 100);
        assert_eq!(BaseSymbol::D.value(), 500);
        assert_eq!(BaseSymbol::M.value(), 1000);
    }

    #[test]
    fn test_base_symbol_try_from() {
        assert_eq!(BaseSymbol::try_from('I').ok(), Some(BaseSymbol::I));
        assert_eq!(BaseSymbol::try_from('V').ok(), Some(BaseSymbol::V));
        assert_eq!(BaseSymbol::try_from('X').ok(), Some(BaseSymbol::X));
        assert_eq!(BaseSymbol::try_from('L').ok(), Some(BaseSymbol::L));
        assert_eq!(BaseSymbol::try_from('C').ok(), Some(BaseSymbol::C));
        assert_eq!(BaseSymbol::try_from('D').ok(), Some(BaseSymbol::D));
        assert_eq!(BaseSymbol::try_from('M').ok(), Some(BaseSymbol::M));
        assert!(BaseSymbol::try_from('A').is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn test_from_char_deprecated() {
        assert_eq!(BaseSymbol::from_char('I'), Some(BaseSymbol::I));
        assert_eq!(BaseSymbol::from_char('A'), None);
    }

    #[test]
    fn test_symbol_ordering() {
        let s1 = Symbol::new(BaseSymbol::I, 0);
        let s2 = Symbol::new(BaseSymbol::V, 0);
        let s3 = Symbol::new(BaseSymbol::I, 1); // Vinculum 1 (x1000) -> 1000

        // Base value check
        assert!(s1 < s2);

        // Vinculum check: I with vinculum 1 (1000) > V (5)
        assert!(s3 > s2);

        // Same vinculum check
        let s4 = Symbol::new(BaseSymbol::V, 1);
        assert!(s4 > s3);
    }

    #[test]
    fn test_symbol_value() {
        let s = Symbol::new(BaseSymbol::V, 2); // 5 * 1000^2 = 5,000,000
        assert_eq!(s.value(), BigUint::from(5_000_000u32));
    }

    #[test]
    fn test_roman_from_u64() {
        // Standard additive cases
        let r1 = Roman::from_u64(1);
        assert_eq!(r1.to_string(), "I");

        let r2 = Roman::from_u64(3);
        assert_eq!(r2.to_string(), "III");

        let r3 = Roman::from_u64(4);
        // Note: Implementation is additive-only (IIII), not subtractive (IV)
        assert_eq!(r3.to_string(), "IIII");

        let r4 = Roman::from_u64(5);
        assert_eq!(r4.to_string(), "V");

        let r5 = Roman::from_u64(6);
        assert_eq!(r5.to_string(), "VI");

        let r6 = Roman::from_u64(9);
        // Note: Implementation is additive-only (VIIII), not subtractive (IX)
        assert_eq!(r6.to_string(), "VIIII");

        let r7 = Roman::from_u64(10);
        assert_eq!(r7.to_string(), "X");

        let r8 = Roman::from_u64(1666);
        // MDCLXVI
        assert_eq!(r8.to_string(), "MDCLXVI");
    }

    #[test]
    fn test_roman_value() {
        // Test value calculation from digits
        // Subtractive logic (IV = 4) IS implemented in value(), but from_u64 generates additive forms.
        // So we test manual construction.

        // IV = 4
        let iv = Roman {
            digits: vec![Symbol::new(BaseSymbol::I, 0), Symbol::new(BaseSymbol::V, 0)],
        };
        assert_eq!(iv.value(), BigUint::from(4u32));

        // IIII = 4 (Additive form generated by from_u64)
        let iiii = Roman::from_u64(4);
        assert_eq!(iiii.value(), BigUint::from(4u32));

        // IX = 9
        let ix = Roman {
            digits: vec![Symbol::new(BaseSymbol::I, 0), Symbol::new(BaseSymbol::X, 0)],
        };
        assert_eq!(ix.value(), BigUint::from(9u32));

        // MCMLIV = 1954
        let mcmliv = Roman {
            digits: vec![
                Symbol::new(BaseSymbol::M, 0),
                Symbol::new(BaseSymbol::C, 0),
                Symbol::new(BaseSymbol::M, 0),
                Symbol::new(BaseSymbol::L, 0),
                Symbol::new(BaseSymbol::I, 0),
                Symbol::new(BaseSymbol::V, 0),
            ],
        };
        assert_eq!(mcmliv.value(), BigUint::from(1954u32));
    }

    #[test]
    fn test_roman_from_str() {
        let r = Roman::from_str("MCMLIV").unwrap();
        assert_eq!(r.value(), BigUint::from(1954u32));

        let r_err = Roman::from_str("A");
        assert!(r_err.is_err());
    }
}
