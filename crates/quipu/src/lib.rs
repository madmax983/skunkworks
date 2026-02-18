//! # Quipu 🧶
//!
//! > "The Quipu is a device for recording information, consisting of a main cord with smaller cords of different colors attached to it and knotted in various ways." - *Garcilaso de la Vega*
//!
//! A library for modeling the **Quipu** (Khipu), the ancient Inca recording device used for accounting and census data.
//! This crate provides data structures to represent Knots, Cords, and the Quipu itself, allowing you to perform
//! arithmetic operations using the logic of the Inca civilization.
//!
//! ## The Inca Number System
//!
//! The Incas used a **base-10 positional system**, similar to ours, but represented vertically on hanging cords:
//!
//! - **The Top:** Higher powers of 10 (Hundreds, Thousands, etc.).
//! - **The Bottom:** The Units place ($10^0$).
//! - **The Zero:** Represented by an empty space (no knot) in a position.
//!
//! ## Knots
//!
//! There are three types of knots used to represent numbers:
//!
//! 1.  **Simple Knot (●):** Represents `1` in the Tens place and higher.
//! 2.  **Long Knot (≡L):** Represents `2` to `9` in the Units place. The number of turns indicates the value.
//! 3.  **Figure-Eight Knot (∞):** Represents `1` in the Units place.
//!
//! ## Example: The Hero's Journey (Accounting for the Harvest)
//!
//! Imagine you are a *Quipucamayoc* (Keeper of the Quipu), recording the harvest of potatoes and maize.
//!
//! ```
//! use quipu::{Quipu, Cord, Knot};
//!
//! // 1. Create a new Quipu to record the harvest.
//! let mut harvest_record = Quipu::new();
//!
//! // 2. Record 123 sacks of potatoes.
//! //    - 1 Hundred (Simple)
//! //    - 2 Tens (Simple, Simple)
//! //    - 3 Units (Long Knot with 3 turns)
//! let potatoes = Cord::from(123);
//! harvest_record.add_cord(potatoes);
//!
//! // 3. Record 45 sacks of maize.
//! //    - 4 Tens (Simple x4)
//! //    - 5 Units (Long Knot with 5 turns)
//! let maize = Cord::from(45);
//! harvest_record.add_cord(maize);
//!
//! // 4. Calculate the total harvest.
//! //    The Incas performed arithmetic by moving knots or combining cords.
//! let total = harvest_record.cords[0].clone() + harvest_record.cords[1].clone();
//!
//! assert_eq!(total.value(), 168);
//!
//! // Display the total cord (TUI representation)
//! // Output:
//! // ●          (1 Hundred)
//! // ● ● ● ● ● ● (6 Tens)
//! // ≡8         (8 Units)
//! println!("{}", total);
//! ```

use std::fmt;
use std::ops::{Add, Sub};

#[cfg(feature = "audio")]
pub mod audio;

/// Represents a single knot on a Quipu cord.
///
/// Knots are the fundamental digits of the Inca number system.
/// Their value depends on their type and position (though position is handled by [`Cord`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Knot {
    /// **Simple Knot (s)**: Value 1. Used for all positions *except* Units.
    ///
    /// Visually represented as a small dot.
    Simple,
    /// **Long Knot (L)**: Value 2-9. Used *only* for the Units position.
    ///
    /// The `u8` payload represents the number of turns (and thus the value).
    Long(u8),
    /// **Figure-Eight Knot (E)**: Value 1. Used *only* for the Units position.
    ///
    /// A special knot used because a Simple knot in the units place could be mistaken for a Long knot with 1 turn (which doesn't exist).
    FigureEight,
}

impl Knot {
    /// Returns the numeric value of the knot.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Knot;
    ///
    /// assert_eq!(Knot::Simple.value(), 1);
    /// assert_eq!(Knot::FigureEight.value(), 1);
    /// assert_eq!(Knot::Long(5).value(), 5);
    /// ```
    pub fn value(&self) -> u8 {
        match self {
            Knot::Simple => 1,
            Knot::Long(v) => *v,
            Knot::FigureEight => 1,
        }
    }

    /// Returns the symbol used for TUI display.
    ///
    /// - `●`: Simple Knot
    /// - `≡N`: Long Knot (where N is the value)
    /// - `∞`: Figure-Eight Knot
    pub fn symbol(&self) -> String {
        match self {
            Knot::Simple => "●".to_string(),
            Knot::Long(v) => format!("≡{}", v),
            Knot::FigureEight => "∞".to_string(),
        }
    }
}

