use num_rational::Ratio;
use num_traits::{Zero, One};
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EgyptianFraction {
    /// Distinct denominators, sorted ascending.
    pub parts: Vec<u64>,
}

impl EgyptianFraction {
    pub fn new(parts: Vec<u64>) -> Self {
        let mut p = parts;
        p.sort();
        p.dedup();
        Self { parts: p }
    }

    pub fn to_ratio(&self) -> Ratio<u64> {
        let mut sum = Ratio::zero();
        for &d in &self.parts {
            sum = sum + Ratio::new(1, d);
        }
        sum
    }
}

impl From<Ratio<u64>> for EgyptianFraction {
    fn from(val: Ratio<u64>) -> Self {
        if val <= Ratio::zero() {
            return Self::default();
        }

        let mut parts = Vec::new();
        let mut remaining = val;

        // Limiter to prevent infinite loops for irrational-ish behaviors (though Ratio is rational)
        // or extremely long expansions.
        let mut iterations = 0;
        while !remaining.is_zero() && iterations < 100 {
            let num = *remaining.numer();
            let den = *remaining.denom();

            // Greedy algorithm: n = ceil(den / num)
            let n = (den + num - 1) / num;

            parts.push(n);

            remaining = remaining - Ratio::new(1, n);
            iterations += 1;
        }

        Self { parts }
    }
}

impl Add for EgyptianFraction {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let sum_ratio = self.to_ratio() + other.to_ratio();
        Self::from(sum_ratio)
    }
}

impl Sub for EgyptianFraction {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let a = self.to_ratio();
        let b = other.to_ratio();
        if b >= a {
            return Self::default();
        }
        Self::from(a - b)
    }
}

impl fmt::Display for EgyptianFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.parts.is_empty() {
            return write!(f, "0");
        }
        let terms: Vec<String> = self.parts.iter()
            .map(|d| format!("1/{}", d))
            .collect();
        write!(f, "{}", terms.join(" + "))
    }
}
