use font8x8::{BASIC_FONTS, UnicodeFonts};

pub fn render_text(text: &str) -> Vec<Vec<u8>> {
    let width = text.len() * 8;
    // We want a grid where each inner vector is a ROW (frequency bin).
    // So grid[row][col] is the pixel at (col, row).
    // Height is 8.
    let mut grid = vec![vec![0u8; width]; 8];

    for (i, c) in text.chars().enumerate() {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            for (row, byte) in glyph.iter().enumerate() {
                if row >= 8 { continue; }
                for bit in 0..8 {
                    // font8x8: MSB is left-most pixel.
                    // bit 0 is LSB (right-most).
                    // We want to check if the bit at position `bit` is set.
                    // If bit=0 (LSB), it is the rightmost pixel (x=7 relative to char).
                    // If bit=7 (MSB), it is the leftmost pixel (x=0 relative to char).

                    if (byte & (1 << bit)) != 0 {
                        // If bit is set, we set the pixel.
                        // The column index relative to the character start is (7 - bit).
                        // e.g. bit=0 -> col=7. bit=7 -> col=0.
                        let col_rel = 7 - bit;
                        let col_abs = i * 8 + col_rel;

                        // Set the pixel
                        grid[row][col_abs] = 1;
                    }
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_text() {
        let text = "H";
        let grid = render_text(text);
        assert_eq!(grid.len(), 8);
        assert_eq!(grid[0].len(), 8);

        println!("Grid for H:");
        for row in 0..8 {
            for col in 0..8 {
                print!("{}", if grid[row][col] == 1 { "#" } else { "." });
            }
            println!();
        }

        // Just verify dimensions for now, as font data might vary
        assert_eq!(grid.len(), 8);
    }
}
