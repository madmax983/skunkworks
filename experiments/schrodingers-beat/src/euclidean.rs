pub fn generate(k: usize, n: usize) -> Vec<bool> {
    if n == 0 {
        return vec![];
    }
    let k = k.min(n);
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        // Simple Euclidean approximation using modular arithmetic
        // Generates maximally even sets (Bjorklund rhythms are a subset/rotation of these)
        let is_beat = (i * k) % n < k;
        result.push(is_beat);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_3_8() {
        let pattern = generate(3, 8);
        // Expect 3 beats
        assert_eq!(pattern.iter().filter(|&&b| b).count(), 3);
        // Expect length 8
        assert_eq!(pattern.len(), 8);
        // Pattern: 10010010 (based on (i*3)%8 < 3)
        // 0: 0<3 T
        // 1: 3<3 F
        // 2: 6<3 F
        // 3: 1<3 T
        // 4: 4<3 F
        // 5: 7<3 F
        // 6: 2<3 T
        // 7: 5<3 F
        assert_eq!(pattern, vec![true, false, false, true, false, false, true, false]);
    }

    #[test]
    fn test_euclidean_edge_cases() {
        assert_eq!(generate(0, 5), vec![false, false, false, false, false]);
        assert_eq!(generate(5, 5), vec![true, true, true, true, true]);
        assert_eq!(generate(10, 5), vec![true, true, true, true, true]); // k clamped to n
        assert_eq!(generate(1, 4), vec![true, false, false, false]);
    }
}
