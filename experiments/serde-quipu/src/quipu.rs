use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Knot {
    Single,        // 's' - Used for powers >= 10
    Long(u8),      // 'L' - Used for 2-9 in units position
    FigureEight,   // 'E' - Used for 1 in units position
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cluster {
    pub knots: Vec<Knot>,
    pub value: u8,
    pub position_power: u32, // Power of 10 this cluster represents (0 = units, 1 = tens, etc.)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cord {
    pub clusters: Vec<Cluster>, // Ordered from Top (Highest Power) to Bottom (Units)
    pub color: String,          // For metadata
    pub subsidiaries: Vec<Cord>, // Subsidiary cords attached to this cord
}

impl Cord {
    pub fn new(color: impl Into<String>) -> Self {
        Self {
            clusters: Vec::new(),
            color: color.into(),
            subsidiaries: Vec::new(),
        }
    }

    /// Converts a u64 into a Cord representation.
    pub fn from_u64(mut n: u64, color: impl Into<String>) -> Self {
        if n == 0 {
            return Self {
                clusters: vec![Cluster { knots: vec![], value: 0, position_power: 0 }],
                color: color.into(),
                subsidiaries: Vec::new(),
            };
        }

        let mut clusters = Vec::new();
        let mut power = 0;

        while n > 0 {
            let digit = (n % 10) as u8;
            n /= 10;

            let knots = if power == 0 {
                match digit {
                    0 => vec![],
                    1 => vec![Knot::FigureEight],
                    2..=9 => vec![Knot::Long(digit)],
                    _ => unreachable!(),
                }
            } else {
                match digit {
                    0 => vec![],
                    d => vec![Knot::Single; d as usize],
                }
            };

            clusters.push(Cluster {
                knots,
                value: digit,
                position_power: power,
            });

            power += 1;
        }

        clusters.reverse();

        Self {
            clusters,
            color: color.into(),
            subsidiaries: Vec::new(),
        }
    }

    pub fn add_subsidiary(&mut self, cord: Cord) {
        self.subsidiaries.push(cord);
    }

    pub fn to_u64(&self) -> u64 {
        let mut n = 0;
        for cluster in &self.clusters {
            n += (cluster.value as u64) * 10u64.pow(cluster.position_power);
        }
        n
    }
}

impl fmt::Display for Cord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We need to handle indentation for recursive display
        // But Display trait doesn't accept arguments.
        // We'll just display the main cord here, and let the parent handle the tree structure?
        // Or we implement a helper.

        // This just formats the clusters.
        write!(f, "[{}] ", self.color)?;

        if self.clusters.is_empty() {
             write!(f, "(empty)")?;
        } else {
            let parts: Vec<String> = self.clusters.iter().map(|c| {
                if c.knots.is_empty() {
                    "_".to_string()
                } else {
                    let k_str: Vec<String> = c.knots.iter().map(|k| match k {
                        Knot::Single => "s".to_string(),
                        Knot::Long(v) => format!("L{}", v),
                        Knot::FigureEight => "E".to_string(),
                    }).collect();
                    k_str.join("")
                }
            }).collect();
            write!(f, "{}", parts.join("-"))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quipu {
    pub cords: Vec<Cord>,
}

impl Default for Quipu {
    fn default() -> Self {
        Self::new()
    }
}

impl Quipu {
    pub fn new() -> Self {
        Self { cords: Vec::new() }
    }

    pub fn add_cord(&mut self, cord: Cord) {
        self.cords.push(cord);
    }
}

impl fmt::Display for Quipu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Q-ROOT")?;
        writeln!(f, "|")?;
        for cord in &self.cords {
            write_cord_recursive(f, cord, 0)?;
        }
        Ok(())
    }
}

fn write_cord_recursive(f: &mut fmt::Formatter<'_>, cord: &Cord, depth: usize) -> fmt::Result {
    let indent = "  ".repeat(depth);
    writeln!(f, "{}+-- {}", indent, cord)?;
    for sub in &cord.subsidiaries {
        // Subsidiary cords usually hang off the main cord.
        // We visualize them indented.
        // We add a "|" to show connection?
        write_cord_recursive(f, sub, depth + 1)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_conversion() {
        let cord = Cord::from_u64(123, "Red");
        assert_eq!(cord.to_string(), "[Red] s-ss-L3");
        assert_eq!(cord.to_u64(), 123);
    }

    #[test]
    fn test_nested_quipu() {
        let mut q = Quipu::new();
        let mut main_cord = Cord::from_u64(10, "Field1");
        let sub_cord = Cord::from_u64(5, "Field1_Sub");
        main_cord.add_subsidiary(sub_cord);

        q.add_cord(main_cord);

        let output = q.to_string();
        println!("{}", output);
        assert!(output.contains("+-- [Field1] s-_"));
        assert!(output.contains("  +-- [Field1_Sub] L5")); // L5 or E? 5 is L5.
    }
}
