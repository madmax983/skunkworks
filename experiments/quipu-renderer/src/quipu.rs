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
    /// Used for local reduction, not for converting the whole cord.
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
            Knot::Long(v) => format!("≡({})", v), // Or just a stack of lines if we get fancy
            Knot::FigureEight => "∞".to_string(),
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

    /// Ancient Addition: Combine knots position by position, carrying over excess.
    /// This implementation does NOT convert the entire Cord to a number.
    /// It operates structurally on the knots.
    pub fn add(&self, other: &Cord) -> Cord {
        let max_len = std::cmp::max(self.clusters.len(), other.clusters.len());
        let mut new_clusters = Vec::with_capacity(max_len + 1);

        // We accumulate 'carry' knots (Simple knots) to add to the next position.
        let mut carry_knots: Vec<Knot> = Vec::new();

        for i in 0..max_len {
            // 1. Gather all knots for this position
            let mut current_knots: Vec<Knot> = Vec::new();

            // From self
            if i < self.clusters.len() {
                current_knots.extend_from_slice(&self.clusters[i]);
            }
            // From other
            if i < other.clusters.len() {
                current_knots.extend_from_slice(&other.clusters[i]);
            }
            // From carry
            current_knots.append(&mut carry_knots); // These are Simple knots from previous level

            // 2. Calculate total magnitude in this position to normalize
            // We are forced to count them to know if we have >= 10.
            let mut position_total: u32 = 0;
            for k in &current_knots {
                position_total += k.value() as u32;
            }

            // 3. Determine carry and remainder
            // Ancient logic: "I have 14. Tie one knot for next level, keep 4."
            let carry_count = position_total / 10;
            let remainder = (position_total % 10) as u8;

            // Prepare carry for next iteration (Simple knots)
            carry_knots = vec![Knot::Simple; carry_count as usize];

            // 4. Form the knots for this level
            let new_level_knots = if remainder == 0 {
                Vec::new()
            } else if i == 0 {
                // Units level: 1 is FigureEight, 2-9 is Long
                if remainder == 1 {
                    vec![Knot::FigureEight]
                } else {
                    vec![Knot::Long(remainder)]
                }
            } else {
                // Higher levels: remainder number of Simple knots
                vec![Knot::Simple; remainder as usize]
            };

            new_clusters.push(new_level_knots);
        }

        // Handle remaining carry
        while !carry_knots.is_empty() {
            let mut position_total: u32 = 0;
            for k in &carry_knots {
                position_total += k.value() as u32;
            }

            let carry_count = position_total / 10;
            let remainder = (position_total % 10) as u8;

            carry_knots = vec![Knot::Simple; carry_count as usize];

            let new_level_knots = if remainder == 0 {
                Vec::new()
            } else {
                vec![Knot::Simple; remainder as usize]
            };
            new_clusters.push(new_level_knots);
        }

        Cord {
            clusters: new_clusters,
        }
    }

    /// Helper to convert to modern u32 (for verification/interaction)
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

impl FromStr for Cord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplified parser for now
        // Assuming format like "●●\n|\n∞"
        let parts: Vec<&str> = s.split('|').collect();
        // parts are high to low.
        // We want low to high for clusters.

        let mut clusters = Vec::new();

        for part in parts.iter().rev() {
            let mut knots = Vec::new();
            // Split by whitespace or just scan chars
            for c in part.chars() {
                match c {
                    '●' => knots.push(Knot::Simple),
                    '∞' => knots.push(Knot::FigureEight),
                    '≡' => {
                        // We will rely on 'L' parsing below or specialized parsing
                        // But for now, let's handle the symbol itself if followed by number?
                        // It's hard in a simple char loop.
                    }
                    's' => knots.push(Knot::Simple),
                    'E' => knots.push(Knot::FigureEight),
                    _ => {}
                }
            }

            // Handle Long knots if formatted as L3 or ≡3
            // Simple scan for numbers in the string
            let part_trim = part.trim();
            if part_trim.contains('L') {
                for word in part_trim.split_whitespace() {
                    if word.starts_with('L') {
                        let val = word[1..].parse::<u8>().unwrap_or(0);
                        knots.push(Knot::Long(val));
                    }
                }
            } else if part_trim.contains('≡') {
                // Format: ≡3
                if let Some(idx) = part_trim.find('≡') {
                    if let Ok(val) = part_trim[idx + 3..].parse::<u8>() {
                        // '≡' is U+2261. 3 bytes in UTF-8.
                        // Safe because we skip the 3 bytes of the character.
                        knots.push(Knot::Long(val));
                    } else {
                        // Try finding digit
                        if let Some(digit_idx) = part_trim.find(|c: char| c.is_digit(10)) {
                            if let Ok(val) = part_trim[digit_idx..].parse::<u8>() {
                                knots.push(Knot::Long(val));
                            }
                        }
                    }
                }
            }

            clusters.push(knots);
        }

        Ok(Cord { clusters })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_panic_repro() {
        // This test reproduces the panic caused by unsafe slicing of '≡'
        // '≡' is 3 bytes. The original code tried to slice at index 1.
        // This should no longer panic.
        let _ = Cord::from_str("≡");
    }

    #[test]
    fn test_from_str_basic() {
        let cord = Cord::from_str("∞").unwrap();
        assert_eq!(cord.value(), 1);
    }
}