/// Represents the color of a cord, which could indicate data type or category.
///
/// In the Inca system, colors were used to distinguish different types of data
/// (e.g., one color for potatoes, another for maize, another for census data).
///
/// # Examples
///
/// ```
/// use quipu::Color;
/// let c = Color::Red;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    /// Un-dyed cotton or wool (the default).
    #[default]
    Natural,
    /// Often associated with political or military data.
    Red,
    /// Often associated with conquest or geography.
    Green,
    /// Often associated with religious data.
    Blue,
    /// Often associated with gold or corn (maize).
    Yellow,
    /// Often associated with time or history.
    Black,
    /// Often associated with silver or peace.
    White,
}

/// A hanging cord representing a single integer number.
///
/// The cord is divided into clusters of knots, representing powers of 10.
///
/// - **Index 0:** Units ($10^0$)
/// - **Index 1:** Tens ($10^1$)
/// - **Index 2:** Hundreds ($10^2$)
/// - ...and so on.
///
/// # Examples
///
/// ## Creating a Cord
///
/// ```
/// use quipu::{Cord, Knot};
///
/// let cord = Cord::from(205);
/// // This cord will have:
/// // - Index 0 (Units): Knot::Long(5)
/// // - Index 1 (Tens): Empty (Zero)
/// // - Index 2 (Hundreds): 2 Simple Knots
/// assert_eq!(cord.value(), 205);
/// ```
///
/// ## Accessing Clusters (Advanced)
///
/// You can inspect the raw knots if needed, though `value()` is preferred.
///
/// ```
/// use quipu::{Cord, Knot};
///
/// let cord = Cord::from(12);
/// // 12 -> 2 Units, 1 Ten
///
/// // Units (10^0)
/// assert_eq!(cord.clusters[0], vec![Knot::Long(2)]);
///
/// // Tens (10^1)
/// assert_eq!(cord.clusters[1], vec![Knot::Simple]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cord {
    /// The clusters of knots, ordered from Units (index 0) to highest power.
    ///
    /// `clusters[0]` represents units ($10^0$), `clusters[1]` represents tens ($10^1$), etc.
    pub clusters: Vec<Vec<Knot>>,
    /// Subsidiary cords hanging from this cord.
    pub subsidiaries: Vec<Cord>,
    /// The color of the cord.
    pub color: Color,
}

impl Cord {
    /// Creates a new, empty Cord (representing 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    /// let cord = Cord::new();
    /// assert_eq!(cord.value(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculates the total integer value of the cord.
    ///
    /// Iterates through the clusters, summing the knot values and applying the power-of-10 multiplier.
    pub fn value(&self) -> u64 {
        let mut total: u64 = 0;
        let mut multiplier: u64 = 1;

        for (i, cluster) in self.clusters.iter().enumerate() {
            let mut cluster_val: u64 = 0;
            for knot in cluster {
                cluster_val += knot.value() as u64;
            }
            // Use saturating arithmetic to prevent panic/wrap on overflow
            let term = cluster_val.saturating_mul(multiplier);
            total = total.saturating_add(term);

            if i < self.clusters.len() - 1 {
                multiplier = multiplier.saturating_mul(10);
            }
        }
        total
    }

    /// Performs checked subtraction.
    ///
    /// Computes `self - rhs`, returning `None` if the result would be negative (underflow).
    /// This is safer than the standard `Sub` trait, which may panic or wrap unexpectedly
    /// depending on implementation, although the Inca system strictly deals with natural numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    ///
    /// let c100 = Cord::from(100);
    /// let c50 = Cord::from(50);
    ///
    /// let result = c100.checked_sub(&c50);
    /// assert_eq!(result.unwrap().value(), 50);
    ///
    /// let underflow = c50.checked_sub(&c100);
    /// assert!(underflow.is_none());
    /// ```
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
    /// Converts a `u64` integer into a `Cord`.
    ///
    /// This process mimics the physical act of tying knots:
    /// - Digits 1-9 in the units place become Long Knots (or Figure-Eight for 1).
    /// - Digits 1-9 in higher places become clusters of Simple Knots.
    /// - Zeros become empty spaces (empty clusters).
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    ///
    /// let c = Cord::from(321);
    /// assert_eq!(c.value(), 321);
    /// ```
    fn from(mut val: u64) -> Self {
        if val == 0 {
            return Cord::default();
        }

        // Optimization: Pre-allocate vector capacity to avoid reallocations.
        // The number of clusters corresponds to the number of digits in base 10.
        let capacity = (val.checked_ilog10().unwrap_or(0) + 1) as usize;
        let mut clusters = Vec::with_capacity(capacity);
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

        Cord {
            clusters,
            subsidiaries: Vec::new(),
            color: Color::default(),
        }
    }
}

