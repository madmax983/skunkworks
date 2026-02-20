use crate::sexagesimal::Sexagesimal;

pub struct TimeSeries {
    pub data: Vec<Sexagesimal>,
}

impl TimeSeries {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn generate_sine_wave(len: usize) -> Self {
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            // Generate sin(t) where t goes from 0 to 4*PI
            let t = (i as f64 / len as f64) * 4.0 * std::f64::consts::PI;
            let val = t.sin();
            // Scale up to be interesting in integer/fraction
            // range [-1, 1] -> [0, 20] maybe?
            let scaled = (val + 1.0) * 10.0;
            data.push(Sexagesimal::from_f64(scaled));
        }
        Self { data }
    }

    pub fn simple_moving_average(&self, window: usize) -> Self {
        if window == 0 || window > self.data.len() {
            return Self::new();
        }

        let mut sma = Vec::with_capacity(self.data.len() - window + 1);

        for i in 0..=self.data.len() - window {
            let mut sum = Sexagesimal::new();
            for j in 0..window {
                sum = sum + self.data[i + j].clone();
            }
            let avg = sum / (window as u64);
            sma.push(avg);
        }

        Self { data: sma }
    }
}
