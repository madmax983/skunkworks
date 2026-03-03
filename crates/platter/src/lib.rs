//! # Platter
//!
//! A 2D grid structure optimized for simulating fields like magnetism, density, or pheromones.
//!
//! The `platter` crate provides the [`Platter`] struct, which is designed to efficiently
//! store and update field values across a 2D grid. It includes methods for accumulating values,
//! hard-capping saturation, and applying time-based decay.

/// A 2D grid representing a field of values, commonly used for magnetism or fluid density.
///
/// `Platter` provides a flat `Vec<f64>` storage mapped to 2D coordinates `(x, y)`.
/// It supports two primary modes of modification:
/// - [`Platter::magnetize`] (or [`Platter::saturate`]): Adds a value with a hard cap at `1.0`. Useful for saturation.
/// - [`Platter::accumulate`]: Adds a value without a cap. Useful for density accumulation.
///
/// It also includes a [`Platter::decay`] method to simulate dissipation over time.
///
/// # DX Audit Notes
///
/// - **Negative Values**: `decay` respects the sign of values but zeroes them out if their *absolute* magnitude is below `0.001`.
/// - **Aliases**: Generic aliases like `saturate` and `get` are available for non-magnetic contexts (e.g. fluid simulation).
#[derive(Debug, Clone)]
pub struct Platter {
    /// The flat vector of field values.
    magnetism: Vec<f64>,
    /// The width of the grid.
    width: usize,
    /// The height of the grid.
    height: usize,
}

const DECAY_THRESHOLD: f64 = 0.001;
const SATURATION_LIMIT: f64 = 1.0;

