pub fn distance(s1: &str, s2: &str) -> f32 {
    if s1 == s2 {
        return 0.0;
    }
    let len = s1.len().max(s2.len());
    if len == 0 {
        return 0.0;
    }
    let dist = strsim::levenshtein(s1, s2);
    dist as f32 / len as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical() {
        assert_eq!(distance("hello", "hello"), 0.0);
    }

    #[test]
    fn test_completely_different() {
        // "abc" vs "def" -> distance 3, len 3 -> 1.0
        assert_eq!(distance("abc", "def"), 1.0);
    }

    #[test]
    fn test_partial() {
        // "kitten" vs "sitting" -> dist 3, len 7 -> 3/7
        let d = distance("kitten", "sitting");
        assert!((d - 3.0 / 7.0).abs() < 1e-6);
    }

    #[test]
    fn test_empty() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("a", ""), 1.0);
    }
}
