use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Glyph(pub u8);

impl Glyph {
    pub fn new(byte: u8) -> Self {
        Self(byte)
    }

    pub fn to_braille(&self) -> char {
        std::char::from_u32(0x2800 + self.0 as u32).unwrap_or('⠀')
    }

    pub fn from_grid(grid: [bool; 8]) -> Self {
        let mut byte = 0u8;
        // Map grid indices to Braille bits
        // Grid is row-major 2x4 (w x h)
        // Indices:
        // 0 1
        // 2 3
        // 4 5
        // 6 7
        // Braille bits: 1, 2, 3, 7, 4, 5, 6, 8 (bits 0..7 map to dots 1..8)
        // Map grid index to BIT POSITION (0..7)
        // (0,0)->Dot1(Bit0), (1,0)->Dot4(Bit3)
        // (0,1)->Dot2(Bit1), (1,1)->Dot5(Bit4)
        // (0,2)->Dot3(Bit2), (1,2)->Dot6(Bit5)
        // (0,3)->Dot7(Bit6), (1,3)->Dot8(Bit7)
        let map = [0, 3, 1, 4, 2, 5, 6, 7];

        for (i, &set) in grid.iter().enumerate() {
            if set {
                byte |= 1 << map[i];
            }
        }
        Self(byte)
    }

    /// Merges two glyphs by horizontally concatenating them and smashing them back to 2 width.
    /// Implementation: ORs the left column of B onto the right column of A?
    /// Or ORs (Col0 of A, Col1 of A) -> New Col 0?
    /// Let's try: New Col 0 = A, New Col 1 = B.
    /// Wait, A is 2-wide. B is 2-wide.
    /// Result should be 2-wide.
    /// Let's take Left Col of A and Right Col of B? No, that loses info.
    /// Let's OR them? A | B. Overlap. This simulates writing on top.
    /// But `merge` implies sequence.
    /// Let's try: New Col 0 = A's bits OR B's bits (squashed)?
    /// Let's go with:
    /// New Col 0 = A's Col 0 | A's Col 1
    /// New Col 1 = B's Col 0 | B's Col 1
    /// This simplifies each character to a single vertical bar, then puts them side-by-side.
    pub fn merge(&self, other: &Glyph) -> Glyph {
        let g1 = self.to_grid();
        let g2 = other.to_grid();

        let mut new_grid = [false; 8];

        for row in 0..4 {
            // New Col 0 comes from A (OR of its cols)
            let a_idx0 = row * 2;
            let a_idx1 = row * 2 + 1;
            new_grid[row * 2] = g1[a_idx0] || g1[a_idx1];

            // New Col 1 comes from B (OR of its cols)
            let b_idx0 = row * 2;
            let b_idx1 = row * 2 + 1;
            new_grid[row * 2 + 1] = g2[b_idx0] || g2[b_idx1];
        }

        Glyph::from_grid(new_grid)
    }

    pub fn to_grid(&self) -> [bool; 8] {
        let mut grid = [false; 8];
        let map = [0, 3, 1, 4, 2, 5, 6, 7];
        for (i, bit_idx) in map.iter().enumerate() {
            if (self.0 >> bit_idx) & 1 == 1 {
                grid[i] = true;
            }
        }
        grid
    }

    pub fn erode(&self, rng: &mut impl Rng) -> Glyph {
        // Flip a random bit with low probability, or just unset a set bit?
        // Erosion usually means loss. So unset.
        let mut byte = self.0;
        if byte == 0 {
            return *self;
        }

        // Pick a bit to unset
        let bit = rng.gen_range(0..8);
        byte &= !(1 << bit);
        Glyph(byte)
    }
}

pub struct Script {
    pub tokens: Vec<Glyph>,
    pub corpus: Vec<usize>,       // Indices into tokens
    pub active_tokens: Vec<bool>, // To track if a token is still used (optional)
}

