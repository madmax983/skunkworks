use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Knot {
    Simple,      // Value: 1 (Used for Tens+)
    Long(u8),    // Value: 2-9 (Used for Units)
    FigureEight, // Value: 1 (Used for Units)
}

impl Knot {
    pub fn value(&self) -> u8 {
        match self {
            Knot::Simple => 1,
            Knot::Long(v) => *v,
            Knot::FigureEight => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cord {
    // Index 0 = Units (10^0)
    // Index 1 = Tens (10^1)
    // ...
    pub clusters: Vec<Vec<Knot>>,
}

impl Cord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(&self) -> u64 {
        let mut total: u64 = 0;
        let mut multiplier: u64 = 1;

        for cluster in &self.clusters {
            let mut cluster_val: u64 = 0;
            for knot in cluster {
                cluster_val += knot.value() as u64;
            }
            total += cluster_val * multiplier;
            multiplier *= 10;
        }
        total
    }
}

impl From<u64> for Cord {
    fn from(mut val: u64) -> Self {
        if val == 0 {
            return Cord::default();
        }

        let mut clusters = Vec::new();
        let mut pos = 0;

        while val > 0 {
            let digit = (val % 10) as u8;
            val /= 10;

            let knots = if digit == 0 {
                Vec::new()
            } else if pos == 0 {
                // Units
                if digit == 1 {
                    vec![Knot::FigureEight]
                } else {
                    vec![Knot::Long(digit)]
                }
            } else {
                // Tens+
                vec![Knot::Simple; digit as usize]
            };

            clusters.push(knots);
            pos += 1;
        }

        Cord { clusters }
    }
}

impl fmt::Display for Cord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.clusters.is_empty() {
            return write!(f, "(empty)");
        }
        // Display from Top (highest power) to Bottom (units)
        // clusters[0] is units.
        for (i, cluster) in self.clusters.iter().enumerate().rev() {
            if cluster.is_empty() {
                write!(f, "  |  ")?;
            } else {
                for (j, knot) in cluster.iter().enumerate() {
                    match knot {
                        Knot::Simple => write!(f, "●")?,
                        Knot::Long(v) => write!(f, "≡{}", v)?,
                        Knot::FigureEight => write!(f, "∞")?,
                    }
                    if j < cluster.len() - 1 {
                        write!(f, " ")?;
                    }
                }
            }
            if i > 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Add for Cord {
    type Output = Cord;

    fn add(self, rhs: Self) -> Self::Output {
        let max_len = std::cmp::max(self.clusters.len(), rhs.clusters.len());
        let mut result_clusters = Vec::new();
        let mut carry = 0;

        for i in 0..=max_len { // Go one past max to handle final carry
            if i == max_len && carry == 0 {
                break;
            }

            let mut sum = carry;

            // Add self knots
            if i < self.clusters.len() {
                for k in &self.clusters[i] {
                    sum += k.value() as u64;
                }
            }

            // Add rhs knots
            if i < rhs.clusters.len() {
                for k in &rhs.clusters[i] {
                    sum += k.value() as u64;
                }
            }

            // Determine new digit and carry
            let digit = (sum % 10) as u8;
            carry = sum / 10;

            // Create knots for digit
            let knots = if digit == 0 {
                Vec::new()
            } else if i == 0 {
                // Units logic
                if digit == 1 {
                    vec![Knot::FigureEight]
                } else {
                    vec![Knot::Long(digit)]
                }
            } else {
                // Tens+ logic
                vec![Knot::Simple; digit as usize]
            };

            result_clusters.push(knots);
        }

        Cord { clusters: result_clusters }
    }
}

impl Sub for Cord {
    type Output = Cord;

    fn sub(self, rhs: Self) -> Self::Output {
        // Only implementing positive result subtraction
        if self.value() < rhs.value() {
            panic!("Quipu subtraction resulted in negative value (not supported by Incas!)");
        }

        let val = self.value() - rhs.value();
        Cord::from(val)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Quipu {
    pub cords: Vec<Cord>,
}

impl fmt::Display for Quipu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Quipu with {} cords:\n", self.cords.len())?;
        for (i, cord) in self.cords.iter().enumerate() {
            write!(f, "Cord {}:\n{}\n", i, cord)?;
        }
        Ok(())
    }
}
