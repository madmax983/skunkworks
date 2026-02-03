use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct HarmonographParams {
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,
    pub f4: f64,
    pub p1: f64,
    pub p2: f64,
    pub p3: f64,
    pub p4: f64,
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
    pub d4: f64,
}

impl HarmonographParams {
    pub fn from_hash(hash: &str) -> Self {
        // Ensure we have enough data. If hash is short (it shouldn't be), pad or cycle.
        // SHA1 is 40 chars. We need 12 params.
        // Let's take 3 chars per param -> 12 * 3 = 36 chars. Perfect.

        let get_val = |start: usize, count: usize| -> f64 {
            if start + count > hash.len() {
                return 0.5; // Fallback
            }
            let slice = &hash[start..start + count];
            let val = u32::from_str_radix(slice, 16).unwrap_or(0);
            val as f64
        };

        // Frequencies: Range [1.0, 5.0] mostly integers or simple ratios look best
        // But for "chaos", float is fine.
        // Let's normalize to [0, 1] then map.
        let norm = |v: f64, max: f64| v / max;

        let f1 = 1.0 + norm(get_val(0, 3), 4096.0) * 4.0;
        let f2 = 1.0 + norm(get_val(3, 3), 4096.0) * 4.0;
        let f3 = 1.0 + norm(get_val(6, 3), 4096.0) * 4.0;
        let f4 = 1.0 + norm(get_val(9, 3), 4096.0) * 4.0;

        // Phases: [0, 2PI]
        let p1 = norm(get_val(12, 3), 4096.0) * 2.0 * PI;
        let p2 = norm(get_val(15, 3), 4096.0) * 2.0 * PI;
        let p3 = norm(get_val(18, 3), 4096.0) * 2.0 * PI;
        let p4 = norm(get_val(21, 3), 4096.0) * 2.0 * PI;

        // Damping: [0.001, 0.05]
        // Very small damping makes it last longer (more complex).
        let d1 = 0.001 + norm(get_val(24, 3), 4096.0) * 0.02;
        let d2 = 0.001 + norm(get_val(27, 3), 4096.0) * 0.02;
        let d3 = 0.001 + norm(get_val(30, 3), 4096.0) * 0.02;
        let d4 = 0.001 + norm(get_val(33, 3), 4096.0) * 0.02;

        Self {
            f1,
            f2,
            f3,
            f4,
            p1,
            p2,
            p3,
            p4,
            d1,
            d2,
            d3,
            d4,
        }
    }
}

pub fn generate_points(params: &HarmonographParams, steps: usize) -> Vec<(f64, f64)> {
    let mut points = Vec::with_capacity(steps);
    let dt = 0.05; // Time step
    let amp = 100.0; // Base amplitude

    for i in 0..steps {
        let t = i as f64 * dt;

        // Pendulum 1 & 2 affect X
        let x = amp
            * (f64::sin(params.f1 * t + params.p1) * (-params.d1 * t).exp()
                + f64::sin(params.f2 * t + params.p2) * (-params.d2 * t).exp());

        // Pendulum 3 & 4 affect Y
        let y = amp
            * (f64::sin(params.f3 * t + params.p3) * (-params.d3 * t).exp()
                + f64::sin(params.f4 * t + params.p4) * (-params.d4 * t).exp());

        points.push((x, y));
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_from_hash() {
        let hash = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";
        let params = HarmonographParams::from_hash(hash);

        // Just verify ranges
        assert!(params.f1 >= 1.0 && params.f1 <= 5.0);
        assert!(params.p1 >= 0.0 && params.p1 <= 2.0 * PI);
        assert!(params.d1 >= 0.001);
    }

    #[test]
    fn test_generate_points() {
        let params = HarmonographParams {
            f1: 1.0,
            f2: 1.0,
            f3: 1.0,
            f4: 1.0,
            p1: 0.0,
            p2: 0.0,
            p3: 0.0,
            p4: 0.0,
            d1: 0.0,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
        };
        let points = generate_points(&params, 10);
        assert_eq!(points.len(), 10);
    }
}
