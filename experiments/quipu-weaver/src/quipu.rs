use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Knot {
    Single,        // Simple overhand knot. Used for 10s, 100s, etc.
    Long(u8),      // Multiple turns. Used for 2-9 in units position.
    FigureEight,   // Used for 1 in units position.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Natural, // White/Beige/Cotton
    Red,     // Government/Military?
    Blue,    // Religious?
    Green,   // Conquest?
    Yellow,  // Gold/Corn?
    Black,   // Time?
    Brown,   // Potatoes?
}

impl Default for Color {
    fn default() -> Self {
        Color::Natural
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cord {
    pub color: Color,
    /// Knots are stored with their "position" (level).
    /// Level 0 = Units (Bottom)
    /// Level 1 = Tens
    /// Level 2 = Hundreds
    /// ...
    pub knots: Vec<(u8, Knot)>,
    pub children: Vec<Cord>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Quipu {
    pub primary_cord: Cord,
}

impl Quipu {
    pub fn new() -> Self {
        Quipu {
            primary_cord: Cord {
                label: Some("Primary".to_string()),
                ..Default::default()
            },
        }
    }
}

impl Cord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, mut val: u64) -> Self {
        if val == 0 {
            // Empty cord represents zero? Or explicitly empty?
            // Usually, zero is just absence of knots in that position.
            return self;
        }

        let mut power = 0;
        while val > 0 {
            let digit = (val % 10) as u8;
            val /= 10;

            if digit > 0 {
                self.add_digit_knots(digit, power);
            }
            power += 1;
        }
        self
    }

    fn add_digit_knots(&mut self, digit: u8, power: u8) {
        if power == 0 {
            // Units position
            if digit == 1 {
                self.knots.push((power, Knot::FigureEight));
            } else {
                // 2-9 are Long knots
                self.knots.push((power, Knot::Long(digit)));
            }
        } else {
            // Tens, Hundreds, etc. use clusters of Single knots
            for _ in 0..digit {
                self.knots.push((power, Knot::Single));
            }
        }
    }
}

// Display impl for ASCII debugging
impl fmt::Display for Quipu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quipu:")?;
        writeln!(f, "{}", self.primary_cord)?;
        Ok(())
    }
}

impl fmt::Display for Cord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cord [{:?}]", self.label)?;
        if !self.knots.is_empty() {
            write!(f, " Knots: ")?;
            // Sort by power descending
            let mut sorted_knots = self.knots.clone();
            sorted_knots.sort_by(|a, b| b.0.cmp(&a.0));

            for (power, knot) in sorted_knots {
                match knot {
                    Knot::Single => write!(f, "(x @ 10^{}) ", power)?,
                    Knot::Long(t) => write!(f, "(L{} @ 10^{}) ", t, power)?,
                    Knot::FigureEight => write!(f, "(8 @ 10^{}) ", power)?,
                }
            }
        }
        if !self.children.is_empty() {
            writeln!(f, "\n  Children:")?;
            for child in &self.children {
                writeln!(f, "  - {}", child)?;
            }
        }
        Ok(())
    }
}
