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

    /// Returns the symbol for TUI display (from quipu-symphony)
    pub fn symbol(&self) -> String {
        match self {
            Knot::Simple => "●".to_string(),
            Knot::Long(v) => format!("≡{}", v),
            Knot::FigureEight => "∞".to_string(),
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

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if underflow occurred.
    ///
    /// This is safer than the `Sub` implementation which panics on underflow (since Quipus
    /// cannot represent negative numbers).
    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let v1 = self.value();
        let v2 = rhs.value();
        if v1 < v2 {
            None
        } else {
            Some(Cord::from(v1 - v2))
        }
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
        let val = self.value() + rhs.value();
        Cord::from(val)
    }
}

impl Sub for Cord {
    type Output = Cord;

    fn sub(self, rhs: Self) -> Self::Output {
        // Only implementing positive result subtraction
        if self.value() < rhs.value() {
            // In a real library we might want to return Result or panic,
            // but for now panic fits the original behavior.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_values() {
        assert_eq!(Knot::Simple.value(), 1);
        assert_eq!(Knot::FigureEight.value(), 1);
        assert_eq!(Knot::Long(5).value(), 5);
    }

    #[test]
    fn test_cord_from_u64() {
        let val = 123;
        let cord = Cord::from(val);
        // 123 -> Units: 3, Tens: 2, Hundreds: 1
        // clusters[0]: 3 units -> Long(3)
        // clusters[1]: 2 tens -> 2 Simple
        // clusters[2]: 1 hundred -> 1 Simple

        assert_eq!(cord.clusters.len(), 3);

        // Units
        assert_eq!(cord.clusters[0].len(), 1);
        assert_eq!(cord.clusters[0][0], Knot::Long(3));

        // Tens
        assert_eq!(cord.clusters[1].len(), 2);
        assert_eq!(cord.clusters[1][0], Knot::Simple);

        // Hundreds
        assert_eq!(cord.clusters[2].len(), 1);
        assert_eq!(cord.clusters[2][0], Knot::Simple);
    }

    #[test]
    fn test_cord_value() {
        let cord = Cord::from(456);
        assert_eq!(cord.value(), 456);
    }

    #[test]
    fn test_cord_add() {
        let c1 = Cord::from(100);
        let c2 = Cord::from(25);
        let sum = c1 + c2;
        assert_eq!(sum.value(), 125);
    }

    #[test]
    fn test_cord_sub() {
        let c1 = Cord::from(100);
        let c2 = Cord::from(25);
        let diff = c1 - c2;
        assert_eq!(diff.value(), 75);
    }

    #[test]
    fn test_checked_sub() {
        let c1 = Cord::from(50);
        let c2 = Cord::from(20);
        let c3 = Cord::from(60);

        assert_eq!(c1.checked_sub(&c2).unwrap().value(), 30);
        assert!(c1.checked_sub(&c3).is_none());
    }
}
