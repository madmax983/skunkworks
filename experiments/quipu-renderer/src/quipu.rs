use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Knot {
    Simple,      // 's' (Val: 1, for tens+)
    Long(u8),    // 'L' (Val: 2-9, for units)
    FigureEight, // 'E' (Val: 1, for units)
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cord {
    // Index 0 = Units (10^0)
    // Index 1 = Tens (10^1)
    // ...
    // Higher indices = higher powers
    pub clusters: Vec<Vec<Knot>>,
}

impl Cord {
    pub fn new() -> Self {
        Self::default()
    }

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

    // Ancient Addition: Combine knots position by position
    pub fn add(&self, other: &Cord) -> Cord {
        let max_len = std::cmp::max(self.clusters.len(), other.clusters.len());
        let mut new_clusters = Vec::with_capacity(max_len + 1);
        let mut carry = 0;

        // Iterate through positions (units, tens, hundreds...)
        for i in 0..max_len {
            let mut pos_sum = carry;

            // Add value from self at this position
            if i < self.clusters.len() {
                for k in &self.clusters[i] {
                    pos_sum += k.value() as u32;
                }
            }
            // Add value from other at this position
            if i < other.clusters.len() {
                for k in &other.clusters[i] {
                    pos_sum += k.value() as u32;
                }
            }

            carry = pos_sum / 10;
            let remainder = (pos_sum % 10) as u8;

            // Reconstruct knots for this position
            let new_knots = if remainder == 0 {
                Vec::new()
            } else if i == 0 {
                // Units position rules
                if remainder == 1 {
                    vec![Knot::FigureEight]
                } else {
                    vec![Knot::Long(remainder)]
                }
            } else {
                // Tens+ position rules: just simple knots
                vec![Knot::Simple; remainder as usize]
            };

            new_clusters.push(new_knots);
        }

        // Handle remaining carry
        while carry > 0 {
            let val = (carry % 10) as u8;
            let new_knots = if val == 0 {
                Vec::new()
            } else {
                // Any new positions are 10^1 or higher, so Simple knots
                vec![Knot::Simple; val as usize]
            };
            new_clusters.push(new_knots);
            carry /= 10;
        }

        Cord {
            clusters: new_clusters,
        }
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
            return Ok(());
        }

        for (i, cluster) in self.clusters.iter().enumerate().rev() {
            // Print knots
            for (j, knot) in cluster.iter().enumerate() {
                match knot {
                    Knot::Simple => write!(f, "s")?,
                    Knot::Long(v) => write!(f, "L{}", v)?,
                    Knot::FigureEight => write!(f, "E")?,
                }
                if j < cluster.len() - 1 {
                    writeln!(f)?;
                }
            }

            // Print separator if not the last position (units)
            if i > 0 {
                if !cluster.is_empty() {
                    writeln!(f)?;
                }
                write!(f, "|")?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Cord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        // parts are high to low.
        // We want low to high for clusters.

        let mut clusters = Vec::new();

        for part in parts.iter().rev() {
            let mut knots = Vec::new();
            for line in part.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if trimmed == "s" {
                    knots.push(Knot::Simple);
                } else if trimmed == "E" {
                    knots.push(Knot::FigureEight);
                } else if trimmed.starts_with("L") {
                    let val_str = &trimmed[1..];
                    let val = val_str
                        .parse::<u8>()
                        .map_err(|_| "Invalid Long knot value")?;
                    knots.push(Knot::Long(val));
                } else {
                    return Err(format!("Unknown knot: {}", trimmed));
                }
            }
            clusters.push(knots);
        }

        Ok(Cord { clusters })
    }
}
