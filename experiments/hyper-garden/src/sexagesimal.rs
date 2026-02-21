#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sexagesimal {
    // Stored Big-Endian (Most significant digit first)
    pub digits: Vec<u8>,
}

impl Sexagesimal {
    pub fn new(digits: Vec<u8>) -> Self {
        Self { digits }
    }

    pub fn zero() -> Self {
        Self { digits: vec![0] }
    }

    pub fn from_u64(mut val: u64) -> Self {
        if val == 0 {
            return Self::zero();
        }
        let mut digits = Vec::new();
        while val > 0 {
            digits.push((val % 60) as u8);
            val /= 60;
        }
        digits.reverse();
        Self::new(digits)
    }
}
