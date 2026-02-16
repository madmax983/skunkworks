use anyhow::{Result, anyhow};
use num_rational::Ratio;
use num_traits::{Zero, One};
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EgyptianFraction {
    /// Distinct denominators, sorted ascending.
    pub parts: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: usize,
    pub demand: Ratio<u64>,
    pub allocated: EgyptianFraction,
}

#[derive(Debug)]
pub struct Scheduler {
    pub total: Ratio<u64>,
    pub allocated: Vec<Process>,
    pub free_space: EgyptianFraction,
    next_id: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            total: Ratio::one(),
            allocated: Vec::new(),
            free_space: EgyptianFraction::from(Ratio::one()),
            next_id: 1,
        }
    }

    pub fn allocate(&mut self, demand: Ratio<u64>) -> Result<EgyptianFraction> {
        let available = self.free_space.to_ratio();
        if demand > available {
            return Err(anyhow!(
                "Not enough resources. Demand: {}, Available: {}",
                demand,
                available
            ));
        }

        // Optimize: Update free space in Ratio domain first to avoid redundant conversions
        let new_free_ratio = available - demand;
        self.free_space = EgyptianFraction::from(new_free_ratio);

        // Convert demand to Egyptian Fraction for the process record
        let allocated_fraction = EgyptianFraction::from(demand);

        let process = Process {
            id: self.next_id,
            demand,
            allocated: allocated_fraction.clone(),
        };
        self.next_id += 1;
        self.allocated.push(process);

        Ok(allocated_fraction)
    }
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

        while !remaining.is_zero() {
            let num = *remaining.numer();
            let den = *remaining.denom();

            // Greedy algorithm: n = ceil(den / num)
            let n = (den + num - 1) / num;

            parts.push(n);

            remaining = remaining - Ratio::new(1, n);
        }

        // Greedy algorithm naturally produces sorted and distinct denominators.
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
