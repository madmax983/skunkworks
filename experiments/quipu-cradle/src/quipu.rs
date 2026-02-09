/// Represents the type of knot used in the Quipu system.
/// Incan Quipus used a base-10 positional system.
#[derive(Debug, Clone, PartialEq)]
pub enum KnotType {
    /// A simple overhand knot. represents 1, 10, 100, etc.
    /// The position on the cord determines the power of 10.
    Single,
    /// A long knot with multiple turns (2-9).
    /// Used for units (2-9).
    Long(u8),
    /// A figure-eight knot.
    /// Used for units (1).
    FigureEight,
    /// Absence of a knot (0).
    /// Represented by a gap on the cord.
    Empty,
}

/// A single knot on a pendant cord.
#[derive(Debug, Clone, PartialEq)]
pub struct Knot {
    pub value: u8,
    pub knot_type: KnotType,
    /// The power of 10 this knot represents (0 = units, 1 = tens, etc.)
    /// In a physical Quipu, higher powers are higher up the cord.
    pub power: u8,
}

/// A pendant cord hanging from the main cord.
#[derive(Debug, Clone, PartialEq)]
pub struct Pendant {
    /// The color of the cord. Can be used for categorizing data types.
    pub color: [u8; 3],
    /// The knots on this cord, ordered from top (highest power) to bottom (lowest power).
    pub knots: Vec<Knot>,
    /// Subsidiary cords hanging from this pendant.
    pub subsidiaries: Vec<Pendant>,
}

/// The entire Quipu structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Quipu {
    /// The main horizontal cord.
    pub main_cord_color: [u8; 3],
    /// The pendant cords hanging from the main cord.
    pub pendants: Vec<Pendant>,
}

impl Quipu {
    pub fn new() -> Self {
        Self {
            main_cord_color: [200, 180, 140], // Default hemp/cotton color
            pendants: Vec::new(),
        }
    }
}

impl Pendant {
    pub fn new(color: [u8; 3]) -> Self {
        Self {
            color,
            knots: Vec::new(),
            subsidiaries: Vec::new(),
        }
    }

    /// Converts a u64 number into knots on this pendant.
    pub fn add_number(&mut self, mut value: u64) {
        if value == 0 {
            // Represent 0 as an empty cord (no knots)
            return;
        }

        let mut digits = Vec::new();

        while value > 0 {
            digits.push((value % 10) as u8);
            value /= 10;
        }

        // Process digits from highest power (top of cord) to lowest (bottom)
        // digits is [units, tens, hundreds...], so reverse it for processing?
        // Actually, physically:
        // Top: Hundreds
        // Middle: Tens
        // Bottom: Units

        // Let's iterate from highest power down to 0.
        for (p, &digit) in digits.iter().enumerate().rev() {
            let p = p as u8;
            if digit == 0 {
                continue; // 0 is empty space
            }

            match p {
                0 => {
                    // Units place rules:
                    // 1 -> Figure Eight
                    // 2-9 -> Long Knot with N turns
                    match digit {
                        1 => self.knots.push(Knot { value: 1, knot_type: KnotType::FigureEight, power: 0 }),
                        2..=9 => self.knots.push(Knot { value: digit, knot_type: KnotType::Long(digit), power: 0 }),
                        _ => {} // Should not happen
                    }
                }
                _ => {
                    // Tens, Hundreds, etc. use clusters of Single knots.
                    // E.g., 30 -> 3 single knots at the tens position.
                    for _ in 0..digit {
                        self.knots.push(Knot { value: 1, knot_type: KnotType::Single, power: p });
                    }
                }
            }
        }
    }
}
