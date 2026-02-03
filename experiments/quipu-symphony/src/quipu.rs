#![allow(dead_code, unused_imports)]
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Knot {
    Simple,      // '●' (Val: 1, for tens+)
    Long(u8),    // '≡' (Val: 2-9, for units)
    FigureEight, // '∞' (Val: 1, for units)
}

impl Knot {
    /// Returns the numerical value of a single knot.
    pub fn value(&self) -> u8 {
        match self {
            Knot::Simple => 1,
            Knot::Long(v) => *v,
            Knot::FigureEight => 1,
        }
    }

    /// Returns the symbol for TUI display
    pub fn symbol(&self) -> String {
        match self {
            Knot::Simple => "●".to_string(),
            Knot::Long(v) => format!("≡{}", v),
            Knot::FigureEight => "∞".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cord {
    // Index 0 = Units (10^0) (Bottom of cord)
    // Index 1 = Tens (10^1)
    // ...
    // Higher indices = higher powers (Top of cord)
    // Note: Physically, quipus hang down. High powers are at top, units at bottom.
    // So Index N is Top, Index 0 is Bottom.
    pub clusters: Vec<Vec<Knot>>,
}

impl Cord {
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper to convert to modern u32
    pub fn value(&self) -> u32 {
        let mut total = 0;
        let mut multiplier = 1;

        for cluster in &self.clusters {
            let mut cluster_val = 0;
            for knot in cluster {
                cluster_val += knot.value() as u32;
            }
            total += cluster_val * multiplier;
            multiplier *= 10;
        }
        total
    }
}

impl From<u32> for Cord {
    fn from(mut val: u32) -> Self {
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
            write!(f, "Empty")?;
            return Ok(());
        }

        // Display from top (highest power) to bottom (units)
        // clusters[0] is units.
        for (i, cluster) in self.clusters.iter().enumerate().rev() {
            // Print knots
            if cluster.is_empty() {
                // Empty space on the cord
                write!(f, "     ")?;
            } else {
                for (j, knot) in cluster.iter().enumerate() {
                    match knot {
                        Knot::Simple => write!(f, "●")?,
                        Knot::Long(v) => write!(f, "≡{}", v)?,
                        Knot::FigureEight => write!(f, "∞")?,
                    }
                    if j < cluster.len() - 1 {
                        // Spacing between knots in same cluster
                        write!(f, " ")?;
                    }
                }
            }

            // Print separator if not the last position (units)
            if i > 0 {
                write!(f, "\n  |  \n")?;
            }
        }
        Ok(())
    }
}
