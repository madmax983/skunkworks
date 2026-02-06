pub struct Euclidean {
    pub steps: usize,
    pub pulses: usize,
    pub pattern: Vec<bool>,
}

impl Euclidean {
    pub fn new(pulses: usize, steps: usize) -> Self {
        let pattern = Self::generate(pulses, steps);
        Self {
            steps,
            pulses,
            pattern,
        }
    }

    pub fn generate(k: usize, n: usize) -> Vec<bool> {
        if n == 0 {
            return vec![];
        }
        let mut pattern = vec![false; n];

        // Using (i * k) % n < k
        // This generates a variation of the Euclidean rhythm.
        // E(3,8): 0, 3, 6, 1(9), 4(12), 7(15), 2(18), 5(21)
        // Checks < 3:
        // 0: 0 < 3 (T)
        // 1: 3 < 3 (F)
        // 2: 6 < 3 (F)
        // 3: 1 < 3 (T)
        // 4: 4 < 3 (F)
        // 5: 7 < 3 (F)
        // 6: 2 < 3 (T)
        // 7: 5 < 3 (F)
        // Result: X . . X . . X .
        // This matches the standard E(3,8) [X..X..X.] perfectly.

        for i in 0..n {
            if (i * k) % n < k {
                pattern[i] = true;
            }
        }
        pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bools_to_str(v: &[bool]) -> String {
        v.iter().map(|&b| if b { 'X' } else { '.' }).collect()
    }

    #[test]
    fn test_e3_8() {
        let e = Euclidean::new(3, 8);
        assert_eq!(bools_to_str(&e.pattern), "X..X..X.");
    }

    #[test]
    fn test_e5_13() {
        let e = Euclidean::new(5, 13);
        assert_eq!(e.pattern.iter().filter(|&&b| b).count(), 5);
        // E(5,13) is usually X.X.X..X.X... or rotation
        // My alg:
        // 0: 0 < 5 T
        // 1: 5 < 5 F
        // 2: 10 < 5 F
        // 3: 2 (15) < 5 T
        // 4: 7 (20) < 5 F
        // 5: 12 (25) < 5 F
        // 6: 4 (30) < 5 T
        // 7: 9 (35) < 5 F
        // 8: 1 (40) < 5 T
        // 9: 6 (45) < 5 F
        // 10: 11 (50) < 5 F
        // 11: 3 (55) < 5 T
        // 12: 8 (60) < 5 F
        // Res: X..X..X.X..X.
        println!("E(5,13): {}", bools_to_str(&e.pattern));
        // Looks evenly spaced.
    }
}
