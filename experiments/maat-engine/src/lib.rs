use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::{One, ToPrimitive, Zero};
use std::fmt;

/// A Soul represents a process or request for resources.
/// Its `demand` is a Rational number (e.g., 3/4 of the CPU).
#[derive(Debug, Clone)]
pub struct Soul {
    pub id: u64,
    pub demand: Ratio<BigUint>,
}

impl Soul {
    pub fn new(id: u64, numer: u64, denom: u64) -> Self {
        Self {
            id,
            demand: Ratio::new(BigUint::from(numer), BigUint::from(denom)),
        }
    }
}

/// An Egyptian Fraction is a sum of distinct unit fractions (1/n).
/// We store only the denominators.
#[derive(Debug, Clone, PartialEq)]
pub struct EgyptianFraction {
    pub denominators: Vec<BigUint>,
}

impl fmt::Display for EgyptianFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms: Vec<String> = self
            .denominators
            .iter()
            .map(|d| format!("1/{}", to_hieroglyphs(d)))
            .collect();
        write!(f, "{}", terms.join(" + "))
    }
}

/// Converts a number to Egyptian Hieroglyphs.
/// 1 = 𓏤, 10 = 𓎆, 100 = 𓍢, 1000 = 𓆼, 10000 = 𓂭, 100000 = 𓆐, 1000000 = 𓁨
pub fn to_hieroglyphs(n: &BigUint) -> String {
    let mut s = String::new();
    let mut val = n.clone();

    // Ordered from largest to smallest for correct representation
    let mapping = [
        (1000000u64, '𓁨'),
        (100000u64, '𓆐'),
        (10000u64, '𓂭'),
        (1000u64, '𓆼'),
        (100u64, '𓍢'),
        (10u64, '𓎆'),
        (1u64, '𓏤'),
    ];

    if val.is_zero() {
        return "𓄤".to_string(); // Nefer (Zero/Beautiful/Complete)
    }

    for (limit, glyph) in mapping {
        let l_big = BigUint::from(limit);
        while val >= l_big {
            s.push(glyph);
            val -= &l_big;
        }
    }

    s
}

/// The Scales of Maat represent the resource allocator.
/// The `timeline` is a fixed-size array of slots (e.g., 3600 ticks).
pub struct ScalesOfMaat {
    pub timeline: Vec<Option<u64>>, // None = Empty, Some(id) = Occupied
    pub capacity: usize,
}

impl ScalesOfMaat {
    pub fn new(capacity: usize) -> Self {
        Self {
            timeline: vec![None; capacity],
            capacity,
        }
    }

    pub fn weigh_heart(&mut self, soul: &Soul) -> Result<Vec<(usize, usize)>, String> {
        let decomposition = EgyptianFraction::from(soul.demand.clone());
        let mut temp_timeline = self.timeline.clone();
        let mut allocations = Vec::new();

        for denom in &decomposition.denominators {
            let d_usize = denom.to_usize().unwrap_or(usize::MAX);
            if d_usize == 0 {
                continue;
            }

            // Ancient Strictness: Allocation must be exact unit fraction of total capacity.
            let size = self.capacity / d_usize;
            if size == 0 {
                return Err(format!(
                    "Demand 1/{} is too small for the Scales.",
                    to_hieroglyphs(denom)
                ));
            }

            if let Some(start) = Self::allocate_block(&mut temp_timeline, size, soul.id) {
                allocations.push((start, size));
            } else {
                return Err(format!(
                    "The Scales cannot balance 1/{}.",
                    to_hieroglyphs(denom)
                ));
            }
        }

        self.timeline = temp_timeline;
        Ok(allocations)
    }

