use crate::sexagesimal::Sexagesimal;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub data: Vec<Sexagesimal>,
}

impl TimeSeries {
    pub fn new(data: Vec<Sexagesimal>) -> Self {
        Self { data }
    }

    pub fn moving_average(&self, window_size: usize) -> TimeSeries {
        if window_size == 0 || window_size > self.data.len() {
            return TimeSeries::new(vec![]);
        }

        let mut result = Vec::new();
        // Simple moving average
        // We need to sum `window_size` elements and divide by `window_size`.

        for i in 0..=self.data.len() - window_size {
            let window = &self.data[i..i+window_size];
            let mut sum = Sexagesimal::zero();
            for val in window {
                sum = sum + val.clone();
            }
            let avg = sum / (window_size as u64);
            result.push(avg);
        }

        TimeSeries::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_moving_average() {
        // Data: 10, 20, 30, 40, 50
        // Window 3.
        // [10, 20, 30] avg 20.
        // [20, 30, 40] avg 30.
        // [30, 40, 50] avg 40.

        let data = vec![
            Sexagesimal::from_str("𒌋").unwrap(), // 10
            Sexagesimal::from_str("𒌋𒌋").unwrap(), // 20
            Sexagesimal::from_str("𒌋𒌋𒌋").unwrap(), // 30
            Sexagesimal::from_str("𒌋𒌋𒌋𒌋").unwrap(), // 40
            Sexagesimal::from_str("𒌋𒌋𒌋𒌋𒌋").unwrap(), // 50
        ];

        let ts = TimeSeries::new(data);
        let ma = ts.moving_average(3);

        assert_eq!(ma.data.len(), 3);
        // 20
        assert_eq!(ma.data[0], Sexagesimal::from_str("𒌋𒌋").unwrap());
        // 30
        assert_eq!(ma.data[1], Sexagesimal::from_str("𒌋𒌋𒌋").unwrap());
        // 40
        assert_eq!(ma.data[2], Sexagesimal::from_str("𒌋𒌋𒌋𒌋").unwrap());
    }
}
