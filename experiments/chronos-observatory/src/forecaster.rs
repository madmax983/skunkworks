use crate::babylonian::BabylonianNumber;
use crate::mayan::MayanDate;
use num_rational::Ratio;
use num_traits::Zero;

pub struct Forecaster {
    pub data: Vec<(MayanDate, BabylonianNumber)>,
}

impl Default for Forecaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Forecaster {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_point(&mut self, date: MayanDate, value: BabylonianNumber) {
        self.data.push((date, value));
    }

    /// Calculate linear regression y = mx + b
    /// Returns (m, b)
    pub fn linear_regression(&self) -> Option<(BabylonianNumber, BabylonianNumber)> {
        let n = self.data.len() as i64;
        if n < 2 {
            return None;
        }

        let mut sum_x = Ratio::zero();
        let mut sum_y = Ratio::zero();
        let mut sum_xy = Ratio::zero();
        let mut sum_xx = Ratio::zero();

        for (date, value) in &self.data {
            let x = Ratio::from_integer(date.total_days() as i64);
            let y = value.0;

            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }

        let n_ratio = Ratio::from_integer(n);

        // m = (N * Σxy - Σx * Σy) / (N * Σx^2 - (Σx)^2)
        let numerator = (n_ratio * sum_xy) - (sum_x * sum_y);
        let denominator = (n_ratio * sum_xx) - (sum_x * sum_x);

        if denominator.is_zero() {
            return None; // Vertical line or all x same
        }

        let m = numerator / denominator;

        // b = (Σy - m * Σx) / N
        let b = (sum_y - (m * sum_x)) / n_ratio;

        Some((BabylonianNumber(m), BabylonianNumber(b)))
    }

    pub fn predict(&self, date: MayanDate) -> Option<BabylonianNumber> {
        let (m, b) = self.linear_regression()?;
        let x = BabylonianNumber::from(date.total_days() as i64);

        // y = mx + b
        Some(m * x + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_perfect_line() {
        let mut f = Forecaster::new();
        // y = x
        // Day 1, val 1
        // Day 2, val 2
        // Day 3, val 3

        f.add_point(MayanDate::new(0, 0, 0, 0, 1), BabylonianNumber::from(1));
        f.add_point(MayanDate::new(0, 0, 0, 0, 2), BabylonianNumber::from(2));
        f.add_point(MayanDate::new(0, 0, 0, 0, 3), BabylonianNumber::from(3));

        let (m, b) = f.linear_regression().unwrap();

        // m should be 1, b should be 0
        assert_eq!(m.0.to_integer(), 1);
        assert_eq!(b.0.to_integer(), 0);

        // Predict Day 4 -> 4
        let p = f.predict(MayanDate::new(0, 0, 0, 0, 4)).unwrap();
        assert_eq!(p.0.to_integer(), 4);
    }
}
