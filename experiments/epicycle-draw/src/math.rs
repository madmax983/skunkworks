use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    // Euler's formula: e^(ix) = cos(x) + i*sin(x)
    pub fn from_angle(angle: f64) -> Self {
        Self {
            re: angle.cos(),
            im: angle.sin(),
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(self) -> f64 {
        self.im.atan2(self.re)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Epicycle {
    pub freq: f64,
    pub amp: f64,
    pub phase: f64,
}

/// Compute Discrete Fourier Transform
/// Returns a vector of Epicycles sorted by amplitude (descending).
pub fn dft(signal: &[Complex]) -> Vec<Epicycle> {
    let n = signal.len();
    let mut epicycles = Vec::with_capacity(n);

    for k in 0..n {
        let mut sum = Complex::zero();

        for (t, &val) in signal.iter().enumerate() {
            let angle = -2.0 * PI * (k as f64) * (t as f64) / (n as f64);
            let c = Complex::from_angle(angle);
            sum = sum.add(val.mul(c));
        }

        // Convert k to frequency (handling aliasing for negative frequencies)
        // If k > N/2, it represents frequency k - N
        let freq = if k > n / 2 {
            (k as f64) - (n as f64)
        } else {
            k as f64
        };

        let amp = sum.magnitude() / (n as f64);
        let phase = sum.phase();

        epicycles.push(Epicycle { freq, amp, phase });
    }

    // Sort by amplitude (largest first) to make approximation look good with fewer circles
    epicycles.sort_by(|a, b| b.amp.partial_cmp(&a.amp).unwrap());

    epicycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        // (1 + 2i)(3 + 4i) = 3 + 4i + 6i - 8 = -5 + 10i
        let c = a.mul(b);
        assert!((c.re - -5.0).abs() < 1e-6);
        assert!((c.im - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_dft_constant() {
        // Signal: 1, 1, 1, 1
        // DFT should have DC component (freq 0) = 1, others 0
        let signal = vec![
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
        ];

        let result = dft(&signal);

        // Find freq 0
        let dc = result.iter().find(|e| e.freq == 0.0).unwrap();
        assert!((dc.amp - 1.0).abs() < 1e-6);

        // Others should be 0
        for e in result.iter() {
            if e.freq != 0.0 {
                assert!(e.amp < 1e-6);
            }
        }
    }
}
