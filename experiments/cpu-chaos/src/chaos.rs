pub struct TimeDelayEmbedding {
    pub dimension: usize,
    pub delay: usize,
}

impl TimeDelayEmbedding {
    pub fn new(dimension: usize, delay: usize) -> Self {
        Self { dimension, delay }
    }

    pub fn embed(&self, data: &[f64]) -> Vec<Vec<f64>> {
        if data.len() < (self.dimension - 1) * self.delay + 1 {
            return vec![];
        }

        let count = data.len() - (self.dimension - 1) * self.delay;
        let mut result = Vec::with_capacity(count);

        for i in 0..count {
            let mut point = Vec::with_capacity(self.dimension);
            for d in 0..self.dimension {
                point.push(data[i + d * self.delay]);
            }
            result.push(point);
        }
        result
    }
}

pub struct LyapunovEstimator {
    pub window: usize,
}

impl LyapunovEstimator {
    pub fn new(window: usize) -> Self {
        Self { window }
    }

    pub fn estimate(&self, points: &[Vec<f64>]) -> f64 {
        let n = points.len();
        if n < self.window + 1 {
            return 0.0;
        }

        let mut sum_log_divergence = 0.0;
        let mut count = 0;
        let theiler_window = 10; // Ignore temporally close points

        for i in 0..(n - self.window) {
            let p_i = &points[i];

            // Find Nearest Neighbor
            let mut min_dist_sq = f64::MAX;
            let mut nearest_j = None;

            for j in 0..(n - self.window) {
                if (i as isize - j as isize).abs() < theiler_window {
                    continue;
                }

                let dist_sq = self.dist_sq(p_i, &points[j]);
                if dist_sq < min_dist_sq && dist_sq > 0.0 {
                    min_dist_sq = dist_sq;
                    nearest_j = Some(j);
                }
            }

            if let Some(j) = nearest_j {
                let init_dist = min_dist_sq.sqrt();
                if init_dist < 1e-9 { continue; } // Avoid division by zero or noise

                // Calculate divergence after 'window' steps
                let p_future_i = &points[i + self.window];
                let p_future_j = &points[j + self.window];
                let final_dist = self.dist_sq(p_future_i, p_future_j).sqrt();

                if final_dist > 1e-9 {
                    sum_log_divergence += (final_dist / init_dist).ln();
                    count += 1;
                }
            }
        }

        if count == 0 {
            println!("Lyapunov: No valid pairs found (n={})", n);
            return 0.0;
        }

        // Lyapunov exponent is the average exponential divergence rate per step
        (sum_log_divergence / count as f64) / self.window as f64
    }

    fn dist_sq(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logistic_map_chaos() {
        // Generate logistic map data (r=4.0)
        // Note: 0.5 maps to 1.0 -> 0.0 (fixed point), so use 0.1
        let mut data = vec![0.1];
        let r = 4.0;
        for _ in 0..1000 {
            let last = *data.last().unwrap();
            data.push(r * last * (1.0 - last));
        }

        let embedding = TimeDelayEmbedding::new(3, 1);
        let points = embedding.embed(&data);

        assert!(points.len() > 900);

        let estimator = LyapunovEstimator::new(5);
        let lambda = estimator.estimate(&points);

        println!("Estimated Lambda (Logistic): {}", lambda);
        // Standard Lyapunov for logistic map r=4 is ln(2) = 0.693
        // Our estimation might be lower/higher but should be clearly positive
        assert!(lambda > 0.2, "Lyapunov exponent should be positive for logistic map, got {}", lambda);
    }

    #[test]
    fn test_sine_wave_stability() {
        let mut data = vec![];
        for i in 0..1000 {
            data.push((i as f64 * 0.1).sin());
        }

        // Embed sine wave
        // Delay 15 is roughly pi/2 at step 0.1 (1.5 rad)
        let embedding = TimeDelayEmbedding::new(3, 15);
        let points = embedding.embed(&data);

        let estimator = LyapunovEstimator::new(10);
        let lambda = estimator.estimate(&points);

        println!("Estimated Lambda (Sine): {}", lambda);
        assert!(lambda.abs() < 0.15, "Lyapunov exponent should be near zero for sine wave, got {}", lambda);
    }
}
