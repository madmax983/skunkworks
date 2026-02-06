use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Knot {
    Simple,       // Value 1 (Tens+)
    Long(u8),     // Value 2-9 (Units)
    FigureEight,  // Value 1 (Units)
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

#[derive(Debug, Clone, Default)]
pub struct Cord {
    // Index 0 = Units (10^0), Index 1 = Tens (10^1), etc.
    pub clusters: Vec<Vec<Knot>>,
    pub subsidiaries: Vec<Cord>,
}

impl Cord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_u64(mut val: u64) -> Self {
        if val == 0 {
            // Empty cord implies 0? Or maybe explicit empty position?
            // Usually, 0 is represented by no knot in that position.
            // If the whole number is 0, the cord is empty.
            return Cord::default();
        }

        let mut clusters = Vec::new();
        let mut pos = 0; // 0 = units

        while val > 0 {
            let digit = (val % 10) as u8;
            val /= 10;

            let knots = if digit == 0 {
                Vec::new()
            } else if pos == 0 {
                // Units position rules
                if digit == 1 {
                    vec![Knot::FigureEight]
                } else {
                    vec![Knot::Long(digit)]
                }
            } else {
                // Tens+ position rules
                vec![Knot::Simple; digit as usize]
            };

            clusters.push(knots);
            pos += 1;
        }

        Cord {
            clusters,
            subsidiaries: Vec::new(),
        }
    }

    pub fn value(&self) -> u64 {
        let mut total = 0;
        let mut multiplier = 1;
        for cluster in &self.clusters {
            let mut cluster_val = 0;
            for knot in cluster {
                cluster_val += knot.value() as u64;
            }
            total += cluster_val * multiplier;
            multiplier *= 10;
        }
        total
    }
}

impl fmt::Display for Cord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, 0)
    }
}

impl Cord {
    fn fmt_recursive(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let padding = " ".repeat(indent * 4);

        if self.clusters.is_empty() {
             writeln!(f, "{}| (Empty/0)", padding)?;
        } else {
            for (_i, cluster) in self.clusters.iter().enumerate().rev() {
                write!(f, "{}| ", padding)?;
                if cluster.is_empty() {
                    writeln!(f, " ")?;
                } else {
                    for knot in cluster {
                        match knot {
                            Knot::Simple => write!(f, "●")?,
                            Knot::Long(v) => write!(f, "≡({})", v)?,
                            Knot::FigureEight => write!(f, "∞")?,
                        }
                    }
                    writeln!(f)?;
                }
            }
        }

        if !self.subsidiaries.is_empty() {
             writeln!(f, "{}|-- Subsidiaries: {}", padding, self.subsidiaries.len())?;
             for sub in &self.subsidiaries {
                 sub.fmt_recursive(f, indent + 1)?;
             }
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
        assert_eq!(Knot::Long(3).value(), 3);
        assert_eq!(Knot::FigureEight.value(), 1);
    }

    #[test]
    fn test_cord_from_u64() {
        // 123 -> 100 + 20 + 3
        let cord = Cord::from_u64(123);
        assert_eq!(cord.value(), 123);
        assert_eq!(cord.clusters.len(), 3); // 1, 10, 100 positions

        // Check units (Index 0)
        assert_eq!(cord.clusters[0].len(), 1);
        match cord.clusters[0][0] {
            Knot::Long(3) => {},
            _ => panic!("Expected Long(3) in units"),
        }

        // Check tens (Index 1)
        assert_eq!(cord.clusters[1].len(), 2);
        assert_eq!(cord.clusters[1][0], Knot::Simple);

        // Check hundreds (Index 2)
        assert_eq!(cord.clusters[2].len(), 1);
        assert_eq!(cord.clusters[2][0], Knot::Simple);
    }

    #[test]
    fn test_cord_zero() {
        let cord = Cord::from_u64(0);
        assert_eq!(cord.value(), 0);
        assert!(cord.clusters.is_empty());
    }

    #[test]
    fn test_cord_one() {
        let cord = Cord::from_u64(1);
        assert_eq!(cord.value(), 1);
        // Units: FigureEight
        assert_eq!(cord.clusters.len(), 1);
        assert_eq!(cord.clusters[0][0], Knot::FigureEight);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Quipu {
    pub pendants: Vec<Cord>,
}

impl Quipu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pendant(&mut self, cord: Cord) {
        self.pendants.push(cord);
    }
}

impl fmt::Display for Quipu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "========================================")?; // Primary Cord
        for (i, cord) in self.pendants.iter().enumerate() {
            writeln!(f, "Cord {}", i)?;
            write!(f, "{}", cord)?;
        }
        writeln!(f, "========================================")?;
        Ok(())
    }
}