impl Cord {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        let indent = "  ".repeat(level);

        if self.color != Color::Natural {
            writeln!(f, "{}[{:?}]", indent, self.color)?;
        }

        if self.clusters.is_empty() {
            write!(f, "{}(empty)", indent)?;
        } else {
            for (i, cluster) in self.clusters.iter().enumerate().rev() {
                write!(f, "{}", indent)?;
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
                    writeln!(f)?;
                }
            }
        }

        if !self.subsidiaries.is_empty() {
            writeln!(f)?;
            for (idx, sub) in self.subsidiaries.iter().enumerate() {
                sub.fmt_indented(f, level + 1)?;
                if idx < self.subsidiaries.len() - 1 {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cord {
    /// Formats the Cord for display, mimicking the visual appearance of a Quipu.
    ///
    /// The cord is displayed vertically (top-down), so higher powers of 10 appear first.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    ///
    /// let cord = Cord::from(123);
    /// // Output:
    /// // ●          (100)
    /// // ● ●        (20)
    /// // ≡3         (3)
    /// println!("{}", cord);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}

impl Add for Cord {
    type Output = Cord;

    /// Adds two Cords together.
    ///
    /// The result is a new Cord representing the sum of the values.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    ///
    /// let c1 = Cord::from(100);
    /// let c2 = Cord::from(50);
    /// let sum = c1 + c2;
    ///
    /// assert_eq!(sum.value(), 150);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let val = self.value() + rhs.value();
        Cord::from(val)
    }
}

impl Sub for Cord {
    type Output = Cord;

    /// Subtracts one Cord from another.
    ///
    /// # Panics
    ///
    /// Panics if the result would be negative (i.e., `rhs > self`).
    /// The Inca number system does not support negative numbers.
    /// Use [`Cord::checked_sub`] for safe subtraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::Cord;
    ///
    /// let c1 = Cord::from(100);
    /// let c2 = Cord::from(25);
    /// let diff = c1 - c2;
    ///
    /// assert_eq!(diff.value(), 75);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        // Only implementing positive result subtraction
        if self.value() < rhs.value() {
            // In a real library we might want to return Result or panic,
            // but for now panic fits the original behavior.
            panic!("Quipu subtraction resulted in negative value (not supported by Incas! Use Cord::checked_sub for safety)");
        }

        let val = self.value() - rhs.value();
        Cord::from(val)
    }
}

/// A full Quipu: A collection of cords hanging from a main primary cord.
///
/// This acts as a database or ledger.
///
/// # Example
///
/// ```
/// use quipu::{Quipu, Cord};
///
/// let mut q = Quipu::new();
/// q.add_cord(Cord::from(10));
/// q.add_cord(Cord::from(20));
///
/// assert_eq!(q.cords.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Quipu {
    /// The list of pendant cords attached to the main cord.
    ///
    /// Each cord represents a number or data point.
    /// The index in the vector corresponds to the physical position on the main cord.
    pub cords: Vec<Cord>,
}

impl Quipu {
    /// Creates a new, empty Quipu.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a cord to the Quipu.
    ///
    /// The new cord is attached at the end of the main cord.
    ///
    /// # Examples
    ///
    /// ```
    /// use quipu::{Quipu, Cord};
    ///
    /// let mut q = Quipu::new();
    /// q.add_cord(Cord::from(10));
    ///
    /// assert_eq!(q.cords.len(), 1);
    /// ```
    pub fn add_cord(&mut self, cord: Cord) {
        self.cords.push(cord);
    }
}

impl fmt::Display for Quipu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quipu with {} cords:", self.cords.len())?;
        for (i, cord) in self.cords.iter().enumerate() {
            writeln!(f, "Cord {}:\n{}", i, cord)?;
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
