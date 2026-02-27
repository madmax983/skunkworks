/// A 2D grid representing a field of values, commonly used for magnetism or fluid density.
///
/// `Platter` provides a flat `Vec<f64>` storage mapped to 2D coordinates `(x, y)`.
/// It supports two primary modes of modification:
/// - [`Platter::magnetize`]: Adds a value with a hard cap at `1.0`. Useful for saturation.
/// - [`Platter::accumulate`]: Adds a value without a cap. Useful for density accumulation.
///
/// It also includes a [`Platter::decay`] method to simulate dissipation over time.
#[derive(Debug, Clone)]
pub struct Platter {
    /// The flat vector of field values.
    pub magnetism: Vec<f64>,
    /// The width of the grid.
    pub width: usize,
    /// The height of the grid.
    pub height: usize,
}

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
    /// use ferrous_core::Platter;
    ///
    /// let p = Platter::new(10, 10);
    /// assert_eq!(p.width, 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.checked_mul(height).expect("Platter size overflow");
        Self {
            magnetism: vec![0.0; size],
            width,
            height,
        }
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
    /// use ferrous_core::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.magnetize(2, 2, 0.6);
    /// p.magnetize(2, 2, 0.5);
    ///
    /// // 0.6 + 0.5 = 1.1, but clamped to 1.0
    /// assert_eq!(p.get_magnetism(2, 2), 1.0);
    /// ```
    pub fn magnetize(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.magnetism[idx] = (self.magnetism[idx] + amount).min(1.0);
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
    /// use ferrous_core::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.accumulate(2, 2, 0.6);
    /// p.accumulate(2, 2, 0.5);
    ///
    /// // 0.6 + 0.5 = 1.1
    /// assert_eq!(p.get_magnetism(2, 2), 1.1);
    /// ```
    pub fn accumulate(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.magnetism[idx] += amount;
        }
    }

    /// Retrieves the value at `(x, y)`.
    ///
    /// Returns `0.0` if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrous_core::Platter;
    ///
    /// let mut p = Platter::new(5, 5);
    /// p.accumulate(1, 1, 0.5);
    ///
    /// assert_eq!(p.get_magnetism(1, 1), 0.5);
    /// assert_eq!(p.get_magnetism(100, 100), 0.0); // Out of bounds
    /// ```
    pub fn get_magnetism(&self, x: usize, y: usize) -> f64 {
        if x < self.width && y < self.height {
            self.magnetism[y * self.width + x]
        } else {
            0.0
        }
    }

    /// Multiplies all values in the grid by `rate`, simulating decay.
    ///
    /// Values that fall below `0.001` are reset to `0.0` to avoid denormal numbers
    /// and clean up the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrous_core::Platter;
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
            *m *= rate;
            if *m < 0.001 {
                *m = 0.0;
            }
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
}
