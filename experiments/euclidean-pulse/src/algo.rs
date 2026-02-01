/// Generates a Euclidean rhythm pattern.
///
/// Uses the formula `(i * pulses) % steps < pulses` to generate a maximally even distribution.
///
/// # Arguments
/// * `steps` - The total number of steps in the cycle (e.g., 16).
/// * `pulses` - The number of active beats (hits) in the cycle.
///
/// # Returns
/// A vector of booleans where `true` is a hit and `false` is a rest.
pub fn bjorklund(steps: usize, pulses: usize) -> Vec<bool> {
    if steps == 0 {
        return vec![];
    }
    let pulses = pulses.min(steps); // Clamp pulses to steps
    let mut pattern = Vec::with_capacity(steps);

    // We can use a rotational offset if we want to align the "downbeat".
    // The standard formula ((i * k) % n) < k puts a beat at 0.
    for i in 0..steps {
        // Using u64 to prevent overflow if steps is huge (unlikely for rhythm)
        let val = (i as u64 * pulses as u64) % steps as u64;
        pattern.push(val < pulses as u64);
    }
    pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e3_8() {
        let p = bjorklund(8, 3);
        // Expect: x . . x . . x .
        assert_eq!(p, vec![true, false, false, true, false, false, true, false]);
    }

    #[test]
    fn test_e5_13() {
        let p = bjorklund(13, 5);
        // Expect spacing: 3, 3, 2, 3, 2 approx
        // Indices: 0, 3, 6, 8, 11
        let indices: Vec<usize> = p
            .iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(indices, vec![0, 3, 6, 8, 11]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(bjorklund(0, 5), Vec::<bool>::new());
    }

    #[test]
    fn test_full() {
        let p = bjorklund(4, 4);
        assert_eq!(p, vec![true, true, true, true]);
    }

    #[test]
    fn test_clamp() {
        let p = bjorklund(4, 10);
        assert_eq!(p, vec![true, true, true, true]);
    }
}