impl Platter {
    /// Creates a new `Platter` with the specified dimensions, initialized to 0.0.
    ///
    /// # Panics
    ///
    /// Panics if `width * height` overflows `usize::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use platter::Platter;
    ///
    /// let p = Platter::new(10, 10);
    /// assert_eq!(p.width(), 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.checked_mul(height).expect("Platter size overflow");
        Self {
            magnetism: vec![0.0; size],
            width,
            height,
        }
    }

    /// Returns the width of the platter.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the platter.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the underlying magnetism grid.
    pub fn magnetism(&self) -> &[f64] {
        &self.magnetism
    }

    /// Adds a value to the cell at `(x, y)`, clamping the result to a maximum of `1.0`.
    ///
    /// This is typically used for pheromone trails or magnetic saturation where
    /// the field strength cannot exceed a physical limit.
    ///
    /// If `x` or `y` are out of bounds, the operation is ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use platter::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.magnetize(2, 2, 0.6);
    /// p.magnetize(2, 2, 0.5);
    ///
    /// // 0.6 + 0.5 = 1.1, but clamped to 1.0
    /// assert_eq!(p.get_magnetism(2, 2), 1.0);
    /// ```
    pub fn magnetize(&mut self, x: usize, y: usize, amount: f64) {
        if let Some(idx) = self.get_index(x, y) {
            self.magnetism[idx] = (self.magnetism[idx] + amount).min(SATURATION_LIMIT);
        }
    }

    /// Adds a value to the cell at `(x, y)` without clamping.
    ///
    /// This is useful for simulations like fluid density where values can accumulate arbitrarily.
    ///
    /// If `x` or `y` are out of bounds, the operation is ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use platter::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.accumulate(2, 2, 0.6);
    /// p.accumulate(2, 2, 0.5);
    ///
    /// // 0.6 + 0.5 = 1.1
    /// assert_eq!(p.get_magnetism(2, 2), 1.1);
    /// ```
    pub fn accumulate(&mut self, x: usize, y: usize, amount: f64) {
        if let Some(idx) = self.get_index(x, y) {
            self.magnetism[idx] += amount;
        }
    }

    /// Clears the platter back to zeros.
    pub fn clear(&mut self) {
        self.magnetism.fill(0.0);
    }

    /// Retrieves the value at `(x, y)`.
    ///
    /// Returns `0.0` if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use platter::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.accumulate(1, 1, 0.5);
    ///
    /// assert_eq!(p.get_magnetism(1, 1), 0.5);
    /// assert_eq!(p.get_magnetism(100, 100), 0.0); // Out of bounds
    /// ```
    pub fn get_magnetism(&self, x: usize, y: usize) -> f64 {
        self.get_index(x, y)
            .map(|idx| self.magnetism[idx])
            .unwrap_or(0.0)
    }

    /// Alias for [`Platter::get_magnetism`] for generic use cases (e.g., fluid density).
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.get_magnetism(x, y)
    }

    /// Alias for [`Platter::magnetize`] for generic use cases (e.g., saturation).
    #[inline]
    pub fn saturate(&mut self, x: usize, y: usize, amount: f64) {
        self.magnetize(x, y, amount)
    }

    /// Multiplies all values in the grid by `rate`, simulating decay.
    ///
    /// Values whose absolute magnitude falls below `0.001` are reset to `0.0`
    /// to avoid denormal numbers and clean up the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use platter::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.accumulate(2, 2, 1.0);
    /// p.decay(0.5);
    ///
    /// assert_eq!(p.get_magnetism(2, 2), 0.5);
    ///
    /// // Decay to below threshold
    /// p.decay(0.0001);
    /// assert_eq!(p.get_magnetism(2, 2), 0.0);
    /// ```
    pub fn decay(&mut self, rate: f64) {
        for m in &mut self.magnetism {
            let val = *m * rate;
            *m = if val.abs() < DECAY_THRESHOLD {
                0.0
            } else {
                val
            };
        }
    }

    /// Helper to get the index for a given coordinate.
    /// Returns `None` if coordinates are out of bounds.
    #[inline]
    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetize_decay() {
        let mut platter = Platter::new(10, 10);
        platter.magnetize(5, 5, 0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);

        platter.magnetize(5, 5, 0.6);
        assert!((platter.get_magnetism(5, 5) - 1.0).abs() < 1e-6); // Clamped

        platter.decay(0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_accumulate() {
        let mut platter = Platter::new(10, 10);
        platter.accumulate(5, 5, 0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);

        platter.accumulate(5, 5, 0.6);
        assert!((platter.get_magnetism(5, 5) - 1.1).abs() < 1e-6); // Not clamped
    }

    #[test]
    fn test_decay_negative_values() {
        let mut platter = Platter::new(10, 10);

        // Setup negative value
        platter.accumulate(2, 2, -10.0);
        assert!((platter.get(2, 2) - -10.0).abs() < 1e-6);

        // Decay by 50%
        platter.decay(0.5);
        assert!((platter.get(2, 2) - -5.0).abs() < 1e-6);

        // Decay to very small negative number (absolute value < 0.001)
        platter.decay(0.00001);
        assert_eq!(platter.get(2, 2), 0.0);
    }

    #[test]
    fn test_aliases() {
        let mut platter = Platter::new(10, 10);

        // Test saturate (alias for magnetize)
        platter.saturate(1, 1, 0.5);
        assert!((platter.get(1, 1) - 0.5).abs() < 1e-6);

        platter.saturate(1, 1, 0.6);
        assert!((platter.get(1, 1) - 1.0).abs() < 1e-6); // Should clamp like magnetize

        // Test get (alias for get_magnetism)
        assert_eq!(platter.get(1, 1), platter.get_magnetism(1, 1));
    }

    #[test]
    fn bench_platter_decay() {
        let width = 1000;
        let height = 1000;
        let mut platter = Platter::new(width, height);

        // Initialize with pattern
        for y in 0..height {
            for x in 0..width {
                let val = ((x + y) % 100) as f64 / 100.0;
                platter.accumulate(x, y, val);
            }
        }

        let start = std::time::Instant::now();
        // Run decay 100 times
        for _ in 0..100 {
            // Decay rate 0.99 ensures values stay non-zero for a while but some might drop below threshold
            platter.decay(0.99);
            std::hint::black_box(());
        }
        let duration = start.elapsed();
        println!("Time taken for 100 decays of 1M elements: {:?}", duration);
        println!("Time per decay: {:?}", duration / 100);
    }
}

#[cfg(test)]
mod warden_tests {
    use super::*;

    #[test]
    fn test_platter_encapsulation() {
        let p = Platter::new(10, 10);
        // p.width = 5; // This would fail to compile now because `width` is private.
        // p.height = 5; // This would fail to compile now because `height` is private.
        // p.magnetism.truncate(5); // This would fail to compile now because `magnetism` is private.

        assert_eq!(p.width(), 10);
        assert_eq!(p.height(), 10);
        assert_eq!(p.magnetism().len(), 100);
    }
}