impl Script {
    pub fn new(text: &str) -> Self {
        // Initial state: ASCII 1-to-1
        // Tokens 0..255 are just the bytes
        // But we only care about used chars.
        // Let's make tokens dynamic.
        let mut tokens = Vec::new();
        let mut corpus = Vec::new();
        let mut char_map = HashMap::new();

        for b in text.bytes() {
            let id = *char_map.entry(b).or_insert_with(|| {
                let id = tokens.len();
                // Create a "pictogram" for this byte.
                // For now, simple hash or just the byte itself if we want raw hex visualization?
                // Let's generate a pseudo-random glyph based on the byte value to make it look "ancient".
                tokens.push(Self::generate_pictogram(b));
                id
            });
            corpus.push(id);
        }

        let count = tokens.len();
        Self {
            tokens,
            corpus,
            active_tokens: vec![true; count], // Not strictly used yet
        }
    }

    fn generate_pictogram(b: u8) -> Glyph {
        // Generate a dense-ish pattern seeded by b
        // e.g., use b as the bitmask directly?
        // ASCII 'a' is 0x61 (01100001). That's 3 dots.
        // It works.
        Glyph(b)
    }

    pub fn evolve(&mut self) -> Option<(usize, usize, usize)> {
        // 1. Count pairs
        if self.corpus.len() < 2 {
            return None;
        }

        let mut counts = HashMap::new();
        for window in self.corpus.windows(2) {
            if let [a, b] = window {
                *counts.entry((*a, *b)).or_insert(0) += 1;
            }
        }

        // 2. Find max
        let ((a, b), count) = counts.into_iter().max_by_key(|&(_, c)| c)?;

        if count < 2 {
            return None;
        } // Don't merge if it only happens once

        // 3. Create new token
        let glyph_a = self.tokens[a];
        let glyph_b = self.tokens[b];
        let new_glyph = glyph_a.merge(&glyph_b);
        let new_id = self.tokens.len();
        self.tokens.push(new_glyph);

        // 4. Replace in corpus
        let mut new_corpus = Vec::with_capacity(self.corpus.len());
        let mut i = 0;
        while i < self.corpus.len() {
            if i + 1 < self.corpus.len() && self.corpus[i] == a && self.corpus[i + 1] == b {
                new_corpus.push(new_id);
                i += 2;
            } else {
                new_corpus.push(self.corpus[i]);
                i += 1;
            }
        }
        self.corpus = new_corpus;

        Some((a, b, new_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_conversion() {
        let g = Glyph(0b00000001); // Dot 1
        assert_eq!(g.to_braille(), '⠁');
    }

    #[test]
    fn test_grid_conversion() {
        let grid = [true, false, false, false, false, false, false, false];
        let g = Glyph::from_grid(grid);
        assert_eq!(g.to_braille(), '⠁');
    }

    #[test]
    fn test_glyph_merge() {
        // A: Left Col Full (1, 0, 1, 0...)
        // B: Right Col Full (0, 1, 0, 1...)
        // Merge should be Full Block (⣿)
        // Wait, my logic:
        // NewCol0 = A.Col0 | A.Col1
        // NewCol1 = B.Col0 | B.Col1

        // Let's try:
        // A: ⠁ (Dot 1). Grid: (T, F, F...). Col0 has bit, Col1 empty.
        // B: ⠈ (Dot 4). Grid: (F, T, F...). Col0 empty, Col1 has bit.

        // A merged with B:
        // NewCol0 = A.Col0 | A.Col1 = T | F = T
        // NewCol1 = B.Col0 | B.Col1 = F | T = T
        // Result Grid: (T, T, F...) -> ⠉ (Dots 1 and 4)

        let g_a = Glyph::new(0x01); // Dot 1
        let g_b = Glyph::new(0x08); // Dot 4 (Bit 3)
        let g_merged = g_a.merge(&g_b);

        // Expected: 0x01 | 0x08 = 0x09
        assert_eq!(g_merged.0, 0x09);
    }

    #[test]
    fn test_script_compression() {
        let text = "ababab";
        let mut script = Script::new(text);
        let original_len = script.corpus.len();
        assert_eq!(original_len, 6);

        // 'a' and 'b' should pair.
        // Evolve once.
        let res = script.evolve();
        assert!(res.is_some());

        // Should replace "ab" with "C".
        // Corpus becomes "CCC".
        assert_eq!(script.corpus.len(), 3);
    }
}
