#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Features {
    pub voice: f32,  // 0.0 (unvoiced) - 1.0 (voiced)
    pub place: f32,  // 0.0 (labial) - 1.0 (glottal)
    pub manner: f32, // 0.0 (stop) - 1.0 (vowel)
}

#[derive(Debug, Clone)]
pub struct Phoneme {
    pub features: Features,
    pub symbol: char,
}

pub fn distance(a: &Features, b: &Features) -> f32 {
    let dv = a.voice - b.voice;
    let dp = a.place - b.place;
    let dm = a.manner - b.manner;
    (dv * dv + dp * dp + dm * dm).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let f1 = Features { voice: 0.0, place: 0.0, manner: 0.0 };
        let f2 = Features { voice: 1.0, place: 0.0, manner: 0.0 };
        // Euclidean distance should be 1.0
        assert_eq!(distance(&f1, &f2), 1.0);

        let p = Phoneme { features: f1, symbol: 'p' }; // Usage to avoid warning
        assert_eq!(p.symbol, 'p');
    }
}