    /// ⚡ Bolt: Prefer slice `&mut [T]` over `&mut Vec<T>`.
    /// This avoids unnecessary dereferencing constraints and is more flexible for callers.
    fn allocate_block(timeline: &mut [Option<u64>], size: usize, id: u64) -> Option<usize> {
        let mut run_start = 0;
        let mut run_len = 0;

        for (i, slot) in timeline.iter().enumerate() {
            if slot.is_none() {
                if run_len == 0 {
                    run_start = i;
                }
                run_len += 1;
                if run_len == size {
                    for slot in timeline.iter_mut().skip(run_start).take(size) {
                        *slot = Some(id);
                    }
                    return Some(run_start);
                }
            } else {
                run_len = 0;
            }
        }
        None
    }
}

impl From<Ratio<BigUint>> for EgyptianFraction {
    fn from(ratio: Ratio<BigUint>) -> Self {
        let mut numer = ratio.numer().clone();
        let mut denom = ratio.denom().clone();
        let mut result = Vec::new();

        if numer.is_zero() {
            return EgyptianFraction {
                denominators: vec![],
            };
        }

        // Greedy Algorithm (Fibonacci-Sylvester)
        while !numer.is_zero() {
            // ceil(y/x) = (y + x - 1) / x
            let unit_denom = (&denom + &numer - BigUint::one()) / &numer;
            result.push(unit_denom.clone());

            // new_numer = numer * unit_denom - denom
            let new_numer = &numer * &unit_denom - &denom;
            let new_denom = &denom * &unit_denom;

            let common = gcd(new_numer.clone(), new_denom.clone());
            numer = new_numer / &common;
            denom = new_denom / &common;
        }

        EgyptianFraction {
            denominators: result,
        }
    }
}

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    let mut a = a;
    let mut b = b;
    while !b.is_zero() {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decomposition_5_21() {
        let ratio = Ratio::new(BigUint::from(5u64), BigUint::from(21u64));
        let ef = EgyptianFraction::from(ratio);
        let denoms: Vec<u64> = ef
            .denominators
            .iter()
            .map(|d| d.to_u64().unwrap())
            .collect();
        assert_eq!(denoms, vec![5, 27, 945]);
    }

    #[test]
    fn test_decomposition_3_4() {
        let ratio = Ratio::new(BigUint::from(3u64), BigUint::from(4u64));
        let ef = EgyptianFraction::from(ratio);
        let denoms: Vec<u64> = ef
            .denominators
            .iter()
            .map(|d| d.to_u64().unwrap())
            .collect();
        assert_eq!(denoms, vec![2, 4]);
    }

    #[test]
    fn test_allocation_success() {
        let mut scales = ScalesOfMaat::new(100);
        let soul = Soul::new(1, 3, 4); // 3/4 = 1/2 + 1/4
                                       // 1/2 of 100 = 50. 1/4 of 100 = 25.
                                       // Should allocate [0..50] and [50..75].
        assert!(scales.weigh_heart(&soul).is_ok());

        // Next 50..75 is used. 75..100 free.
        // Try allocating 1/5 = 20.
        // Should fit in 75..95.
        let soul2 = Soul::new(2, 1, 5);
        assert!(scales.weigh_heart(&soul2).is_ok());
    }

    #[test]
    fn test_allocation_failure() {
        let mut scales = ScalesOfMaat::new(100);
        let soul = Soul::new(1, 3, 4);
        assert!(scales.weigh_heart(&soul).is_ok()); // uses 75 slots

        let soul2 = Soul::new(2, 1, 2); // needs 50 slots. Only 25 left.
        assert!(scales.weigh_heart(&soul2).is_err());
    }

    #[test]
    fn test_hieroglyphs() {
        assert_eq!(to_hieroglyphs(&BigUint::from(1u64)), "𓏤");
        assert_eq!(to_hieroglyphs(&BigUint::from(12u64)), "𓎆𓏤𓏤"); // 10 + 1 + 1
        assert_eq!(to_hieroglyphs(&BigUint::from(105u64)), "𓍢𓏤𓏤𓏤𓏤𓏤"); // 100 + 5
    }
}
